pub mod ast_node;
mod acl_parsers;
mod combinators;
mod config;
mod const_numeric_parsers;
mod consume_macro;
mod error;
mod expr_parsers;
mod func_name_parser;
mod func_type;
mod located_combinator;
mod op_parsers;
mod opt_array_bounds;
mod opt_interval;
mod opt_precision;
mod opt_transaction;
mod opt_transaction_chain;
mod opt_unique_null_treatment;
mod privilege_parsers;
mod result;
mod role_parsers;
mod simple_typename;
mod stmt;
mod token_stream;
mod transaction_mode_list;
mod transaction_stmt_legacy;
mod type_parsers;
mod typename;
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
}

impl<'src> Parser<'src> {

    pub fn new(source: &'src str, config: ParserConfig) -> Self {
        Self {
            buffer: TokenStream::new(source, config)
        }
    }

    /// Not reentrant (yet)!
    /// The TokenStream state is changed.
    pub fn parse(&mut self) -> ParserResult {

        let mut result = stmtmulti()
            .parse(&mut self.buffer)
            .required();

        // If it's not Eof, then something didn't match properly.
        // Discard the previous result, and mark the current location as a Syntax error.
        if !self.buffer.eof() {
            let loc = self.buffer.current_location();
            result = Err(syntax_err(loc));
        }

        ParserResult {
            result,
            warnings: mem::take(self.buffer.warnings()),
        }
    }
}

fn stmtmulti() -> impl Combinator<Output = Vec<RawStmt>> {

    // This production is slightly cheating, not because it's more efficient,
    // but helps simplify capturing the combinator.
    // Production:
    //     (';')* ( toplevel_stmt ( (';')+ toplevel_stmt? )* )?
    // Original production:
    //     toplevel_stmt? ( ';' toplevel_stmt? )*

    semicolons().optional()
        .and_right(
            many_sep(semicolons(), toplevel_stmt().optional())
        )
        .map(|stmts|
            stmts.into_iter()
                .flatten()
                .collect()
        )
}

fn toplevel_stmt() -> impl Combinator<Output = RawStmt> {
    match_first!(
        transaction_stmt_legacy().map(From::from),
        stmt()
    )
}

/// Post-condition: Vec **May** be empty
fn opt_type_modifiers() -> impl Combinator<Output = TypeModifiers> {

    /*
        ( '(' expr_list ')' )?
    */

    expr_list_paren()
        .optional()
        .map(Option::unwrap_or_default)
}

fn opt_varying() -> impl Combinator<Output = bool> {

    /*
        ( VARYING )?
    */

    Varying
        .optional()
        .map(|varying| varying.is_some())
}

fn opt_timezone() -> impl Combinator<Output = bool> {

    /*
        ( (WITH | WITHOUT) TIME ZONE )?
    */

    match_first!(
        With.map(|_| true),
        Without.map(|_| false)
    )
        .and_left(sequence!(Time, Zone).skip())
        .optional()
        .map(|tz| tz.unwrap_or(false))
}

/// Post-condition: Vec is **Not** empty
fn expr_list_paren() -> impl Combinator<Output = Vec<ExprNode>> {

    /*
        '(' expr_list ')'
    */

    between(
        OpenParenthesis,
        expr_list(),
        CloseParenthesis
    )
}

/// Post-condition: Vec is **Not** empty
fn expr_list() -> impl Combinator<Output = Vec<ExprNode>> {

    /*
        a_expr ( ',' a_expr )*
    */

    many_sep(Comma, a_expr())
}

/// Returns `Ok` if it consumed at least 1 `;` (semicolon).
fn semicolons() -> impl Combinator<Output = ()> {

    // Production: ( ';' )+

    many(Semicolon.skip()).skip()
}

/// Alias: `transaction_mode_item`
fn transaction_mode() -> impl Combinator<Output = TransactionMode> {
    use Kw::{Isolation, Level, Not, Only, Read, Write};
    use TransactionMode::*;

    /*
          ISOLATION LEVEL iso_level
        | READ ONLY
        | READ WRITE
        | DEFERRABLE
        | NOT DEFERRABLE
    */

    match_first!{
        Kw::Deferrable.map(|_| Deferrable),
        Not.and_then(Kw::Deferrable, |_, _| NotDeferrable),
        Read.and_right(
            match_first!{
                Only.map(|_| ReadOnly),
                Write.map(|_| ReadWrite)
            }
        ),
        Isolation.and(Level)
            .and_right(isolation_level())
            .map(IsolationLevel)
    }
}

