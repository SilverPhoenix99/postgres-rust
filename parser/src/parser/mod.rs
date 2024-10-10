pub mod ast_node;
mod bit_string_parser;
mod config;
mod error;
mod expr_parsers;
mod ident_parser;
mod result;
mod stmt_parsers;
mod string_parser;
mod token_buffer;
mod warning;

pub use self::{
    config::ParserConfig,
    error::ParserErrorKind,
    warning::ParserWarning,
};

type CowStr = Cow<'static, str>;
type QnName = Vec<CowStr>;

pub struct ParserResult {
    pub result: Result<Vec<AstNode>, Located<ParserErrorKind>>,
    pub warnings: Vec<Located<ParserWarning>>,
}

pub struct Parser<'src> {
    buffer: TokenBuffer<'src>,
    config: ParserConfig,
    /// All the warnings that have been collected while parsing.
    warnings: Vec<Located<ParserWarning>>,
    /// Overrides the default error location.
    /// Useful when lookahead is needed,
    /// or when the error happens somewhere inside the token.
    err_loc_override: Option<Location>,
}

impl<'src> Parser<'src> {

    pub fn new(source: &'src str, config: ParserConfig) -> Self {
        let lexer = Lexer::new(source, config.standard_conforming_strings());
        Self::with_lexer(lexer, config)
    }

    #[inline(always)]
    pub fn with_lexer(lexer: Lexer<'src>, config: ParserConfig) -> Self {
        Self {
            buffer: TokenBuffer::new(lexer),
            config,
            warnings: Vec::new(),
            err_loc_override: None
        }
    }

    /// Not reentrant!
    pub fn parse(&mut self) -> ParserResult {

        let result = match self.stmtmulti() {
            Ok(stmts) => Ok(stmts),
            Err(err) => {
                let loc = self.err_loc_override.take()
                    .unwrap_or_else(|| self.buffer.current_location());
                Err((err, loc))
            }
        };

        ParserResult {
            result,
            warnings: mem::take(&mut self.warnings),
        }
    }

    fn stmtmulti(&mut self) -> Result<Vec<AstNode>, ParserErrorKind> {

        // This production is slightly cheating, not because it's more efficient,
        // but helps simplify capturing errors a bit.
        // Production:
        //     ( stmt? ((';')+ stmt?)* )?
        // Original production:
        //     ( stmt? (';' stmt?)* )?

        let mut stmts = match self.toplevel_stmt() {
            Ok(stmt) => vec![stmt],
            Err(Eof) => {
                // The whole string is empty, or just contains whitespace and/or comments.
                return Ok(Vec::new());
            },
            Err(NoMatch) => {
                // Something went wrong with the 1st token already?!
                return Err(ParserErrorKind::default())
            }
            Err(ParserErr(err)) => return Err(err),
        };

        while self.semicolons()? {

            let stmt = match self.toplevel_stmt() {
                Ok(stmt) => stmt,
                Err(Eof) => break,
                Err(NoMatch) => {
                    // No stmt matched
                    return Err(ParserErrorKind::default())
                }
                Err(ParserErr(err)) => return Err(err),
            };

            stmts.push(stmt);
        }

        // if it's not Eof, then something didn't match properly
        if self.buffer.peek().is_some() {
            return Err(ParserErrorKind::default())
        }

        Ok(stmts)
    }

    /// Returns `true` if it consumed at least 1 `;` (semicolon)
    fn semicolons(&mut self) -> Result<bool, ParserErrorKind> {
        use TokenKind::Semicolon;

        // Production: (';')*

        if self.buffer.consume_eq(Semicolon).optional()?.is_none() {
            return Ok(false)
        }

        while self.buffer.consume_eq(Semicolon).optional()?.is_some() {}

        Ok(true)
    }

    fn toplevel_stmt(&mut self) -> ScanResult<AstNode> {

        if self.buffer.eof() { Err(Eof) }
        else if let Some(node) = self.stmt().optional()? { Ok(node) }
        else if let Some(node) = self.begin_stmt().optional()? { Ok(node.into()) }
        else if let Some(node) = self.end_stmt().optional()? { Ok(node.into()) }
        else { Err(NoMatch) }
    }

