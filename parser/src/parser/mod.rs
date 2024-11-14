pub mod ast_node;
mod acl_parsers;
mod bit_string_parser;
mod config;
mod const_numeric_parsers;
mod const_string_parsers;
mod consume_macro;
mod error;
mod expr_parsers;
mod func_name_parser;
mod ident_parser;
mod op_parsers;
mod parse_number;
mod privilege_parsers;
mod result;
mod role_parsers;
mod stmt_parser;
mod stmt_parsers;
mod string_parser;
mod token_stream;
mod type_parsers;
mod uescape_escape;
mod warning;

pub use self::{
    config::ParserConfig,
    error::{ParserError, ParserErrorKind},
    warning::ParserWarningKind,
};

pub(crate) type ParseResult<T> = Result<T, ParserError>;

pub struct ParserResult {
    pub result: ParseResult<Vec<RawStmt>>,
    pub warnings: Vec<Located<ParserWarningKind>>,
}

pub struct Parser<'src> {
    buffer: TokenStream<'src>,
    config: ParserConfig,
    /// All the warnings that have been collected while parsing.
    warnings: Vec<Located<ParserWarningKind>>,
}

impl<'src> Parser<'src> {

    pub fn new(source: &'src str, config: ParserConfig) -> Self {
        let lexer = Lexer::new(source, config.standard_conforming_strings());
        Self {
            buffer: TokenStream::new(lexer),
            config,
            warnings: Vec::new(),
        }
    }

    /// Not reentrant!
    pub fn parse(&mut self) -> ParserResult {

        let result = self.stmtmulti();

        ParserResult {
            result,
            warnings: mem::take(&mut self.warnings),
        }
    }

    fn stmtmulti(&mut self) -> ParseResult<Vec<RawStmt>> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::stmtmulti";

        // This production is slightly cheating, not because it's more efficient,
        // but helps simplify capturing errors.
        // Production:
        //     (';')* ( toplevel_stmt ( (';')+ toplevel_stmt? )* )?
        // Original production:
        //     toplevel_stmt? ( ';' toplevel_stmt? )*

        self.semicolons()?;
        if self.buffer.eof() {
            // The whole string is empty, or just contains semicolons, whitespace, or comments.
            return Ok(Vec::new())
        }

        // If the string wasn't considered "empty", then it has at least 1 token, that *must* match some statement.
        let stmt = self.toplevel_stmt()?;
        let mut stmts = vec![stmt];

        while self.semicolons()? && !self.buffer.eof() {
            let stmt = self.toplevel_stmt()?;
            stmts.push(stmt);
        }

        // if it's not Eof, then something didn't match properly
        if !self.buffer.eof() {
            let loc = self.buffer.current_location();
            return Err(syntax_err(fn_info!(FN_NAME), loc))
        }