/// Alias: `iso_level`
fn isolation_level() -> impl Combinator<Output = IsolationLevel> {
    use Kw::{Committed, Read, Repeatable, Serializable, Uncommitted};

    /*
          READ UNCOMMITTED
        | READ COMMITTED
        | REPEATABLE READ
        | SERIALIZABLE
    */

    match_first!{
        Serializable.map(|_| IsolationLevel::Serializable),
        Repeatable
            .and_then(Read, |_, _| IsolationLevel::RepeatableRead),
        Read.and_right(
            match_first!{
                Committed.map(|_| IsolationLevel::ReadCommitted),
                Uncommitted.map(|_| IsolationLevel::ReadUncommitted)
            }
        )
    }
}

/// Post-condition: Vec is **Not** empty
///
/// Alias: `opt_column_list`
fn opt_name_list() -> impl Combinator<Output = Vec<Str>> {

    /*
        '(' name_list ')'
    */

    between(OpenParenthesis, name_list(), CloseParenthesis)
}

/// Post-condition: Vec is **Not** empty
fn var_name() -> impl Combinator<Output = QualifiedName> {

    /*
        col_id ( '.' col_id )*
    */

    many_sep(Dot, col_id())
}

/// Post-condition: Vec is **Not** empty
///
/// Alias: `columnList`
fn name_list() -> impl Combinator<Output = Vec<Str>> {

    /*
        col_id ( ',' col_id )*
    */

    many_sep(Comma, col_id())
}

/// Post-condition: Vec is **Not** empty
fn col_id_list(separator: OperatorKind) -> impl Combinator<Output = QualifiedName> {

    /*
        col_id ( <separator> col_id )*
    */

    many_sep(separator, col_id())
}

/// Post-condition: Vec is **Not** empty
fn qualified_name_list() -> impl Combinator<Output = Vec<RangeVar>> {

    /*
        qualified_name ( ',' qualified_name )*
    */

    many_sep(Comma, qualified_name())
}

fn qualified_name() -> impl Combinator<Output = RangeVar> {

    /*
        col_id attrs{1,3}
    */

    located(any_name())
        .map_result(|result| {
            let (mut qn, loc) = result?;

            match qn.as_mut_slice() {
                [relation] => {
                    let relation = mem::take(relation);
                    Ok(RangeVar::new(relation))
                },
                [schema, relation] => {
                    let schema = mem::take(schema);
                    let relation = mem::take(relation);
                    Ok(
                        RangeVar::new(relation)
                            .with_schema(schema)
                    )
                },
                [catalog, schema, relation] => {
                    let catalog = mem::take(catalog);
                    let schema = mem::take(schema);
                    let relation = mem::take(relation);
                    Ok(
                        RangeVar::new(relation)
                            .with_schema(schema)
                            .with_catalog(catalog)
                    )
                },
                _ => {
                    let err = ParserError::new(ImproperQualifiedName(NameList(qn)), loc);
                    Err(err.into())
                }
            }
        })
}

/// Post-condition: Vec is **Not** empty
fn any_name_list() -> impl Combinator<Output=Vec<QualifiedName>> {

    /*
        any_name ( ',' any_name )*
    */

    many_sep(Comma, any_name())
}

/// Post-condition: Vec is **Not** empty
///
/// Alias: `handler_name`
fn any_name() -> impl Combinator<Output = QualifiedName> {

    /*
        col_id attrs
    */

    attrs(col_id())
}

/// Post-condition: Vec is **Not** empty
fn attrs<F>(prefix: F) -> impl Combinator<Output = QualifiedName>
where
    F: Combinator<Output = Str>
{
    /*
        prefix ( '.' col_label )*
    */

    many_pre(
        prefix,
        Dot.and_right(col_label())
    )
}

/// Aliases:
/// * `ColLabel`
/// * `attr_name`
fn col_label() -> impl Combinator<Output = Str> {
    match_first!(
        identifier().map(From::from),
        keyword_if(|_| true).map(From::from)
    )
}

/// Aliases:
/// * `ColId`
/// * `name`
fn col_id() -> impl Combinator<Output = Str> {
    match_first!(
        identifier().map(From::from),
        Unreserved.map(From::from),
        ColumnName.map(From::from),
    )
}

fn type_function_name() -> impl Combinator<Output = Str> {
    match_first!(
        identifier().map(From::from),
        Unreserved.map(From::from),
        TypeFuncName.map(From::from),
    )
}