    fn stmt(&mut self) -> ScanResult<AstNode> {

        if self.buffer.eof() { Err(Eof) }
        else if self.buffer.consume_kw_eq(Keyword::Checkpoint).optional()?.is_some() { Ok(AstNode::CheckPoint) }
        else if let Some(node) = self.abort_stmt().optional()? { Ok(node.into()) }
        else if let Some(node) = self.alter_stmt().optional()? { Ok(node) }
        else if let Some(node) = self.analyze_stmt().optional()? { Ok(node) }
        else if let Some(node) = self.call_stmt().optional()? { Ok(node) }
        else if let Some(node) = self.close_stmt().optional()? { Ok(ClosePortalStmt(node)) }
        else if let Some(node) = self.cluster_stmt().optional()? { Ok(node) }
        else if let Some(node) = self.comment_stmt().optional()? { Ok(node) }
        else if let Some(node) = self.commit_stmt().optional()? { Ok(node.into()) }
        else if let Some(node) = self.copy_stmt().optional()? { Ok(node) }
        else if let Some(node) = self.deallocate_stmt().optional()? { Ok(ClosePortalStmt(node)) }
        else if let Some(node) = self.discard_stmt().optional()? { Ok(node.into()) }
        else if let Some(node) = self.do_stmt().optional()? { Ok(node) }
        else if let Some(node) = self.drop_stmt().optional()? { Ok(node) }
        else if let Some(node) = self.explain_stmt().optional()? { Ok(node) }
        else if let Some(node) = self.fetch_stmt().optional()? { Ok(node) }
        else if let Some(node) = self.import_stmt().optional()? { Ok(node) }
        else if let Some(node) = self.listen_stmt().optional()? { Ok(ListenStmt(node)) }
        else if let Some(node) = self.load_stmt().optional()? { Ok(LoadStmt(node)) }
        else if let Some(node) = self.lock_stmt().optional()? { Ok(node) }
        else if let Some(node) = self.move_stmt().optional()? { Ok(node) }
        else if let Some(node) = self.notify_stmt().optional()? { Ok(node.into()) }
        else if let Some(node) = self.prepare_stmt().optional()? { Ok(node) }
        else if let Some(node) = self.reassign_owned_stmt().optional()? { Ok(node.into()) }
        else if let Some(node) = self.reindex_stmt().optional()? { Ok(node) }
        else if let Some(node) = self.release_savepoint_stmt().optional()? { Ok(node.into()) }
        else if let Some(node) = self.revoke_stmt().optional()? { Ok(node) }
        else if let Some(node) = self.rollback_stmt().optional()? { Ok(node.into()) }
        else if let Some(node) = self.savepoint_stmt().optional()? { Ok(node.into()) }
        else if let Some(node) = self.security_stmt().optional()? { Ok(node) }
        else if let Some(node) = self.set_stmt().optional()? { Ok(node) }
        else if let Some(node) = self.show_stmt().optional()? { Ok(node.into()) }
        else if let Some(node) = self.start_transaction_stmt().optional()? { Ok(node.into()) }
        else if let Some(node) = self.truncate_stmt().optional()? { Ok(node) }
        else if let Some(node) = self.unlisten_stmt().optional()? { Ok(ClosePortalStmt(node)) }
        else if let Some(node) = self.vacuum_stmt().optional()? { Ok(node) }
        else { Err(NoMatch) }
    }

    fn opt_transaction(&mut self) -> Result<(), ParserErrorKind> {
        use Keyword::{Transaction, Work};

        // Skips over WORK | TRANSACTION

        self.buffer
            .consume(|tok|
                tok.keyword().map(KeywordDetails::keyword)
                    .filter(|kw| matches!(kw, Work | Transaction))
            )
            .optional()?;

        Ok(())
    }

    fn opt_transaction_chain(&mut self) -> Result<bool, ParserErrorKind> {
        use Keyword::{And, Chain, No};

        if self.buffer.consume_kw_eq(And).optional()?.is_none() {
            return Ok(false)
        }

        let result = self.buffer.consume_kw_eq(No).optional()?.is_none();

        self.buffer.consume_kw_eq(Chain).required()?;

        Ok(result)
    }

    /// Post-condition: Vec is **Not** empty
    ///
    /// Alias: `transaction_mode_list_or_empty`
    fn transaction_mode_list(&mut self) -> ScanResult<Vec<TransactionMode>> {

        /*
            transaction_mode ( (',')? transaction_mode )*
        */

        let element = self.transaction_mode()?;
        let mut elements = vec![element];

        loop {
            let element = match self.buffer.consume_eq(Comma) {
                Ok(_) => {
                    self.transaction_mode().required()?
                }
                Err(NoMatch) => {
                    let mode = self.transaction_mode().optional();
                    let Some(mode) = mode? else { break };
                    mode
                }
                Err(Eof) => break,
                Err(ParserErr(err)) => return Err(ParserErr(err)),
            };

            elements.push(element);
        }

        while self.buffer.consume_eq(Comma).optional()?.is_some() {
            let element = self.transaction_mode().required()?;
            elements.push(element);
        }

        Ok(elements)
    }

    /// Alias: `transaction_mode_item`
    fn transaction_mode(&mut self) -> ScanResult<TransactionMode> {
        use Keyword::{Deferrable, Isolation, Level, Not, Only, Read, Write};

        /*
              ISOLATION LEVEL iso_level
            | READ ONLY
            | READ WRITE
            | DEFERRABLE
            | NOT DEFERRABLE
        */

        let result = self.buffer.consume(|tok|
            tok.keyword().map(KeywordDetails::keyword)
                .filter(|kw| matches!(kw, Deferrable | Not | Isolation | Read))
        )?;

        match result {
            Deferrable => Ok(TransactionMode::Deferrable),
            Not => {
                self.buffer.consume_kw_eq(Deferrable).required()?;
                Ok(TransactionMode::NotDeferrable)
            },
            Isolation => {
                self.buffer.consume_kw_eq(Level).required()?;
                let isolation_level = self.isolation_level().required()?;
                Ok(TransactionMode::IsolationLevel(isolation_level))
            },
            Read => {
                self.buffer
                    .consume(|tok| match tok.keyword().map(KeywordDetails::keyword) {
                        Some(Only) => Some(TransactionMode::ReadOnly),
                        Some(Write) => Some(TransactionMode::ReadWrite),
                        _ => None
                    })
                    .required()
                    .map_err(ScanErrorKind::from)
            },
            _ => Err(NoMatch)
        }
    }