        Ok(stmts)
    }

    /// Returns `true` if it consumed at least 1 `;` (semicolon)
    fn semicolons(&mut self) -> ParseResult<bool> {

        // Production: (';')*

        if self.buffer.consume_op(Semicolon).optional()?.is_none() {
            return Ok(false)
        }

        while self.buffer.consume_op(Semicolon).optional()?.is_some() {}

        Ok(true)
    }

    fn toplevel_stmt(&mut self) -> ParseResult<RawStmt> {
        self.stmt(true)
    }

    fn opt_transaction(&mut self) -> ParseResult<()> {
        use Keyword::{Transaction, Work};

        // Skips over WORK | TRANSACTION

        self.buffer.consume_kw(|kw| matches!(kw, Work | Transaction))
            .optional()?;

        Ok(())
    }

    fn opt_transaction_chain(&mut self) -> ParseResult<bool> {
        use Keyword::{And, Chain, No};
        const FN_NAME: &str = "postgres_parser::parser::Parser::opt_transaction_chain";

        if self.buffer.consume_kw_eq(And).optional()?.is_none() {
            return Ok(false)
        }

        let result = self.buffer.consume_kw_eq(No).optional()?.is_none();

        self.buffer.consume_kw_eq(Chain).required(fn_info!(FN_NAME))?;

        Ok(result)
    }

    /// Post-condition: Vec is **Not** empty
    ///
    /// Alias: `transaction_mode_list_or_empty`
    fn transaction_mode_list(&mut self) -> ScanResult<Vec<TransactionMode>> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::transaction_mode_list";

        /*
            transaction_mode ( (',')? transaction_mode )*
        */

        let element = self.transaction_mode()?;
        let mut elements = vec![element];

        loop {
            let element = match self.buffer.consume_op(Comma) {
                Ok(_) => {
                    self.transaction_mode().required(fn_info!(FN_NAME))?
                }
                Err(NoMatch(_)) => {
                    let mode = self.transaction_mode().optional();
                    let Some(mode) = mode? else { break };
                    mode
                }
                Err(Eof(_)) => break,
                Err(ScanErr(err)) => return Err(ScanErr(err)),
            };

            elements.push(element);
        }

        while self.buffer.consume_op(Comma).optional()?.is_some() {
            let element = self.transaction_mode().required(fn_info!(FN_NAME))?;
            elements.push(element);
        }

        Ok(elements)
    }

    /// Alias: `transaction_mode_item`
    fn transaction_mode(&mut self) -> ScanResult<TransactionMode> {
        use Keyword::{Deferrable, Isolation, Level, Not, Only, Read, Write};
        const FN_NAME: &str = "postgres_parser::parser::Parser::transaction_mode";

        /*
              ISOLATION LEVEL iso_level
            | READ ONLY
            | READ WRITE
            | DEFERRABLE
            | NOT DEFERRABLE
        */

        let result = self.buffer.consume_kw(|kw|
            matches!(kw, Deferrable | Not | Isolation | Read)
        )?;

        match result {
            Deferrable => Ok(TransactionMode::Deferrable),
            Not => {
                self.buffer.consume_kw_eq(Deferrable).required(fn_info!(FN_NAME))?;
                Ok(TransactionMode::NotDeferrable)
            },
            Isolation => {
                self.buffer.consume_kw_eq(Level).required(fn_info!(FN_NAME))?;
                let isolation_level = self.isolation_level().required(fn_info!(FN_NAME))?;
                Ok(TransactionMode::IsolationLevel(isolation_level))
            },
            Read => {
                self.buffer
                    .consume(|tok| match tok.keyword() {
                        Some(Only) => Some(TransactionMode::ReadOnly),
                        Some(Write) => Some(TransactionMode::ReadWrite),
                        _ => None
                    })
                    .required(fn_info!(FN_NAME))
                    .map_err(ScanErrorKind::from)
            },
            _ => unreachable!("it was already filtered by consume_kw()")
        }
    }

    /// Alias: `iso_level`
    fn isolation_level(&mut self) -> ScanResult<IsolationLevel> {
        use Keyword::{Committed, Read, Repeatable, Serializable, Uncommitted};
        const FN_NAME: &str = "postgres_parser::parser::Parser::isolation_level";

        /*
            READ UNCOMMITTED
            READ COMMITTED
            REPEATABLE READ
            SERIALIZABLE
        */

        let kw = self.buffer.consume_kw(|kw|
            matches!(kw, Read | Repeatable | Serializable)
        )?;

        match kw {
            Serializable => Ok(IsolationLevel::Serializable),
            Repeatable => {
                self.buffer.consume_kw_eq(Read).required(fn_info!(FN_NAME))?;
                Ok(IsolationLevel::RepeatableRead)
            },
            Read => {
                let level = self.buffer
                    .consume(|tok| match tok.keyword() {
                        Some(Committed) => Some(IsolationLevel::ReadCommitted),
                        Some(Uncommitted) => Some(IsolationLevel::ReadUncommitted),
                        _ => None
                    })
                    .required(fn_info!(FN_NAME))?;

                Ok(level)
            },
            _ => panic!("expected keywords Read, Repeatable, or Serializable, but got {kw:?}"),
        }
    }

    /// Post-condition: Vec is **Not** empty
    fn var_list(&mut self) -> ScanResult<Vec<QualifiedName>> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::var_list";

        /*
            var_name ( ',' var_name )*
        */

        let element = self.var_name()?;
        let mut elements = vec![element];

        while self.buffer.consume_op(Comma).optional()?.is_some() {
            let element = self.var_name().required(fn_info!(FN_NAME))?;
            elements.push(element);
        }

        Ok(elements)
    }

    /// Post-condition: Vec is **Not** empty
    fn var_name(&mut self) -> ScanResult<QualifiedName> {

        /*
            col_id ( '.' col_id )*
        */

        self.col_id_list(Dot)
    }

    /// Post-condition: Vec is **Not** empty
    ///
    /// Alias: `columnList`
    fn name_list(&mut self) -> ScanResult<Vec<Str>> {

        /*
            col_id ( ',' col_id )*
        */

        self.col_id_list(Comma)
    }

    /// Post-condition: Vec is **Not** empty
    ///
    /// Alias: `opt_column_list`
    fn opt_name_list(&mut self) -> ScanResult<Vec<Str>> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::opt_col_list";

        /*
            '(' name_list ')'
        */

        self.buffer.consume_op(OpenParenthesis)?;
        let names = self.name_list().required(fn_info!(FN_NAME))?;
        self.buffer.consume_op(CloseParenthesis).required(fn_info!(FN_NAME))?;

        Ok(names)
    }

    /// Post-condition: Vec is **Not** empty
    fn col_id_list(&mut self, separator: OperatorKind) -> ScanResult<QualifiedName> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::col_id_list";

        /*
            col_id ( <separator> col_id )*
        */

        let element = self.col_id()?;
        let mut elements = vec![element];

        while self.buffer.consume_op(separator).optional()?.is_some() {
            let element = self.col_id().required(fn_info!(FN_NAME))?;
            elements.push(element);
        }

        Ok(elements)
    }

    /// Post-condition: Vec is **Not** empty
    fn expr_list_paren(&mut self) -> ScanResult<Vec<ExprNode>> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::opt_type_modifiers";

        /*
            '(' expr_list ')'
        */

        self.buffer.consume_op(OpenParenthesis)?;
        let exprs = self.expr_list().required(fn_info!(FN_NAME))?;
        self.buffer.consume_op(CloseParenthesis).required(fn_info!(FN_NAME))?;

        Ok(exprs)
    }

    /// Post-condition: Vec is **Not** empty
    fn expr_list(&mut self) -> ScanResult<Vec<ExprNode>> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::expr_list";

        /*
            a_expr ( ',' a_expr )*
        */

        let expr = self.a_expr()?;
        let mut exprs = vec![expr];

        while self.buffer.consume_op(Comma).optional()?.is_some() {
            let expr = self.a_expr().required(fn_info!(FN_NAME))?;
            exprs.push(expr);
        }

        Ok(exprs)
    }

    /// Post-condition: Vec is **Not** empty
    fn qualified_name_list(&mut self) -> ScanResult<Vec<RangeVar>> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::qualified_name_list";

        let mut elements = vec![self.qualified_name()?];

        while self.buffer.consume_op(Comma).optional()?.is_some() {
            let element = self.qualified_name().required(fn_info!(FN_NAME))?;
            elements.push(element);
        }

        Ok(elements)
    }

    fn qualified_name(&mut self) -> ScanResult<RangeVar> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::qualified_name";

        /*
            col_id attrs{0,2}
        */

        let loc = self.buffer.current_location();
        let qn = self.any_name()?;

        if !(1..=3).contains(&qn.len()) {
            let err = ParserError::new(ImproperQualifiedName(NameList(qn)), fn_info!(FN_NAME), loc);
            return Err(err.into())
        }

        let mut it = qn.into_iter();

        let range_var = match (it.next(), it.next(), it.next()) {
            (Some(relation), None, None) => RangeVar::new(relation),
            (Some(schema), Some(relation), None) => {
                RangeVar::new(relation)
                    .with_schema(schema)
            },
            (Some(catalog), Some(schema), Some(relation)) => {
                RangeVar::new(relation)
                    .with_schema(schema)
                    .with_catalog(catalog)
            },
            _ => unreachable!("length was already checked to be between 1..=3")
        };

        Ok(range_var)
    }

    /// Post-condition: Vec is **Not** empty
    fn any_name_list(&mut self) -> ScanResult<Vec<QualifiedName>> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::any_name_list";

        /*
            any_name ( ',' any_name )*
        */

        let element = self.any_name()?;
        let mut elements = vec![element];

        while self.buffer.consume_op(Comma).optional()?.is_some() {
            let element = self.any_name().required(fn_info!(FN_NAME))?;
            elements.push(element);
        }

        Ok(elements)
    }

    /// Post-condition: Vec is **Not** empty
    ///
    /// Alias: `handler_name`
    fn any_name(&mut self) -> ScanResult<QualifiedName> {

        /*
            col_id attrs
        */

        let prefix = self.col_id()?;
        self.attrs(prefix).map_err(From::from)
    }

    /// Post-condition: Vec is **Not** empty
    fn attrs(&mut self, prefix: Str) -> ParseResult<QualifiedName> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::attrs";

        // A prefix token is passed to prevent a right shift of the Vec later on,
        // to insert the 1st element.

        /*
            prefix ( '.' col_label )*
        */

        let mut attrs = vec![prefix];

        while self.buffer.consume_op(Dot).optional()?.is_some() {
            let attr = self.col_label().required(fn_info!(FN_NAME))?;
            attrs.push(attr);
        }

        Ok(attrs)
    }

    /// Production: `'(' ICONST ')'`
    fn i32_literal_paren(&mut self) -> ScanResult<i32> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::i32_literal_paren";

        self.buffer.consume_op(OpenParenthesis)?;
        let num = self.i32_literal().required(fn_info!(FN_NAME))?;
        self.buffer.consume_op(CloseParenthesis).required(fn_info!(FN_NAME))?;

        Ok(num)
    }

    fn opt_unique_null_treatment(&mut self) -> ScanResult<bool> {
        use Keyword::{Distinct, Not, Nulls};
        const FN_NAME: &str = "postgres_parser::parser::Parser::opt_unique_null_treatment";

        if self.buffer.consume_kw_eq(Nulls).optional()?.is_none() {
            return Ok(true)
        }

        let result = self.buffer.consume_kw_eq(Not)
            .try_match(fn_info!(FN_NAME))?
            .is_none();

        self.buffer.consume_kw_eq(Distinct).required(fn_info!(FN_NAME))?;

        Ok(result)
    }

    /// Aliases:
    /// * `ColId`
    /// * `name`
    #[inline(always)]
    fn col_id(&mut self) -> ScanResult<Str> {
        self.ident_or_keyword(|kw|
            matches!(kw.details().category(), Unreserved | ColumnName)
        )
    }

    #[inline(always)]
    fn type_function_name(&mut self) -> ScanResult<Str> {
        self.ident_or_keyword(|kw|
            matches!(kw.details().category(), Unreserved | TypeFuncName)
        )
    }

    /// Alias: `NonReservedWord`
    #[inline(always)]
    fn non_reserved_word(&mut self) -> ScanResult<Str> {
        self.ident_or_keyword(|kw|
            matches!(kw.details().category(), Unreserved | ColumnName | TypeFuncName)
        )
    }

    /// Aliases:
    /// * `ColLabel`
    /// * `attr_name`
    #[inline(always)]
    fn col_label(&mut self) -> ScanResult<Str> {
        self.ident_or_keyword(|_| true)
    }

    /// Alias: `BareColLabel`
    #[inline(always)]
    fn bare_col_label(&mut self) -> ScanResult<Str> {
        self.ident_or_keyword(|kw| kw.details().bare())
    }

    fn ident_or_keyword<P>(&mut self, pred: P) -> ScanResult<Str>
    where
        P: Fn(Keyword) -> bool
    {
        if let Some(ident) = self.identifier().no_match_to_option()? {
            return Ok(ident.into())
        }

        self.buffer.consume(|tok|
            tok.keyword()
                .filter(|kw| pred(*kw))
                .map(|kw| kw.details().text().into())
        )
    }

    /// Alias: `IDENT`
    #[inline(always)]
    fn identifier(&mut self) -> ScanResult<Box<str>> {
        IdentifierParser(self).parse()
    }

    /// '+' | '-'
    fn sign(&mut self) ->  ScanResult<OperatorKind> {
        use OperatorKind::{Minus, Plus};
        self.buffer.consume(|tok|
            tok.operator()
                .filter(|op| matches!(op, Minus | Plus))
        )
    }

    /// Production: `UESCAPE SCONST`
    fn uescape(&mut self) -> ParseResult<char> {
        use Keyword::Uescape;
        const FN_NAME: &str = "postgres_parser::parser::Parser::uescape";

        // Try to consume UESCAPE + the string following it.
        // see [base_yylex](https://github.com/postgres/postgres/blob/1c61fd8b527954f0ec522e5e60a11ce82628b681/src/backend/parser/parser.c#L256)

        if self.buffer.consume_kw_eq(Uescape).optional()?.is_none() {
            return Ok('\\')
        };

        let uescape = self.buffer.consume_with_slice(|(tok, slice, loc)| {

            let RawTokenKind::StringLiteral(_) = tok else {
                return Err(
                    ParserError::new(UescapeDelimiterMissing, fn_info!(FN_NAME), loc)
                )
            };

            match uescape_escape(slice) {
                Some(escape) => Ok(Some(escape)),
                None => Err(
                    ParserError::new(InvalidUescapeDelimiter, fn_info!(FN_NAME), loc)
                ),
            }
        });

        uescape.map_err(|err| match err {
            Eof(loc) => ParserError::new(InvalidUescapeDelimiter, fn_info!(FN_NAME), loc),
            NoMatch(loc) => syntax_err(fn_info!(FN_NAME), loc),
            ScanErr(err) => err
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::OperatorKind::Dot;
    use crate::parser::ast_node::QualifiedName;
    use postgres_basics::guc::BackslashQuote;
    use test_case::test_case;

    pub(in crate::parser) const DEFAULT_CONFIG: ParserConfig = ParserConfig::new(true, BackslashQuote::SafeEncoding);

    #[test_case("begin transaction")]
    #[test_case("start transaction")]
    #[test_case("end transaction")]
    fn test_toplevel_stmt(source: &str) {
        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        let actual = parser.toplevel_stmt();

        // This only quickly tests that statement types aren't missing.
        // More in-depth testing is within each statement's module.
        assert_matches!(actual, Ok(_),
            r"expected Ok(Some(_)) for {source:?} but actually got {actual:?}"
        )
    }

    #[test]
    fn test_opt_transaction() {
        let mut parser = Parser::new("transaction work", DEFAULT_CONFIG);
        assert_eq!(Ok(()), parser.opt_transaction());
        assert_eq!(Ok(()), parser.opt_transaction());
    }

    #[test]
    fn test_opt_transaction_chain() {
        let mut parser = Parser::new("", DEFAULT_CONFIG);
        assert_eq!(Ok(false), parser.opt_transaction_chain());

        let mut parser = Parser::new("and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(false), parser.opt_transaction_chain());

        let mut parser = Parser::new("and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(true), parser.opt_transaction_chain());
    }

    #[test]
    fn test_opt_transaction_mode_list() {
        let mut parser = Parser::new("no_match", DEFAULT_CONFIG);
        assert_matches!(parser.transaction_mode_list(), Err(NoMatch(_)));

        let mut parser = Parser::new(
            "read only , read write isolation level read committed",
            DEFAULT_CONFIG
        );

        let expected = vec![
            TransactionMode::ReadOnly,
            TransactionMode::ReadWrite,
            TransactionMode::IsolationLevel(IsolationLevel::ReadCommitted),
        ];

        assert_eq!(Ok(expected), parser.transaction_mode_list());
    }

    #[test]
    fn test_transaction_mode() {

        let mut parser = Parser::new(
            "\
                read only \
                read write \
                deferrable \
                not deferrable \
                isolation level read committed \
                isolation level read uncommitted \
                isolation level repeatable read \
                isolation level serializable\
            ",
            DEFAULT_CONFIG
        );

        let expected = [
            TransactionMode::ReadOnly,
            TransactionMode::ReadWrite,
            TransactionMode::Deferrable,
            TransactionMode::NotDeferrable,
            TransactionMode::IsolationLevel(IsolationLevel::ReadCommitted),
            TransactionMode::IsolationLevel(IsolationLevel::ReadUncommitted),
            TransactionMode::IsolationLevel(IsolationLevel::RepeatableRead),
            TransactionMode::IsolationLevel(IsolationLevel::Serializable),
        ];

        for expected_mode in expected {
            assert_eq!(Ok(expected_mode), parser.transaction_mode());
        }
    }

    #[test]
    fn test_isolation_level() {

        let mut parser = Parser::new(
            "\
                read committed \
                read uncommitted \
                repeatable read \
                serializable\
            ",
            DEFAULT_CONFIG
        );

        let expected = [
            IsolationLevel::ReadCommitted,
            IsolationLevel::ReadUncommitted,
            IsolationLevel::RepeatableRead,
            IsolationLevel::Serializable,
        ];

        for expected_mode in expected {
            assert_eq!(Ok(expected_mode), parser.isolation_level());
        }
    }

    #[test]
    fn test_var_list() {
        let mut parser = Parser::new("qual.name , second.qualified", DEFAULT_CONFIG);
        let expected = vec![
            vec!["qual".into(), "name".into()],
            vec!["second".into(), "qualified".into()]
        ];

        assert_eq!(Ok(expected), parser.var_list());
    }

    #[test]
    /// All these methods are similar, so no point in repeating tests:
    /// * test_var_name
    /// * test_name_list
    fn test_col_id_list() {
        let mut parser = Parser::new("test.qualified.name", DEFAULT_CONFIG);
        let expected = vec![
            "test".into(),
            "qualified".into(),
            "name".into()
        ];

        assert_eq!(Ok(expected), parser.col_id_list(Dot));
    }

    #[test] #[ignore]
    fn test_expr_list() {
        todo!()
    }

    #[test] #[ignore]
    fn test_a_expr() {
        todo!()
    }

    #[test]
    fn test_any_name_list() {
        let source = "qual.name_, second.qualif";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = vec![
            vec!["qual".into(), "name_".into()],
            vec!["second".into(), "qualif".into()]
        ];

        assert_eq!(Ok(expected), parser.any_name_list());
    }

    #[test]
    fn test_any_name() {
        let source = "some_.qualified_.name_";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        let actual = parser.any_name();

        let expected: QualifiedName = vec![
            "some_".into(),
            "qualified_".into(),
            "name_".into()
        ];

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_qualified_name() {
        let source = "some_catalog.some_schema.some_relation";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = RangeVar::new("some_relation".into())
            .with_schema("some_schema".into())
            .with_catalog("some_catalog".into());

        assert_eq!(Ok(expected), parser.qualified_name());
    }

    #[test]
    fn test_qualified_name_list() {
        let source = "relation_,schema_.relation_, catalog_.schema_.relation_";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = vec![
            RangeVar::new("relation_".into()),
            RangeVar::new("relation_".into())
                .with_schema("schema_".into()),
            RangeVar::new("relation_".into())
                .with_schema("schema_".into())
                .with_catalog("catalog_".into())
        ];

        assert_eq!(Ok(expected), parser.qualified_name_list());
    }

    #[test]
    fn test_attrs() {
        let source = ".qualified_.name_";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        let actual = parser.attrs("*some*".into());

        let expected: QualifiedName = vec![
            "*some*".into(),
            "qualified_".into(),
            "name_".into()
        ];

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_i32_literal_paren() {
        let mut parser = Parser::new(" (123 )", DEFAULT_CONFIG);
        assert_eq!(Ok(123), parser.i32_literal_paren());
    }

    #[test]
    fn test_col_id() {
        let source = "cascaded xxyyzz coalesce";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("cascaded".into()), parser.col_id());
        assert_eq!(Ok("xxyyzz".into()), parser.col_id());
        assert_eq!(Ok("coalesce".into()), parser.col_id());
    }

    #[test]
    fn test_type_function_name() {
        let source = "before xxyyzz collation";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("before".into()), parser.type_function_name());
        assert_eq!(Ok("xxyyzz".into()), parser.type_function_name());
        assert_eq!(Ok("collation".into()), parser.type_function_name());
    }

    #[test]
    fn test_non_reserved_word() {
        let source = "breadth xxyyzz boolean authorization";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("breadth".into()), parser.non_reserved_word());
        assert_eq!(Ok("xxyyzz".into()), parser.non_reserved_word());
        assert_eq!(Ok("boolean".into()), parser.non_reserved_word());
        assert_eq!(Ok("authorization".into()), parser.non_reserved_word());
    }

    #[test]
    fn test_col_label() {
        let source = "sequence xxyyzz character";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("sequence".into()), parser.col_label());
        assert_eq!(Ok("xxyyzz".into()), parser.col_label());
        assert_eq!(Ok("character".into()), parser.col_label());
    }

    #[test]
    fn test_bare_col_label() {
        let source = "sequence xxyyzz";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("sequence".into()), parser.bare_col_label());
        assert_eq!(Ok("xxyyzz".into()), parser.bare_col_label());
    }

    #[test]
    fn test_identifier() {
        let source = "tEsT_iDeNtIfIeR";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!("test_identifier", parser.identifier().unwrap().as_ref());
    }

    #[test]
    fn test_uescape() {
        let source = "UESCAPE '!'";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.uescape().unwrap();

        assert_eq!('!', actual);
    }

    #[test_case("", true)]
    #[test_case("nulls distinct", true)]
    #[test_case("nulls not distinct", false)]
    fn test_opt_unique_null_treatment(source: &str, expected: bool) {
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(expected), parser.opt_unique_null_treatment());
    }
}

use self::{
    ast_node::{
        EventTriggerState,
        ExprNode,
        IsolationLevel,
        QualifiedName,
        RangeVar,
        RawStmt,
        RoleSpec,
        TransactionMode,
    },
    consume_macro::consume,
    error::{syntax_err, NameList, ParserErrorKind::*},
    ident_parser::IdentifierParser,
    result::{
        Optional,
        Required,
        ScanErrorKind::{self, Eof, NoMatch, ScanErr},
        ScanResult,
        ScanResultTrait,
        TryMatch,
    },
    token_stream::{SlicedTokenConsumer, TokenConsumer, TokenStream},
    uescape_escape::uescape_escape
};
use crate::lexer::{
    Keyword,
    KeywordCategory::*,
    Lexer,
    OperatorKind::{self, CloseParenthesis, Comma, Dot, OpenParenthesis, Semicolon},
    RawTokenKind
};
use postgres_basics::{fn_info, Located, Str};
use std::{borrow::Cow, mem};