/// Alias: `NonReservedWord`
fn non_reserved_word() -> impl Combinator<Output = Str> {
    match_first!(
        identifier().map(From::from),
        Unreserved.map(From::from),
        ColumnName.map(From::from),
        TypeFuncName.map(From::from),
    )
}

/// Alias: `BareColLabel`
fn bare_col_label() -> impl Combinator<Output = Str> {
    match_first!(
        identifier().map(From::from),
        keyword_if(|kw| kw.details().bare()).map(From::from)
    )
}

/// Production: `'(' ICONST ')'`
fn i32_literal_paren() -> impl Combinator<Output = i32> {

    between(OpenParenthesis, integer(), CloseParenthesis)
        .map(From::from)
}

fn arg_class() -> impl Combinator<Output = FunctionParameterMode> {
    use FunctionParameterMode::*;

    match_first!(
        Kw::In.and_right(
            Kw::Out.optional()
                .map(|out| if out.is_some() { InOut } else { In })
        ),
        Kw::Out.map(|_| Out),
        Kw::Inout.map(|_| InOut),
        Kw::Variadic.map(|_| Variadic),
    )
}

/// '+' | '-'
fn sign() -> impl Combinator<Output = OperatorKind> {
    use OperatorKind::{Minus, Plus};
    operator_if(|op| matches!(op, Minus | Plus))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::OperatorKind::Dot;
    use crate::parser::ast_node::QualifiedName;
    use postgres_basics::guc::BackslashQuote;
    use test_case::test_case;

    pub(in crate::parser) static DEFAULT_CONFIG: ParserConfig = ParserConfig::new(true, BackslashQuote::SafeEncoding);

    #[test_case("begin transaction")]
    #[test_case("start transaction")]
    #[test_case("end transaction")]
    fn test_toplevel_stmt(source: &str) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = toplevel_stmt().parse(&mut stream);

        // This only quickly tests that statement types aren't missing.
        // More in-depth testing is within each statement's module.
        assert_matches!(actual, Ok(_),
            r"expected Ok(Some(_)) for {source:?} but actually got {actual:?}"
        )
    }

    #[test]
    fn test_transaction_mode() {

        let mut stream = TokenStream::new(
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
            assert_eq!(Ok(expected_mode), transaction_mode().parse(&mut stream));
        }
    }

    #[test]
    fn test_isolation_level() {

        let mut stream = TokenStream::new(
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
            assert_eq!(Ok(expected_mode), isolation_level().parse(&mut stream));
        }
    }

    #[test]
    /// All these methods are similar, so no point in repeating tests:
    /// * test_var_name
    /// * test_name_list
    fn test_col_id_list() {
        let mut stream = TokenStream::new("test.qualified.name", DEFAULT_CONFIG);
        let expected = vec![
            "test".into(),
            "qualified".into(),
            "name".into()
        ];

        assert_eq!(Ok(expected), col_id_list(Dot).parse(&mut stream));
    }

    #[test]
    fn test_expr_list() {
        let mut stream = TokenStream::new("1, 2, 3", DEFAULT_CONFIG);

        let expected = vec![
            ExprNode::IntegerConst(1),
            ExprNode::IntegerConst(2),
            ExprNode::IntegerConst(3),
        ];

        assert_eq!(Ok(expected), expr_list().parse(&mut stream));
    }

    #[test]
    fn test_any_name_list() {
        let source = "qual.name_, second.qualif";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = vec![
            vec!["qual".into(), "name_".into()],
            vec!["second".into(), "qualif".into()]
        ];

        assert_eq!(Ok(expected), any_name_list().parse(&mut stream));
    }

    #[test]
    fn test_any_name() {
        let source = "some_.qualified_.name_";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = any_name().parse(&mut stream);

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
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = RangeVar::new("some_relation".into())
            .with_schema("some_schema".into())
            .with_catalog("some_catalog".into());

        assert_eq!(Ok(expected), qualified_name().parse(&mut stream));
    }

    #[test]
    fn test_qualified_name_list() {
        let source = "relation_,schema_.relation_, catalog_.schema_.relation_";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = vec![
            RangeVar::new("relation_".into()),
            RangeVar::new("relation_".into())
                .with_schema("schema_".into()),
            RangeVar::new("relation_".into())
                .with_schema("schema_".into())
                .with_catalog("catalog_".into())
        ];

        assert_eq!(Ok(expected), qualified_name_list().parse(&mut stream));
    }

    #[test]
    fn test_attrs() {
        let source = ".qualified_.name_";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let parser = combinators::parser(|_| Ok("*some*".into()));
        let actual = attrs(parser).parse(&mut stream);

        let expected: QualifiedName = vec![
            "*some*".into(),
            "qualified_".into(),
            "name_".into()
        ];

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_i32_literal_paren() {
        let mut stream = TokenStream::new(" (123 )", DEFAULT_CONFIG);
        assert_eq!(Ok(123), i32_literal_paren().parse(&mut stream));
    }

    #[test]
    fn test_col_id() {
        let source = "cascaded xxyyzz coalesce";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("cascaded".into()), col_id().parse(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), col_id().parse(&mut stream));
        assert_eq!(Ok("coalesce".into()), col_id().parse(&mut stream));
    }

    #[test]
    fn test_type_function_name() {
        let source = "before xxyyzz collation";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("before".into()), type_function_name().parse(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), type_function_name().parse(&mut stream));
        assert_eq!(Ok("collation".into()), type_function_name().parse(&mut stream));
    }

    #[test]
    fn test_non_reserved_word() {
        let source = "breadth xxyyzz boolean authorization";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("breadth".into()), non_reserved_word().parse(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), non_reserved_word().parse(&mut stream));
        assert_eq!(Ok("boolean".into()), non_reserved_word().parse(&mut stream));
        assert_eq!(Ok("authorization".into()), non_reserved_word().parse(&mut stream));
    }

    #[test]
    fn test_col_label() {
        let source = "sequence xxyyzz character";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("sequence".into()), col_label().parse(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), col_label().parse(&mut stream));
        assert_eq!(Ok("character".into()), col_label().parse(&mut stream));
    }

    #[test]
    fn test_bare_col_label() {
        let source = "sequence xxyyzz";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("sequence".into()), bare_col_label().parse(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), bare_col_label().parse(&mut stream));
    }

    #[test_case("in", FunctionParameterMode::In)]
    #[test_case("in out", FunctionParameterMode::InOut)]
    #[test_case("out", FunctionParameterMode::Out)]
    #[test_case("inout", FunctionParameterMode::InOut)]
    #[test_case("variadic", FunctionParameterMode::Variadic)]
    fn test_arg_class(source: &str, expected: FunctionParameterMode) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(expected), arg_class().parse(&mut stream));
    }
}