    /// Alias: `iso_level`
    fn isolation_level(&mut self) -> ScanResult<IsolationLevel> {
        use Keyword::{Committed, Read, Repeatable, Serializable, Uncommitted};

        /*
            READ UNCOMMITTED
            READ COMMITTED
            REPEATABLE READ
            SERIALIZABLE
        */

        let kw = self.buffer.consume(|tok|
            tok.keyword().map(KeywordDetails::keyword)
                .filter(|kw| matches!(kw, Read | Repeatable | Serializable))
        )?;

        match kw {
            Serializable => Ok(IsolationLevel::Serializable),
            Repeatable => {
                self.buffer.consume_kw_eq(Read).required()?;
                Ok(IsolationLevel::RepeatableRead)
            },
            Read => {
                let level = self.buffer
                    .consume(|tok|
                        tok.keyword().and_then(|kw| match kw.keyword() {
                            Committed => Some(IsolationLevel::ReadCommitted),
                            Uncommitted => Some(IsolationLevel::ReadUncommitted),
                            _ => None
                        })
                    )
                    .required()?;

                Ok(level)
            },
            _ => panic!("expected keywords Read, Repeatable, or Serializable, but got {kw:?}"),
        }
    }

    /// Post-condition: Vec is **not** empty
    fn var_name(&mut self) -> ScanResult<QnName> {

        /*
            col_id ( '.' col_id )*
        */

        let element = self.col_id()?;
        let mut elements = vec![element];

        while self.buffer.consume_eq(Dot).optional()?.is_some() {
            let element = self.col_id().required()?;
            elements.push(element);
        }

        Ok(elements)
    }

    /// Alias: `AexprConst`
    fn a_expr_const(&mut self) -> ScanResult<()> {

        /*
        AexprConst :
            ICONST
          | FCONST
          | SCONST
          | BCONST
          | XCONST
          | func_name SCONST
          | func_name '(' func_arg_list opt_sort_clause ')' SCONST
          | ConstTypename SCONST
          | INTERVAL SCONST opt_interval
          | INTERVAL '(' ICONST ')' SCONST
          | TRUE_P
          | FALSE_P
          | NULL_P
        */

        todo!()
    }

    /// Alias: `ConstTypename`
    fn const_typename(&mut self) -> ScanResult<SystemType> {
        use Keyword::{Bit, Varying};

        /*
        ConstTypename :
            numeric // Numeric
          | character ( '(' ICONST ')' )? // ConstCharacter
          | BIT (VARYING)? ( '(' expr_list ')' )? // ConstBit
          | TIMESTAMP ( '(' ICONST ')' )? ( (WITH_LA | WITHOUT_LA) TIME ZONE )? // ConstDatetime
          | TIME ( '(' ICONST ')' )? ( (WITH_LA | WITHOUT_LA) TIME ZONE )?      // ConstDatetime
          | JSON
        */

        if let Some(num) = self.numeric().no_match_to_option()? {
            return Ok(num)
        }

        if let Some(character) = self.character().optional()? {

            let len = self.i32_literal_paren().optional()?;
            let character = match character {
                CharacterSystemType::Varchar => { SystemType::Varchar(len) },
                CharacterSystemType::Bpchar => SystemType::Bpchar(len),
            };

            return Ok(character)
        }

        let bit = self.buffer.consume_kw_eq(Bit).optional()?;
        if bit.is_some() {
            let varying = self.buffer.consume_kw_eq(Varying).optional()?;
            // TODO: self.expr_list()
        }

        todo!()
    }

    /// Alias: `Numeric`<p/>
    /// Inline: `opt_float`
    fn numeric(&mut self) -> ScanResult<SystemType> {
        use Keyword::{Bigint, Boolean, Dec, Decimal, Double, Float, Int, Integer, Numeric, Precision, Real, Smallint};

        /*
        Numeric :
            BOOLEAN
          | INT_P
          | INTEGER
          | SMALLINT
          | BIGINT
          | REAL
          | FLOAT ( '(' ICONST ')' )?
          | DOUBLE PRECISION
          | (DECIMAL | DEC | NUMERIC) opt_type_modifiers
        */

        let kw = self.buffer.consume(|tok|
            tok.keyword().map(KeywordDetails::keyword).filter(|kw|
                matches!(kw,
                      Smallint
                    | Int
                    | Integer
                    | Bigint
                    | Real
                    | Boolean
                    | Double
                    | Float
                    | Decimal
                    | Dec
                    | Numeric
                )
            )
        )?;

        match kw {
            Smallint => return Ok(Int2),
            Int | Integer => return Ok(Int4),
            Bigint => return Ok(Int8),
            Real => return Ok(Float4),
            Boolean => return Ok(Bool),
            _ => {},
        };

        if kw == Double {
            self.buffer.consume_kw_eq(Precision).required()?;
            return Ok(Float8)
        }

        if kw == Float {
            // opt_float: '(' ICONST ')'
            return match self.i32_literal_paren().optional()? {
                None => Ok(Float8),
                Some(num @ ..=0) => Err(FloatPrecisionUnderflow(num).into()),
                Some(1..=24) => Ok(Float4),
                Some(25..=53) => Ok(Float8),
                Some(num @ 54..) => Err(FloatPrecisionOverflow(num).into()),
            }
        }

        let type_mods = self.opt_type_modifiers()
            .optional()?
            .unwrap_or_else(Vec::new);

        Ok(SystemType::Numeric(type_mods))
    }

