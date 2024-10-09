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

macro_rules! list_production {
    (gather $production:block delim $separator:block) => {
        list_production!(prefix $production gather $production delim $separator)
    };
    (prefix $prefix:block gather $production:block delim $separator:block) => {
        (|| {
            let mut elements = match $prefix? {
                None => return Ok(None),
                Some(element) => vec![element],
            };

            while {
                match $separator {
                    Ok(None) | Err(None) => false,
                    Ok(Some(_)) => true,
                    Err(err) => return Err(err),
                }
            } {
                let element = $production.required()?;
                elements.push(element)
            }

            Ok(Some(elements))
        })()
    };
}

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
            Ok(Some(stmt)) => vec![stmt],
            Ok(None) => {
                // something went wrong with the 1st token already?!
                return Err(ParserErrorKind::default())
            }
            Err(None) => { // eof
                // The string didn't have anything useful: empty, whitespace or comments
                return Ok(Vec::new());
            },
            Err(Some(err)) => return Err(err),
        };

        while self.semicolons()? {

            let stmt = match self.toplevel_stmt() {
                Ok(Some(stmt)) => stmt,
                Ok(None) => {
                    // No stmt matched
                    return Err(ParserErrorKind::default())
                }
                Err(Some(err)) => return Err(err),
                Err(None) => break,
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

        let mut saw_semicolon = false;
        loop {
            match self.buffer.consume_eq(Semicolon) {
                Ok(Some(_)) => {
                    saw_semicolon = true;
                }
                Ok(None) | Err(None) => {
                    // not a semicolon
                    break
                },
                Err(Some(err)) => return Err(err),
            }
        }

        Ok(saw_semicolon)
    }

    fn toplevel_stmt(&mut self) -> OptResult<AstNode> {

        if let Some(node) = self.stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.begin_stmt()? { Ok(Some(node.into())) }
        else if let Some(node) = self.end_stmt()? { Ok(Some(node.into())) }
        else { Ok(None) }
    }

    fn stmt(&mut self) -> OptResult<AstNode> {

        if self.buffer.consume_kw_eq(Keyword::Checkpoint)?.is_some() {
            return Ok(Some(AstNode::CheckPoint))
        }

        if let Some(node) = self.abort_stmt()? { Ok(Some(node.into())) }
        else if let Some(node) = self.alter_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.analyze_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.call_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.close_stmt()? { Ok(Some(ClosePortalStmt(node))) }
        else if let Some(node) = self.cluster_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.comment_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.commit_stmt()? { Ok(Some(node.into())) }
        else if let Some(node) = self.copy_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.deallocate_stmt()? { Ok(Some(ClosePortalStmt(node))) }
        else if let Some(node) = self.discard_stmt()? { Ok(Some(node.into())) }
        else if let Some(node) = self.do_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.drop_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.explain_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.fetch_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.import_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.listen_stmt()? { Ok(Some(ListenStmt(node))) }
        else if let Some(node) = self.load_stmt()? { Ok(Some(LoadStmt(node))) }
        else if let Some(node) = self.lock_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.move_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.notify_stmt()? { Ok(Some(node.into())) }
        else if let Some(node) = self.prepare_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.reassign_owned_stmt()? { Ok(Some(node.into())) }
        else if let Some(node) = self.reindex_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.release_savepoint_stmt()? { Ok(Some(node.into())) }
        else if let Some(node) = self.revoke_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.rollback_stmt()? { Ok(Some(node.into())) }
        else if let Some(node) = self.savepoint_stmt()? { Ok(Some(node.into())) }
        else if let Some(node) = self.security_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.set_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.show_stmt()? { Ok(Some(node.into())) }
        else if let Some(node) = self.start_transaction_stmt()? { Ok(Some(node.into())) }
        else if let Some(node) = self.truncate_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.unlisten_stmt()? { Ok(Some(ClosePortalStmt(node))) }
        else if let Some(node) = self.vacuum_stmt()? { Ok(Some(node)) }
        else { Ok(None) }
    }

    fn opt_transaction(&mut self) -> ReqResult<()> {
        use Keyword::{Transaction, Work};

        // Skips over WORK | TRANSACTION
        self.buffer.consume(|tok|
            matches!(tok.keyword().map(KeywordDetails::keyword), Some(Work | Transaction))
        ).replace_eof(Ok(None))?;

        Ok(())
    }

    fn opt_transaction_chain(&mut self) -> ReqResult<bool> {
        use Keyword::{And, Chain, No};

        if self.buffer.consume_kw_eq(And).replace_eof(Ok(None))?.is_none() {
            return Ok(false)
        }

        let result = self.buffer.consume_kw_eq(No).replace_eof(Ok(None))?.is_none();

        self.buffer.consume_kw_eq(Chain).required()?;

        Ok(result)
    }

    /// Post-condition: if `Ok(Some(_))`, then Vec is **Not** empty
    /// Alias: `transaction_mode_list_or_empty`
    fn opt_transaction_mode_list(&mut self) -> OptResult<Vec<TransactionMode>> {

        let mut elements = match self.transaction_mode()? {
            None => return Ok(None),
            Some(element) => vec![element],
        };

        loop {
            let comma = match self.buffer.consume_eq(Comma) {
                Ok(comma) => comma.is_some(),
                Err(None) => break,
                Err(Some(err)) => return Err(Some(err)),
            };

            match self.transaction_mode().replace_eof(Ok(None))? {
                Some(element) => elements.push(element),
                None => {
                    if comma { return Err(Some(ParserErrorKind::default())) }
                    break
                },
            }
        }

        Ok(Some(elements))
    }

    /// Alias: `transaction_mode_item`
    fn transaction_mode(&mut self) -> OptResult<TransactionMode> {
        use Keyword::{Deferrable, Isolation, Level, Not, Only, Read, Write};

        /*
            ISOLATION LEVEL iso_level
            READ ONLY
            READ WRITE
            DEFERRABLE
            NOT DEFERRABLE
        */

        let result = self.buffer.consume(|tok|
            tok.keyword().map(KeywordDetails::keyword)
                .filter(|kw| matches!(kw, Deferrable | Not | Isolation | Read))
        )?;

        let Some(result) = result else { return Ok(None) };

        match result {
            Deferrable => Ok(Some(TransactionMode::Deferrable)),
            Not => {
                self.buffer.consume_kw_eq(Deferrable).required()?;
                Ok(Some(TransactionMode::NotDeferrable))
            },
            Isolation => {
                self.buffer.consume_kw_eq(Level).required()?;
                let isolation_level = self.isolation_level()?;
                Ok(Some(TransactionMode::IsolationLevel(isolation_level)))
            },
            Read => {
                self.buffer.consume(|tok|
                    match tok.keyword().map(KeywordDetails::keyword) {
                        Some(Only) => Some(TransactionMode::ReadOnly),
                        Some(Write) => Some(TransactionMode::ReadWrite),
                        _ => None
                    }
                ).required().optional()
            },
            _ => Ok(None)
        }
    }

    /// Alias: `iso_level`
    fn isolation_level(&mut self) -> ReqResult<IsolationLevel> {
        use Keyword::{Committed, Read, Repeatable, Serializable, Uncommitted};

        /*
            READ UNCOMMITTED
            READ COMMITTED
            REPEATABLE READ
            SERIALIZABLE
        */

        let result = self.buffer.consume(|tok|
            tok.keyword().map(KeywordDetails::keyword)
                .filter(|kw| matches!(kw, Read | Repeatable | Serializable))
        ).required()?;

        match result {
            Serializable => Ok(IsolationLevel::Serializable),
            Repeatable => {
                self.buffer.consume_kw_eq(Read).required()?;
                Ok(IsolationLevel::RepeatableRead)
            },
            Read => {
                let result = self.buffer.consume(|tok|
                    tok.keyword().map(KeywordDetails::keyword)
                        .filter(|kw| matches!(kw, Committed | Uncommitted))
                ).required()?;

                match result {
                    Committed => Ok(IsolationLevel::ReadCommitted),
                    Uncommitted => Ok(IsolationLevel::ReadUncommitted),
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }

    fn var_name(&mut self) -> ReqResult<QnName> {

        list_production!(
            gather { self.col_id() }
            delim { self.buffer.consume_eq(Dot) }
        ).required()
    }

    /// Alias: `AexprConst`
    fn a_expr_const(&mut self) -> OptResult<()> {

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
    fn const_typename(&mut self) -> OptResult<SystemType> {
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

        if let Some(num) = self.numeric()? {
            return Ok(Some(num))
        }

        if let Some(character) = self.character()? {

            let len = self.i32_literal_paren().replace_eof(Ok(None))?;
            let character = match character {
                CharacterSystemType::Varchar => { SystemType::Varchar(len) },
                CharacterSystemType::Bpchar => SystemType::Bpchar(len),
            };

            return Ok(Some(character))
        }

        let bit = self.buffer.consume_kw_eq(Bit)?;
        if bit.is_some() {
            let varying = self.buffer.consume_kw_eq(Varying)
                .replace_eof(Ok(None))?;
            // TODO: self.expr_list()
        }

        todo!()
    }

    /// Alias: `Numeric`<p/>
    /// Inline: `opt_float`
    fn numeric(&mut self) -> OptResult<SystemType> {
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

        let kw = match kw {
            None => return Ok(None),
            Some(Smallint) => return Ok(Some(Int2)),
            Some(Int | Integer) => return Ok(Some(Int4)),
            Some(Bigint) => return Ok(Some(Int8)),
            Some(Real) => return Ok(Some(Float4)),
            Some(Boolean) => return Ok(Some(Bool)),
            Some(kw) => kw,
        };

        if kw == Double {
            self.buffer.consume_kw_eq(Precision).required()?;
            return Ok(Some(Float8))
        }

        if kw == Float {
            // opt_float: '(' ICONST ')'
            return match self.i32_literal_paren().replace_eof(Ok(None))? {
                None => Ok(Some(Float8)),
                Some(num @ ..=0) => Err(Some(FloatPrecisionUnderflow(num))),
                Some(1..=24) => Ok(Some(Float4)),
                Some(25..=53) => Ok(Some(Float8)),
                Some(num @ 54..) => Err(Some(FloatPrecisionOverflow(num))),
            }
        }

        let type_mods = self.opt_type_modifiers()
            .replace_eof(Ok(None))?
            .unwrap_or_else(Vec::new);

        Ok(Some(SystemType::Numeric(type_mods)))
    }

    fn opt_type_modifiers(&mut self) -> OptResult<Vec<AstNode>> {
        use TokenKind::{CloseParenthesis, OpenParenthesis};

        // '(' expr_list ')'

        if self.buffer.consume_eq(OpenParenthesis)?.is_none() {
            return Ok(None)
        }

        let exprs = self.expr_list()?;

        self.buffer.consume_eq(CloseParenthesis).required()?;

        Ok(Some(exprs))
    }

    /// Post-condition: Vec is **Not** empty
    fn expr_list(&mut self) -> ReqResult<Vec<AstNode>> {

        // a_expr ( ',' a_expr )*

        list_production!(
            gather { self.a_expr() }
            delim { self.buffer.consume_eq(Comma) }
        ).required()
    }

    /// Alias: `handler_name`
    fn any_name(&mut self) -> OptResult<QnName> {
        let Some(prefix) = self.col_id()? else { return Ok(None) };
        self.attrs(prefix)
    }

    fn attrs(&mut self, prefix: CowStr) -> OptResult<QnName> {

        // A prefix token is passed to prevent a right shift of the Vec later on.

        list_production!(
            prefix { Ok::<Option<CowStr>, Option<ParserErrorKind>>(Some(prefix)) }
            gather { self.col_label() }
            delim { self.buffer.consume_eq(Dot) }
        )
    }

    /// Production: `'(' ICONST ')'`
    fn i32_literal_paren(&mut self) -> OptResult<i32> {
        use TokenKind::{CloseParenthesis, OpenParenthesis};

        if self.buffer.consume_eq(OpenParenthesis)?.is_none() {
            return Ok(None)
        }

        let num = self.i32_literal().required()?;

        self.buffer.consume_eq(CloseParenthesis).required()?;

        Ok(Some(num))
    }

    fn character(&mut self) -> OptResult<CharacterSystemType> {
        use Keyword::{Char, Character, National, Nchar, Varchar};

        /*
        character :
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

        let Some(char_type) = char_type else {
            return Ok(None)
        };

        if char_type == Varchar {
            return Ok(Some(CharacterSystemType::Varchar))
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

        let varying = self.buffer.consume_kw_eq(Keyword::Varying)
            .replace_eof(Ok(None))?;

        let char_type = if varying.is_some() {
            CharacterSystemType::Varchar
        }
        else {
            CharacterSystemType::Bpchar
        };

        Ok(Some(char_type))
    }

    /// Alias: `NumericOnly`
    fn signed_number(&mut self) -> OptResult<SignedNumber> {
        use TokenKind::{Minus, Plus};

        // ('+' | '-')? (ICONST | FCONST)

        let sign = self.buffer.consume(|tok|
            match tok {
                Minus | Plus => Some(*tok),
                _ => None
            }
        )?;

        let number = self.unsigned_number();

        let number = if sign.is_some() {
            number.required()?
        }
        else {
            let Some(number) = number? else { return Ok(None) };
            number
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

        Ok(Some(value))
    }

    fn unsigned_number(&mut self) -> OptResult<UnsignedNumber> {

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
    fn signed_i32_literal(&mut self) -> OptResult<i32> {
        use TokenKind::{Minus, Plus};

        // ('+' | '-')? ICONST

        let sign = self.buffer.consume(|tok|
            match tok {
                Minus | Plus => Some(*tok),
                _ => None
            }
        )?;

        let num = self.i32_literal();

        // If sign is Some(_), then ICONST is required

        match sign {
            None => num,
            Some(Minus) => {
                num.map(|v| v.map(|v| -v))
                    .required()
                    .optional()
            },
            Some(_) => {
                num.required().optional()
            }
        }
    }

    /// Alias: `ICONST`
    fn i32_literal(&mut self) -> OptResult<i32> {

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

    fn role_list(&mut self) -> ReqResult<Vec<RoleSpec>> {

        list_production!(
            gather { self.role_spec() }
            delim  { self.buffer.consume_eq(Comma) }
        ).required()
    }

    /// Alias: `RoleId`
    #[inline]
    fn role_id(&mut self) -> OptResult<CowStr> {

        // Similar to role_spec, but only allows an identifier, i.e., disallows builtin roles

        match self.role_spec()? {
            None => Ok(None),
            Some(role_spec) => {
                match role_spec.into_role_id() {
                    Ok(role) => Ok(Some(role)),
                    Err(err) => Err(Some(err))
                }
            },
        }
    }

    /// Alias: `RoleSpec`
    fn role_spec(&mut self) -> OptResult<RoleSpec> {

        /*
            role_spec :
                  NONE => Err(ReservedRoleSpec)
                | CURRENT_ROLE
                | CURRENT_USER
                | SESSION_USER
                | "public"
                | non_reserved_word
        */

        if let Some(ident) = self.identifier()? {
            return if ident == "public" {
                Ok(Some(RoleSpec::Public))
            }
            else {
                Ok(Some(RoleSpec::Name(ident.into())))
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
    fn qual_op(&mut self) -> OptResult<QnOperator> {

        if let Some(op) = self.operator()? {
            let op = AllOp::Operator(op);
            return Ok(Some(QnOperator(vec![], op)))
        }

        self.prefixed_operator()
    }

    /// Production: `OPERATOR '(' any_operator ')'`
    fn prefixed_operator(&mut self) -> OptResult<QnOperator> {
        use Keyword::Operator;
        use TokenKind::{CloseParenthesis, OpenParenthesis};

        if self.buffer.consume_kw_eq(Operator)?.is_none() {
            return Ok(None);
        }

        self.buffer.consume_eq(OpenParenthesis).required()?;
        let op = self.any_operator().required()?;
        self.buffer.consume_eq(CloseParenthesis).required()?;

        Ok(Some(op))
    }

    fn any_operator(&mut self) -> OptResult<QnOperator> {

        let mut qn = Vec::new();

        loop {
            let id = self.col_id()
                .map_eof(|| {
                    let err = if qn.is_empty() { None } else { Some(ParserErrorKind::default()) };
                    Err(err)
                })?;

            let Some(id) = id else { break };
            self.buffer.consume_eq(Dot).required()?;

            qn.push(id);
        }

        let op = self.all_op();

        let op = if qn.is_empty() {
            let Some(op) = op? else { return Ok(None) };
            op
        }
        else {
            op.required()?
        };

        let op = QnOperator(qn, op);
        Ok(Some(op))
    }

    /// Alias: `all_Op`
    fn all_op(&mut self) -> OptResult<AllOp> {

        if let Some(op) = self.math_op()? {
            return Ok(Some(AllOp::MathOp(op)))
        }

        if let Some(op) = self.operator()? {
            return Ok(Some(AllOp::Operator(op)))
        }

        Ok(None)
    }

    /// Returns the text of the `UserDefinedOperator`
    fn operator(&mut self) -> OptResult<String> {

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
    fn math_op(&mut self) -> OptResult<MathOp> {
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
    fn col_id(&mut self) -> OptResult<CowStr> {
        self.ident_or_keyword(|kw|
               kw.unreserved().is_some()
            || kw.col_name().is_some()
        )
    }

    #[inline(always)]
    fn type_function_name(&mut self) -> OptResult<CowStr> {
        self.ident_or_keyword(|kw|
               kw.unreserved().is_some()
            || kw.type_func_name().is_some()
        )
    }

    /// Alias: `NonReservedWord`
    #[inline(always)]
    fn non_reserved_word(&mut self) -> OptResult<CowStr> {
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
    fn col_label(&mut self) -> OptResult<CowStr> {
        self.ident_or_keyword(|_| true)
    }

    /// Alias: `BareColLabel`
    #[inline(always)]
    fn bare_col_label(&mut self) -> OptResult<CowStr> {
        self.ident_or_keyword(KeywordDetails::bare)
    }

    fn ident_or_keyword<P>(&mut self, pred: P) -> OptResult<CowStr>
    where
        P: Fn(&KeywordDetails) -> bool
    {
        if let Some(ident) = self.identifier()? {
            return Ok(Some(ident.into()))
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
    fn string(&mut self) -> OptResult<String> {
        StringParser(self).parse()
    }

    /// Alias: `IDENT`
    #[inline(always)]
    fn identifier(&mut self) -> OptResult<String> {
        IdentifierParser(self).parse()
    }

    /// Production: `UESCAPE SCONST`
    fn uescape(&mut self) -> Result<char, ParserErrorKind> {
        use Keyword::Uescape;

        // Try to consume UESCAPE + the string following it.
        // see [base_yylex](https://github.com/postgres/postgres/blob/1c61fd8b527954f0ec522e5e60a11ce82628b681/src/backend/parser/parser.c#L256)

        let uescape = self.buffer.consume_kw_eq(Uescape);

        match uescape {
            Ok(None) | Err(None) => return Ok('\\'),
            Err(Some(err)) => return Err(err),
            Ok(Some(_)) => {/* it matched */}
        }

        let loc = self.buffer.current_location();
        let source = self.buffer.source();

        let escape = self.buffer
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

        match escape {
            Ok(Some(escape)) => Ok(escape),
            Ok(None) => unreachable!("replaced with Err(UescapeDelimiterMissing)"),
            Err(Some(err)) => Err(err),
            Err(None) => Err(InvalidUescapeDelimiter)
        }
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
            assert_matches!(actual, Ok(Some(_)),
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
            assert_matches!(actual, Ok(Some(_)),
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
        assert_eq!(Ok(None), parser.opt_transaction_mode_list());

        let mut parser = Parser::new(
            "read only , read write isolation level read committed",
            DEFAULT_CONFIG
        );

        let expected = vec![
            TransactionMode::ReadOnly,
            TransactionMode::ReadWrite,
            TransactionMode::IsolationLevel(IsolationLevel::ReadCommitted),
        ];

        assert_eq!(Ok(Some(expected)), parser.opt_transaction_mode_list());
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
            assert_eq!(Ok(Some(expected_mode)), parser.transaction_mode());
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
        let actual = parser.numeric().unwrap().unwrap();
        assert_eq!(Bool, actual);
        let actual = parser.numeric().unwrap().unwrap();
        assert_eq!(Int2, actual);
        let actual = parser.numeric().unwrap().unwrap();
        assert_eq!(Int4, actual);
        let actual = parser.numeric().unwrap().unwrap();
        assert_eq!(Int4, actual);
        let actual = parser.numeric().unwrap().unwrap();
        assert_eq!(Int8, actual);
        let actual = parser.numeric().unwrap().unwrap();
        assert_eq!(Float4, actual);
        let actual = parser.numeric().unwrap().unwrap();
        assert_eq!(Float8, actual);
        let actual = parser.numeric().unwrap().unwrap();
        assert_eq!(Float4, actual);
        let actual = parser.numeric().unwrap().unwrap();
        assert_eq!(Float8, actual);
        let actual = parser.numeric().unwrap().unwrap();
        assert_eq!(Float8, actual);

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

        assert_matches!(actual, Ok(Some(_)));
        let actual = actual.unwrap().unwrap();

        let expected: QnName = vec![
            "some_".into(),
            "qualified_".into(),
            "name_".into()
        ];

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_attrs() {
        let source = ".qualified_.name_";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        let actual = parser.attrs("*some*".into());

        assert_matches!(actual, Ok(Some(_)));
        let actual = actual.unwrap().unwrap();

        let expected: QnName = vec![
            "*some*".into(),
            "qualified_".into(),
            "name_".into()
        ];

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_i32_literal_paren() {
        let mut parser = Parser::new(" (123 )", DEFAULT_CONFIG);
        let actual = parser.i32_literal_paren().unwrap().unwrap();
        assert_eq!(123, actual);
    }

    #[test]
    fn test_character() {
        const EXPECTED_VARCHAR: OptResult<CharacterSystemType> = Ok(Some(CharacterSystemType::Varchar));
        const EXPECTED_BPCHAR: OptResult<CharacterSystemType> = Ok(Some(CharacterSystemType::Bpchar));

        let sources = [
            (EXPECTED_VARCHAR, "varchar"),
            (EXPECTED_VARCHAR, "char varying"),
            (EXPECTED_VARCHAR, "character varying"),
            (EXPECTED_VARCHAR, "nchar varying"),
            (EXPECTED_VARCHAR, "national char varying"),
            (EXPECTED_VARCHAR, "national character varying"),
            (EXPECTED_BPCHAR, "char"),
            (EXPECTED_BPCHAR, "character"),
            (EXPECTED_BPCHAR, "nchar"),
            (EXPECTED_BPCHAR, "national char"),
            (EXPECTED_BPCHAR, "national character"),
        ];

        for (expected, source) in sources {
            let mut parser = Parser::new(source, DEFAULT_CONFIG);
            let actual = parser.character();
            assert_eq!(
                expected,
                actual,
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
            assert_matches!(actual, Ok(Some(_)));
            let actual = actual.unwrap().unwrap();
            assert_eq!(e, actual);
        }
    }

    #[test]
    fn test_unsigned_number() {
        use UnsignedNumber::{IConst, Numeric};

        let mut parser = Parser::new("1.1 11", DEFAULT_CONFIG);

        let actual = parser.unsigned_number();
        assert_matches!(actual, Ok(Some(_)));
        let actual = actual.unwrap().unwrap();
        assert_eq!(Numeric { value: "1.1".into(), radix: 10 }, actual);

        let actual = parser.unsigned_number();
        assert_matches!(actual, Ok(Some(_)));
        let actual = actual.unwrap().unwrap();
        assert_eq!(IConst(11), actual);
    }

    #[test]
    fn test_signed_i32_literal() {
        let mut parser = Parser::new("-123 +321", DEFAULT_CONFIG);
        let actual = parser.signed_i32_literal().unwrap().unwrap();
        assert_eq!(-123, actual);
        let actual = parser.signed_i32_literal().unwrap().unwrap();
        assert_eq!(321, actual);
    }

    #[test]
    fn test_i32_literal() {
        let mut parser = Parser::new("123", DEFAULT_CONFIG);
        let actual = parser.i32_literal().unwrap().unwrap();
        assert_eq!(123, actual);
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
        let actual = parser.role_id().unwrap().unwrap();
        assert_eq!("coalesce", actual);
        let actual = parser.role_id().unwrap().unwrap();
        assert_eq!("xxyyzz", actual);

        let mut parser = Parser::new("none", DEFAULT_CONFIG);
        let actual = parser.role_id().unwrap_err().unwrap();
        assert_eq!(ReservedRoleSpec("none"), actual);

        let mut parser = Parser::new("public", DEFAULT_CONFIG);
        let actual = parser.role_id().unwrap_err().unwrap();
        assert_eq!(ReservedRoleSpec("public"), actual);

        let mut parser = Parser::new("current_role", DEFAULT_CONFIG);
        let actual = parser.role_id().unwrap_err().unwrap();
        assert_eq!(ForbiddenRoleSpec("CURRENT_ROLE"), actual);

        let mut parser = Parser::new("current_user", DEFAULT_CONFIG);
        let actual = parser.role_id().unwrap_err().unwrap();
        assert_eq!(ForbiddenRoleSpec("CURRENT_USER"), actual);

        let mut parser = Parser::new("session_user", DEFAULT_CONFIG);
        let actual = parser.role_id().unwrap_err().unwrap();
        assert_eq!(ForbiddenRoleSpec("SESSION_USER"), actual);
    }

    #[test]
    fn test_role_spec() {
        let source = "public CuRrEnT_rOlE CURRENT_USER session_user coalesce xxyyzz";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.role_spec().unwrap().unwrap();
        assert_eq!(RoleSpec::Public, actual);
        let actual = parser.role_spec().unwrap().unwrap();
        assert_eq!(RoleSpec::CurrentRole, actual);
        let actual = parser.role_spec().unwrap().unwrap();
        assert_eq!(RoleSpec::CurrentUser, actual);
        let actual = parser.role_spec().unwrap().unwrap();
        assert_eq!(RoleSpec::SessionUser, actual);
        let actual = parser.role_spec().unwrap().unwrap();
        assert_eq!(RoleSpec::Name("coalesce".into()), actual);
        let actual = parser.role_spec().unwrap().unwrap();
        assert_eq!(RoleSpec::Name("xxyyzz".into()), actual);

        let mut parser = Parser::new("collate", DEFAULT_CONFIG);
        let actual = parser.role_spec();
        assert_eq!(Ok(None), actual);

        let mut parser = Parser::new("none", DEFAULT_CONFIG);
        let actual = parser.role_spec().unwrap_err().unwrap();
        assert_eq!(ReservedRoleSpec("none"), actual);
    }

    #[test]
    fn test_qual_op() {
        use ast_node::{AllOp, QnOperator};

        let source = "operator(|/) <@>";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.qual_op();
        assert_matches!(actual, Ok(Some(_)));
        let actual = actual.unwrap().unwrap();
        assert_eq!(QnOperator(vec![], AllOp::Operator("|/".into())), actual);

        let actual = parser.qual_op();
        assert_matches!(actual, Ok(Some(_)));
        let actual = actual.unwrap().unwrap();
        assert_eq!(QnOperator(vec![], AllOp::Operator("<@>".into())), actual);
    }

    #[test]
    fn test_prefixed_operator() {
        use ast_node::{AllOp, MathOp, QnOperator};

        let source = "operator(some_qn.*)";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.prefixed_operator();
        assert_matches!(actual, Ok(Some(_)));
        let actual = actual.unwrap().unwrap();

        assert_eq!(QnOperator(vec!["some_qn".into()], AllOp::MathOp(MathOp::Multiplication)), actual);
    }

    #[test]
    fn test_any_operator() {
        use ast_node::{AllOp, MathOp, QnOperator};

        let source = "@@ != qn_name.+";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.any_operator();
        assert_matches!(actual, Ok(Some(_)));
        let actual = actual.unwrap().unwrap();
        assert_eq!(QnOperator(vec![], AllOp::Operator("@@".into())), actual);

        let actual = parser.any_operator();
        assert_matches!(actual, Ok(Some(_)));
        let actual = actual.unwrap().unwrap();
        assert_eq!(QnOperator(vec![], AllOp::MathOp(MathOp::NotEquals)), actual);

        let actual = parser.any_operator();
        assert_matches!(actual, Ok(Some(_)));
        let actual = actual.unwrap().unwrap();
        assert_eq!(QnOperator(vec!["qn_name".into()], AllOp::MathOp(MathOp::Addition)), actual);
    }

    #[test]
    fn test_all_op() {
        use AllOp::*;
        use ast_node::MathOp::NotEquals;

        let source = "~@ <>";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(Some(Operator("~@".into()))), parser.all_op());
        assert_eq!(Ok(Some(MathOp(NotEquals))), parser.all_op());
    }

    #[test]
    fn test_operator() {
        let source = "~@";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(Some("~@".into())), parser.operator());
    }

    #[test]
    fn test_math_op() {
        use MathOp::*;

        let source = "+ - * / % ^ < > = <= >= != <>";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(Some(Addition)), parser.math_op());
        assert_eq!(Ok(Some(Subtraction)), parser.math_op());
        assert_eq!(Ok(Some(Multiplication)), parser.math_op());
        assert_eq!(Ok(Some(Division)), parser.math_op());
        assert_eq!(Ok(Some(Modulo)), parser.math_op());
        assert_eq!(Ok(Some(Exponentiation)), parser.math_op());
        assert_eq!(Ok(Some(Less)), parser.math_op());
        assert_eq!(Ok(Some(Greater)), parser.math_op());
        assert_eq!(Ok(Some(Equals)), parser.math_op());
        assert_eq!(Ok(Some(LessEquals)), parser.math_op());
        assert_eq!(Ok(Some(GreaterEquals)), parser.math_op());
        assert_eq!(Ok(Some(NotEquals)), parser.math_op());
        assert_eq!(Ok(Some(NotEquals)), parser.math_op());
    }

    #[test]
    fn test_col_id() {
        let source = "cascaded xxyyzz coalesce";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.col_id().unwrap().unwrap();
        assert_eq!("cascaded", actual);
        let actual = parser.col_id().unwrap().unwrap();
        assert_eq!("xxyyzz", actual);
        let actual = parser.col_id().unwrap().unwrap();
        assert_eq!("coalesce", actual);
    }

    #[test]
    fn test_type_function_name() {
        let source = "before xxyyzz collation";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.type_function_name().unwrap().unwrap();
        assert_eq!("before", actual);
        let actual = parser.type_function_name().unwrap().unwrap();
        assert_eq!("xxyyzz", actual);
        let actual = parser.type_function_name().unwrap().unwrap();
        assert_eq!("collation", actual);
    }

    #[test]
    fn test_non_reserved_word() {
        let source = "breadth xxyyzz boolean authorization";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.non_reserved_word().unwrap().unwrap();
        assert_eq!("breadth", actual);
        let actual = parser.non_reserved_word().unwrap().unwrap();
        assert_eq!("xxyyzz", actual);
        let actual = parser.non_reserved_word().unwrap().unwrap();
        assert_eq!("boolean", actual);
        let actual = parser.non_reserved_word().unwrap().unwrap();
        assert_eq!("authorization", actual);
    }

    #[test]
    fn test_col_label() {
        let source = "sequence xxyyzz character";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.col_label().unwrap().unwrap();
        assert_eq!("sequence", actual);
        let actual = parser.col_label().unwrap().unwrap();
        assert_eq!("xxyyzz", actual);
        let actual = parser.col_label().unwrap().unwrap();
        assert_eq!("character", actual);
    }

    #[test]
    fn test_bare_col_label() {
        let source = "sequence xxyyzz";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.bare_col_label().unwrap().unwrap();
        assert_eq!("sequence", actual);
        let actual = parser.bare_col_label().unwrap().unwrap();
        assert_eq!("xxyyzz", actual);
    }

    #[test]
    fn test_string() {
        let source = "'test string'";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.string().unwrap().unwrap();

        assert_eq!("test string", actual.as_str());
    }

    #[test]
    fn test_identifier() {
        let source = "tEsT_iDeNtIfIeR";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.identifier().unwrap().unwrap();

        assert_eq!("test_identifier", actual);
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
use self::error::ParserErrorKind::*;
use self::ident_parser::IdentifierParser;
use self::result::OptResult;
use self::result::OptionalResult;
use self::result::ReqResult;
use self::result::RequiredResult;
use self::string_parser::StringParser;
use self::token_buffer::TokenBuffer;
use self::token_buffer::TokenConsumer;
use crate::lexer::Keyword;
use crate::lexer::KeywordDetails;
use crate::lexer::Lexer;
use crate::lexer::TokenKind::{self, Comma, Dot, NumberLiteral};
use crate::parser::ast_node::{AllOp, MathOp, QnOperator, SignedNumber, UnsignedNumber};
use postgres_basics::ascii;
use postgres_basics::Located;
use postgres_basics::Location;
use std::borrow::Cow;
use std::mem;