use crate::lexer::Keyword as Kw;
use crate::lexer::Keyword::Time;
use crate::lexer::Keyword::Varying;
use crate::lexer::Keyword::With;
use crate::lexer::Keyword::Without;
use crate::lexer::Keyword::Zone;
use crate::lexer::KeywordCategory::ColumnName;
use crate::lexer::KeywordCategory::TypeFuncName;
use crate::lexer::KeywordCategory::Unreserved;
use crate::lexer::OperatorKind;
use crate::lexer::OperatorKind::CloseParenthesis;
use crate::lexer::OperatorKind::Comma;
use crate::lexer::OperatorKind::Dot;
use crate::lexer::OperatorKind::OpenParenthesis;
use crate::lexer::OperatorKind::Semicolon;
use crate::parser::ast_node::EventTriggerState;
use crate::parser::ast_node::ExprNode;
use crate::parser::ast_node::FunctionParameterMode;
use crate::parser::ast_node::IsolationLevel;
use crate::parser::ast_node::QualifiedName;
use crate::parser::ast_node::RangeVar;
use crate::parser::ast_node::RawStmt;
use crate::parser::ast_node::RoleSpec;
use crate::parser::ast_node::TransactionMode;
use crate::parser::ast_node::TypeModifiers;
use crate::parser::combinators::identifier;
use crate::parser::combinators::integer;
use crate::parser::combinators::keyword_if;
use crate::parser::combinators::many;
use crate::parser::combinators::many_pre;
use crate::parser::combinators::many_sep;
use crate::parser::combinators::match_first;
use crate::parser::combinators::operator_if;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::combinators::{between, sequence};
use crate::parser::error::syntax_err;
use crate::parser::error::NameList;
use crate::parser::expr_parsers::a_expr;
use crate::parser::located_combinator::located;
use crate::parser::opt_transaction::opt_transaction;
use crate::parser::opt_transaction_chain::opt_transaction_chain;
use crate::parser::result::Required;
use crate::parser::stmt::stmt;
use crate::parser::token_stream::TokenConsumer;
use crate::parser::token_stream::TokenStream;
use crate::parser::transaction_stmt_legacy::transaction_stmt_legacy;
use crate::parser::ParserErrorKind::ImproperQualifiedName;
use postgres_basics::Located;
use postgres_basics::Str;
use std::mem;