    /// Post-condition: Vec **can** be empty
    fn opt_type_modifiers(&mut self) -> ScanResult<Vec<AstNode>> {
        use TokenKind::{CloseParenthesis, OpenParenthesis};

        /*
            '(' expr_list ')'
        */

        self.buffer.consume_eq(OpenParenthesis)?;

        let exprs = self.expr_list()
            .no_match_to_option()
            .required()?
            .unwrap_or_else(Vec::new);

        self.buffer.consume_eq(CloseParenthesis).required()?;

        Ok(exprs)
    }

    /// Post-condition: Vec is **Not** empty
    fn expr_list(&mut self) -> ScanResult<Vec<AstNode>> {

        /*
            a_expr ( ',' a_expr )*
        */

        let expr = self.a_expr()?;
        let mut exprs = vec![expr];

        while self.buffer.consume_eq(Comma).optional()?.is_some() {
            let expr = self.a_expr().required()?;
            exprs.push(expr);
        }

        Ok(exprs)
    }

    /// Post-condition: Vec is **Not** empty
    ///
    /// Alias: `handler_name`
    fn any_name(&mut self) -> ScanResult<QnName> {

        /*
            col_id attrs
        */

        let prefix = self.col_id()?;
        self.attrs(prefix)
    }

    /// Post-condition: Vec is **Not** empty
    fn attrs(&mut self, prefix: CowStr) -> ScanResult<QnName> {

        // A prefix token is passed to prevent a right shift of the Vec later on,
        // to insert the 1st element.

        /*
            prefix ( '.' col_label )*
        */

        let mut attrs = vec![prefix];

        while self.buffer.consume_eq(Dot).optional()?.is_some() {
            let attr = self.col_label().required()?;
            attrs.push(attr);
        }

        Ok(attrs)
    }

    /// Production: `'(' ICONST ')'`
    fn i32_literal_paren(&mut self) -> ScanResult<i32> {
        use TokenKind::{CloseParenthesis, OpenParenthesis};

        self.buffer.consume_eq(OpenParenthesis)?;
        let num = self.i32_literal().required()?;
        self.buffer.consume_eq(CloseParenthesis).required()?;

        Ok(num)
    }

    fn character(&mut self) -> ScanResult<CharacterSystemType> {
        use Keyword::{Char, Character, National, Nchar, Varchar};

        /*
            VARCHAR
          | (CHARACTER | CHAR_P | NCHAR) (VARYING)?
          | NATIONAL ( CHARACTER | CHAR_P) (VARYING)?
        */

        let char_type = self.buffer.consume(|tok|
            tok.keyword().map(KeywordDetails::keyword)
                .filter(|col_name|
                    matches!(col_name, Varchar | Character | Char | National | Nchar)
                )
        )?;

        if char_type == Varchar {
            return Ok(CharacterSystemType::Varchar)
        }

        if char_type == National {

            self.buffer
                .consume(|tok|
                    matches!(
                        tok.keyword().map(KeywordDetails::keyword),
                        Some(Character | Char)
                    )
                )
                .required()?;
        }

        let varying = self.buffer.consume_kw_eq(Keyword::Varying).optional()?;

        let char_type = if varying.is_some() {
            CharacterSystemType::Varchar
        }
        else {
            CharacterSystemType::Bpchar
        };

        Ok(char_type)
    }

    /// Alias: `NumericOnly`
    fn signed_number(&mut self) -> ScanResult<SignedNumber> {
        use TokenKind::{Minus, Plus};

        // ('+' | '-')? (ICONST | FCONST)

        let sign = self.buffer
            .consume(|tok| matches!(tok, Minus | Plus))
            .no_match_to_option()?;

        let number = self.unsigned_number();

        let number = if sign.is_some() {
            number.required()?
        }
        else {
            number?
        };

        let negative = sign.is_some_and(|s| s == Minus);

        let value = match number {
            UnsignedNumber::IConst(int) => {
                // SAFETY: `0 <= int <= i32::MAX`
                let mut int = int as i32;
                if negative {
                    int = -int;
                }
                SignedNumber::SignedIConst(int)
            },
            UnsignedNumber::Numeric { value, radix } => {
                SignedNumber::Numeric { value, radix, negative }
            }
        };

        Ok(value)
    }

    fn unsigned_number(&mut self) -> ScanResult<UnsignedNumber> {

        // ICONST | FCONST

        let loc = self.buffer.current_location();
        let source = self.buffer.source();

        self.buffer.consume(|tok| {
            let NumberLiteral { radix } = tok else { return None };
            let value = loc.slice(source);
            parse_number(value, *radix)
        })
    }

    /// Alias: `SignedIconst`
    fn signed_i32_literal(&mut self) -> ScanResult<i32> {
        use TokenKind::{Minus, Plus};

        // ('+' | '-')? ICONST

        let sign = self.buffer
            .consume(|tok| matches!(tok, Minus | Plus))
            .no_match_to_option()?;

        let num = self.i32_literal();

        let Some(sign) = sign else { return num };

        // If sign is Some(_), then ICONST is required
        let mut num = num.required()?;

        if sign == Minus {
            num = -num;
        }

        Ok(num)
    }

    /// Alias: `ICONST`
    fn i32_literal(&mut self) -> ScanResult<i32> {

        let loc = self.buffer.current_location();
        let source = self.buffer.source();

        self.buffer.consume(|tok| {
            let NumberLiteral { radix } = tok else { return None };

            let value = loc.slice(source);
            let Some(UnsignedNumber::IConst(int)) = parse_number(value, *radix) else { return None };
            // SAFETY: `0 <= int <= i32::MAX`
            Some(int as i32)
        })
    }

    /// Post-condition: Vec is **not** empty
    fn role_list(&mut self) -> ScanResult<Vec<RoleSpec>> {

        /*
            role_spec ( ',' role_spec )*
        */

        let role = self.role_spec()?;
        let mut roles = vec![role];

        while self.buffer.consume_eq(Comma).optional()?.is_some() {
            let role = self.role_spec().required()?;
            roles.push(role);
        }

        Ok(roles)
    }

    /// Alias: `RoleId`
    #[inline]
    fn role_id(&mut self) -> ScanResult<CowStr> {

        // Similar to role_spec, but only allows an identifier, i.e., disallows builtin roles

        self.role_spec()?
            .into_role_id()
            .map_err(ScanErrorKind::from)
    }

    /// Alias: `RoleSpec`
    fn role_spec(&mut self) -> ScanResult<RoleSpec> {

        /*
            role_spec :
                  NONE => Err(ReservedRoleSpec)
                | CURRENT_ROLE
                | CURRENT_USER
                | SESSION_USER
                | "public"
                | non_reserved_word
        */

        if let Some(ident) = self.identifier().no_match_to_option()? {
            return if ident == "public" {
                Ok(RoleSpec::Public)
            }
            else {
                Ok(RoleSpec::Name(ident.into()))
            }
        }

        self.buffer.consume(|tok| {
            use Keyword::{CurrentRole, CurrentUser, NoneKw, SessionUser};

            let Some(kw) = tok.keyword() else { return Ok(None) };

            match kw.keyword() {
                NoneKw => Err(ReservedRoleSpec("none")),
                CurrentRole => Ok(Some(RoleSpec::CurrentRole)),
                CurrentUser => Ok(Some(RoleSpec::CurrentUser)),
                SessionUser => Ok(Some(RoleSpec::SessionUser)),
                _ => {
                    if kw.reserved().is_some() {
                        Ok(None)
                    }
                    else {
                        Ok(Some(
                            RoleSpec::Name(kw.text().into())
                        ))
                    }
                },
            }
        })
    }

    /// Alias: `qual_Op`
    fn qual_op(&mut self) -> ScanResult<QnOperator> {

        /*
            Operator | prefixed_operator
        */

        if let Some(op) = self.operator().no_match_to_option()? {
            let op = AllOp::Operator(op);
            return Ok(QnOperator(vec![], op))
        }

        self.prefixed_operator()
    }

    /// Production: `OPERATOR '(' any_operator ')'`
    fn prefixed_operator(&mut self) -> ScanResult<QnOperator> {
        use Keyword::Operator;
        use TokenKind::{CloseParenthesis, OpenParenthesis};

        self.buffer.consume_kw_eq(Operator)?;

        self.buffer.consume_eq(OpenParenthesis).required()?;
        let op = self.any_operator().required()?;
        self.buffer.consume_eq(CloseParenthesis).required()?;

        Ok(op)
    }

    fn any_operator(&mut self) -> ScanResult<QnOperator> {

        /*
            ( col_id '.' )* all_op
        */

        let mut qn = Vec::new();

        while let Some(id) = self.col_id().optional()? {
            self.buffer.consume_eq(Dot).required()?;
            qn.push(id);
        }

        let op = self.all_op();

        let op = if qn.is_empty() {
            op?
        }
        else {
            op.required()?
        };

        let op = QnOperator(qn, op);
        Ok(op)
    }

    /// Alias: `all_Op`
    fn all_op(&mut self) -> ScanResult<AllOp> {

        if let Some(op) = self.math_op().no_match_to_option()? {
            return Ok(AllOp::MathOp(op))
        }

        self.operator().map(AllOp::Operator)
    }

    /// Returns the text of the `UserDefinedOperator`
    fn operator(&mut self) -> ScanResult<String> {

        let loc = self.buffer.current_location();
        let source = self.buffer.source();
        self.buffer.consume(|tok| match tok {
            TokenKind::UserDefinedOperator => {
                let op = loc.slice(source).to_owned();
                Some(op)
            },
            _ => None
        })
    }

    /// Alias: `MathOp`
    fn math_op(&mut self) -> ScanResult<MathOp> {
        use MathOp::*;
        use TokenKind::{Circumflex, Div, Minus, Mul, Percent, Plus};

        self.buffer.consume(|tok| match tok {
            Plus => Some(Addition),
            Minus => Some(Subtraction),
            Mul => Some(Multiplication),
            Div => Some(Division),
            Percent => Some(Modulo),
            Circumflex => Some(Exponentiation),
            TokenKind::Less => Some(Less),
            TokenKind::Greater => Some(Greater),
            TokenKind::Equals => Some(Equals),
            TokenKind::LessEquals => Some(LessEquals),
            TokenKind::GreaterEquals => Some(GreaterEquals),
            TokenKind::NotEquals => Some(NotEquals),
            _ => None
        })
    }

    /// Aliases:
    /// * `ColId`
    /// * `name`
    #[inline(always)]
    fn col_id(&mut self) -> ScanResult<CowStr> {
        self.ident_or_keyword(|kw|
               kw.unreserved().is_some()
            || kw.col_name().is_some()
        )
    }

    #[inline(always)]
    fn type_function_name(&mut self) -> ScanResult<CowStr> {
        self.ident_or_keyword(|kw|
               kw.unreserved().is_some()
            || kw.type_func_name().is_some()
        )
    }

    /// Alias: `NonReservedWord`
    #[inline(always)]
    fn non_reserved_word(&mut self) -> ScanResult<CowStr> {
        self.ident_or_keyword(|kw|
               kw.unreserved().is_some()
            || kw.col_name().is_some()
            || kw.type_func_name().is_some()
        )
    }

    /// Aliases:
    /// * `ColLabel`
    /// * `attr_name`
    #[inline(always)]
    fn col_label(&mut self) -> ScanResult<CowStr> {
        self.ident_or_keyword(|_| true)
    }

    /// Alias: `BareColLabel`
    #[inline(always)]
    fn bare_col_label(&mut self) -> ScanResult<CowStr> {
        self.ident_or_keyword(KeywordDetails::bare)
    }

    fn ident_or_keyword<P>(&mut self, pred: P) -> ScanResult<CowStr>
    where
        P: Fn(&KeywordDetails) -> bool
    {
        if let Some(ident) = self.identifier().no_match_to_option()? {
            return Ok(ident.into())
        }

        self.buffer.consume(|tok| match tok.keyword() {
            Some(kw) if pred(kw) => Some(kw.text().into()),
            _ => None,
        })
    }

    /// Aliases:
    /// * `SCONST`
    /// * `USCONST`
    /// * `file_name`
    #[inline(always)]
    fn string(&mut self) -> ScanResult<String> {
        StringParser(self).parse()
    }

    /// Alias: `IDENT`
    #[inline(always)]
    fn identifier(&mut self) -> ScanResult<String> {
        IdentifierParser(self).parse()
    }

    /// Production: `UESCAPE SCONST`
    fn uescape(&mut self) -> Result<char, ParserErrorKind> {
        use Keyword::Uescape;

        // Try to consume UESCAPE + the string following it.
        // see [base_yylex](https://github.com/postgres/postgres/blob/1c61fd8b527954f0ec522e5e60a11ce82628b681/src/backend/parser/parser.c#L256)

        if self.buffer.consume_kw_eq(Uescape).optional()?.is_none() {
            return Ok('\\')
        };

        let loc = self.buffer.current_location();
        let source = self.buffer.source();

        let uescape = self.buffer
            .consume(|tok| match tok.string_kind() {
                Some(_) => {
                    let slice = loc.slice(source);
                    match uescape_escape(slice) {
                        Some(escape) => Ok(Some(escape)),
                        None => Err(InvalidUescapeDelimiter),
                    }
                },
                None => Err(UescapeDelimiterMissing)
            });

        uescape.map_err(|err| match err {
            Eof => InvalidUescapeDelimiter,
            NoMatch => ParserErrorKind::default(),
            ParserErr(err) => err
        })
    }
}

/// Returns UESCAPE's escape char if the string is valid.
#[inline] // Only called from a single place
fn uescape_escape(source: &str) -> Option<char> {

    if source.len() != 3 {
        // Only (some) ASCII chars are acceptable as the escape char
        return None
    }

    let mut chars = source.chars();
    if !chars.next().is_some_and(|c| c == '\'') {
        return None
    }

    let escape = chars.next()?;
    if ascii::is_hex_digit(escape)
        || ascii::is_whitespace(escape)
        || escape == '+'
        || escape == '\''
        || escape == '"'
    {
        return None
    }

    match chars.next() {
        Some('\'') => Some(escape),
        _ => None
    }
}

fn parse_number(value: &str, radix: u32) -> Option<UnsignedNumber> {

    let value = value.replace("_", "");

    match i32::from_str_radix(&value, radix) {
        Ok(int) => {
            // SAFETY: `0 <= int <= i32::MAX`
            Some(UnsignedNumber::IConst(int as u32))
        },
        Err(_) => {
            Some(UnsignedNumber::Numeric { value, radix })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use postgres_basics::guc::BackslashQuote;

    pub(in crate::parser) const DEFAULT_CONFIG: ParserConfig = ParserConfig::new(true, BackslashQuote::SafeEncoding);

    #[test]
    fn test_toplevel_stmt() {
        let sources = [
            "begin transaction",
            "start transaction",
            "end transaction",
        ];

        for source in sources {
            let mut parser = Parser::new(source, DEFAULT_CONFIG);
            let actual = parser.toplevel_stmt();

            // This only quickly tests that statement types aren't missing.
            // More in-depth testing is within each statement's module.
            assert_matches!(actual, Ok(_),
                r"expected Ok(Some(_)) for {source:?} but actually got {actual:?}"
            )
        }
    }

    #[test]
    fn test_stmt() {
        let sources = [
            // TODO: analyze, call, cluster, comment, copy, do, drop, explain, fetch, import, lock, move,
            //       reindex, revoke, security, set, truncate, vacuum
            "abort transaction",
            "alter group some_group add user public",
            "close all",
            "commit and no chain",
            "deallocate all",
            "discard all",
            "listen ident",
            "load 'test string'",
            "notify test_ident, 'test-payload'",
            "prepare transaction 'tx id'",
            "reassign owned by public, test_role to target_role",
            "release savepoint test_ident",
            "rollback to test_ident",
            "savepoint test_ident",
            "show all",
            "start transaction read only, read write deferrable",
            "unlisten *",
        ];

        for source in sources {
            let mut parser = Parser::new(source, DEFAULT_CONFIG);
            let actual = parser.stmt();

            // This only quickly tests that statement types aren't missing.
            // More in-depth testing is within each statement's module.
            assert_matches!(actual, Ok(_),
                r"expected Ok(Some(_)) for {source:?} but actually got {actual:?}"
            )
        }
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
        assert_eq!(Err(NoMatch), parser.transaction_mode_list());

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
    fn test_var_name() {
        let mut parser = Parser::new("test.qualified.name", DEFAULT_CONFIG);
        let expected = vec![
            "test".into(),
            "qualified".into(),
            "name".into()
        ];

        assert_eq!(Ok(expected), parser.var_name());
    }

    #[test]
    fn test_numeric() {

        let source = "boolean smallint int integer bigint real float float(17) float(44) double precision";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = [
            Bool,
            Int2,
            Int4,
            Int4,
            Int8,
            Float4,
            Float8,
            Float4,
            Float8,
            Float8,
        ];

        for e in expected {
            assert_eq!(Ok(e), parser.numeric());
        }

        // TODO: (DECIMAL | DEC | NUMERIC) opt_type_modifiers
    }

    #[test] #[ignore]
    fn test_opt_type_modifiers() {
        todo!()
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
    fn test_any_name() {
        let source = "some_.qualified_.name_";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        let actual = parser.any_name();

        let expected: QnName = vec![
            "some_".into(),
            "qualified_".into(),
            "name_".into()
        ];

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_attrs() {
        let source = ".qualified_.name_";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        let actual = parser.attrs("*some*".into());

        let expected: QnName = vec![
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
    fn test_character() {
        use CharacterSystemType::{Bpchar, Varchar};

        let sources = [
            (Varchar, "varchar"),
            (Varchar, "char varying"),
            (Varchar, "character varying"),
            (Varchar, "nchar varying"),
            (Varchar, "national char varying"),
            (Varchar, "national character varying"),
            (Bpchar, "char"),
            (Bpchar, "character"),
            (Bpchar, "nchar"),
            (Bpchar, "national char"),
            (Bpchar, "national character"),
        ];

        for (expected, source) in sources {
            let mut parser = Parser::new(source, DEFAULT_CONFIG);
            let actual = parser.character();
            assert_eq!(Ok(expected), actual,
                r"expected {expected:?} for source {source:?} but actually got {actual:?}",
            );
        }
    }

    #[test]
    fn test_signed_number() {
        use SignedNumber::{Numeric, SignedIConst};

        let mut parser = Parser::new("1.01 +2.02 -3.03 101 +202 -303", DEFAULT_CONFIG);

        let expected = vec![
            Numeric { value: "1.01".into(), radix: 10, negative: false },
            Numeric { value: "2.02".into(), radix: 10, negative: false },
            Numeric { value: "3.03".into(), radix: 10, negative: true },
            SignedIConst(101),
            SignedIConst(202),
            SignedIConst(-303),
        ];

        for e in expected {
            let actual = parser.signed_number();
            assert_eq!(Ok(e), actual);
        }
    }

    #[test]
    fn test_unsigned_number() {
        use UnsignedNumber::{IConst, Numeric};

        let mut parser = Parser::new("1.1 11", DEFAULT_CONFIG);

        let actual = parser.unsigned_number();
        assert_eq!(Ok(Numeric { value: "1.1".into(), radix: 10 }), actual);

        let actual = parser.unsigned_number();
        assert_eq!(Ok(IConst(11)), actual);
    }

    #[test]
    fn test_signed_i32_literal() {
        let mut parser = Parser::new("-123 +321", DEFAULT_CONFIG);
        assert_eq!(Ok(-123), parser.signed_i32_literal());
        assert_eq!(Ok(321), parser.signed_i32_literal());
    }

    #[test]
    fn test_i32_literal() {
        let mut parser = Parser::new("123", DEFAULT_CONFIG);
        assert_eq!(Ok(123), parser.i32_literal());
    }

    #[test]
    fn test_role_list() {
        let source = "public , CuRrEnT_rOlE,CURRENT_USER, session_user ,coalesce,xxYYzz none";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.role_list().unwrap();

        let expected = [
            RoleSpec::Public,
            RoleSpec::CurrentRole,
            RoleSpec::CurrentUser,
            RoleSpec::SessionUser,
            RoleSpec::Name("coalesce".into()),
            RoleSpec::Name("xxyyzz".into()),
        ];

        assert_eq!(expected, actual.as_slice());
    }

    #[test]
    fn test_role_id() {

        let mut parser = Parser::new("coalesce xxyyzz", DEFAULT_CONFIG);
        assert_eq!(Ok("coalesce".into()), parser.role_id());
        assert_eq!(Ok("xxyyzz".into()), parser.role_id());

        let mut parser = Parser::new("none", DEFAULT_CONFIG);
        assert_eq!(Err(ParserErr(ReservedRoleSpec("none"))), parser.role_id());

        let mut parser = Parser::new("public", DEFAULT_CONFIG);
        assert_eq!(Err(ParserErr(ReservedRoleSpec("public"))), parser.role_id());

        let mut parser = Parser::new("current_role", DEFAULT_CONFIG);
        assert_eq!(Err(ParserErr(ForbiddenRoleSpec("CURRENT_ROLE"))), parser.role_id());

        let mut parser = Parser::new("current_user", DEFAULT_CONFIG);
        assert_eq!(Err(ParserErr(ForbiddenRoleSpec("CURRENT_USER"))), parser.role_id());

        let mut parser = Parser::new("session_user", DEFAULT_CONFIG);
        assert_eq!(Err(ParserErr(ForbiddenRoleSpec("SESSION_USER"))), parser.role_id());
    }

    #[test]
    fn test_role_spec() {
        let source = "public CuRrEnT_rOlE CURRENT_USER session_user coalesce xxyyzz";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(RoleSpec::Public), parser.role_spec());
        assert_eq!(Ok(RoleSpec::CurrentRole), parser.role_spec());
        assert_eq!(Ok(RoleSpec::CurrentUser), parser.role_spec());
        assert_eq!(Ok(RoleSpec::SessionUser), parser.role_spec());
        assert_eq!(Ok(RoleSpec::Name("coalesce".into())), parser.role_spec());
        assert_eq!(Ok(RoleSpec::Name("xxyyzz".into())), parser.role_spec());

        let mut parser = Parser::new("collate", DEFAULT_CONFIG);
        assert_eq!(Err(NoMatch), parser.role_spec());

        let mut parser = Parser::new("none", DEFAULT_CONFIG);
        assert_eq!(Err(ParserErr(ReservedRoleSpec("none"))), parser.role_spec());
    }

    #[test]
    fn test_qual_op() {
        use ast_node::{AllOp, QnOperator};

        let source = "operator(|/) <@>";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = QnOperator(
            vec![],
            AllOp::Operator("|/".into())
        );
        assert_eq!(Ok(expected), parser.qual_op());

        let expected = QnOperator(
            vec![],
            AllOp::Operator("<@>".into())
        );
        assert_eq!(Ok(expected), parser.qual_op());
    }

    #[test]
    fn test_prefixed_operator() {
        use ast_node::{AllOp, MathOp, QnOperator};

        let source = "operator(some_qn.*)";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.prefixed_operator();
        let expected = QnOperator(
            vec!["some_qn".into()],
            AllOp::MathOp(MathOp::Multiplication)
        );
        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_any_operator() {
        use ast_node::{AllOp, MathOp, QnOperator};

        let source = "@@ != qn_name.+";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = QnOperator(
            vec![],
            AllOp::Operator("@@".into())
        );
        assert_eq!(Ok(expected), parser.any_operator());

        let expected = QnOperator(
            vec![],
            AllOp::MathOp(MathOp::NotEquals)
        );
        assert_eq!(Ok(expected), parser.any_operator());

        let expected = QnOperator(
            vec!["qn_name".into()],
            AllOp::MathOp(MathOp::Addition)
        );
        assert_eq!(Ok(expected), parser.any_operator());
    }

    #[test]
    fn test_all_op() {
        use AllOp::*;
        use ast_node::MathOp::NotEquals;

        let source = "~@ <>";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(Operator("~@".into())), parser.all_op());
        assert_eq!(Ok(MathOp(NotEquals)), parser.all_op());
    }

    #[test]
    fn test_operator() {
        let source = "~@";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("~@".into()), parser.operator());
    }

    #[test]
    fn test_math_op() {
        use MathOp::*;

        let source = "+ - * / % ^ < > = <= >= != <>";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(Addition), parser.math_op());
        assert_eq!(Ok(Subtraction), parser.math_op());
        assert_eq!(Ok(Multiplication), parser.math_op());
        assert_eq!(Ok(Division), parser.math_op());
        assert_eq!(Ok(Modulo), parser.math_op());
        assert_eq!(Ok(Exponentiation), parser.math_op());
        assert_eq!(Ok(Less), parser.math_op());
        assert_eq!(Ok(Greater), parser.math_op());
        assert_eq!(Ok(Equals), parser.math_op());
        assert_eq!(Ok(LessEquals), parser.math_op());
        assert_eq!(Ok(GreaterEquals), parser.math_op());
        assert_eq!(Ok(NotEquals), parser.math_op());
        assert_eq!(Ok(NotEquals), parser.math_op());
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
    fn test_string() {
        let source = "'test string'";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!("test string", parser.string().unwrap());
    }

    #[test]
    fn test_identifier() {
        let source = "tEsT_iDeNtIfIeR";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!("test_identifier", parser.identifier().unwrap());
    }

    #[test]
    fn test_uescape() {
        let source = "UESCAPE '!'";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.uescape().unwrap();

        assert_eq!('!', actual);
    }
}

use self::ast_node::AstNode::{self, ClosePortalStmt, ListenStmt, LoadStmt};
use self::ast_node::CharacterSystemType;
use self::ast_node::EventTriggerState;
use self::ast_node::IsolationLevel;
use self::ast_node::RoleSpec;
use self::ast_node::SystemType::{self, Bool, Float4, Float8, Int2, Int4, Int8};
use self::ast_node::TransactionMode;
use self::ast_node::{AllOp, MathOp, QnOperator, SignedNumber, UnsignedNumber};
use self::error::ParserErrorKind::*;
use self::ident_parser::IdentifierParser;
use self::result::ScanErrorKind::{self, Eof, NoMatch, ParserErr};
use self::result::ScanResultTrait;
use self::string_parser::StringParser;
use self::token_buffer::TokenBuffer;
use self::token_buffer::TokenConsumer;
use crate::lexer::Keyword;
use crate::lexer::KeywordDetails;
use crate::lexer::Lexer;
use crate::lexer::TokenKind::{self, Comma, Dot, NumberLiteral};
use crate::parser::result::{EofResultTrait, ScanResult};
use postgres_basics::ascii;
use postgres_basics::Located;
use postgres_basics::Location;
use std::borrow::Cow;
use std::mem;
