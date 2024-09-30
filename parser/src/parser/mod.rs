mod ast_node;
mod ident_parser;
mod config;
mod error;
mod string_parser;
mod token_buffer;
mod result;
mod parse_report;
mod warning;
mod stmt_parsers;
mod bit_string_parser;

pub use self::{
    ast_node::{
        AlterRoleAction,
        AlterRoleOption,
        AlterRoleStmt,
        AstLiteral,
        AstNode,
        ClosePortalStmt,
        DeallocateStmt,
        DiscardStmt,
        EventTriggerState,
        IsolationLevel,
        NumericSpec,
        ReassignOwnedStmt,
        RenameStmt,
        RoleSpec,
        SystemType,
        TransactionMode,
        UnlistenStmt,
        VariableShowStmt,
    },
    config::ParserConfig,
    error::ParserErrorKind,
    parse_report::ParseReport,
    result::{OptResult, ReqResult},
    warning::ParserWarning,
};

macro_rules! list_production {
    (gather $production:block delim $separator:block) => {
        (|| {
            let mut elements = match $production? {
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
    }
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

    pub fn new(source: &'src [u8], config: ParserConfig) -> Self {
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
        use UnreservedKeyword::Checkpoint;

        if self.buffer.consume_kw_eq(Unreserved(Checkpoint))?.is_some() {
            return Ok(Some(AstNode::CheckPoint))
        }

        if let Some(node) = self.abort_stmt()? { Ok(Some(node.into())) }
        else if let Some(node) = self.alter_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.analyze_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.call_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.close_stmt()? { Ok(Some(node.into())) }
        else if let Some(node) = self.cluster_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.comment_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.commit_stmt()? { Ok(Some(node.into())) }
        else if let Some(node) = self.copy_stmt()? { Ok(Some(node)) }
        else if let Some(node) = self.deallocate_stmt()? { Ok(Some(node.into())) }
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
        else if let Some(node) = self.reassign_owner_stmt()? { Ok(Some(node.into())) }
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
        else if let Some(node) = self.unlisten_stmt()? { Ok(Some(node.into())) }
        else if let Some(node) = self.vacuum_stmt()? { Ok(Some(node)) }
        else { Ok(None) }
    }

    fn opt_transaction(&mut self) -> ReqResult<()> {
        use UnreservedKeyword::{Transaction, Work};

        // Skips over WORK | TRANSACTION
        self.buffer.consume(|tok|
            matches!(tok.keyword().and_then(KeywordDetails::unreserved), Some(Work | Transaction))
        ).replace_eof(Ok(None))?;

        Ok(())
    }

    fn opt_transaction_chain(&mut self) -> ReqResult<bool> {
        use ReservedKeyword::And;
        use UnreservedKeyword::{Chain, No};

        if self.buffer.consume_kw_eq(Reserved(And)).replace_eof(Ok(None))?.is_none() {
            return Ok(false)
        }

        let result = self.buffer.consume_kw_eq(Unreserved(No)).replace_eof(Ok(None))?.is_none();

        self.buffer.consume_kw_eq(Unreserved(Chain)).required()?;

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

    fn transaction_mode(&mut self) -> OptResult<TransactionMode> {
        use ReservedKeyword::{Deferrable, Not, Only};
        use UnreservedKeyword::{Isolation, Level, Read, Write};

        let result = self.buffer.consume(|tok|
            match tok.keyword().map(KeywordDetails::keyword) {
                kw @ Some(Reserved(Deferrable | Not) | Unreserved(Isolation | Read)) => kw,
                _ => None
            }
        )?;

        let Some(result) = result else { return Ok(None) };

        match result {
            Reserved(Deferrable) => Ok(Some(TransactionMode::Deferrable)),
            Reserved(Not) => {
                self.buffer.consume_kw_eq(Reserved(Deferrable)).required()?;
                Ok(Some(TransactionMode::NotDeferrable))
            },
            Unreserved(Isolation) => {
                self.buffer.consume_kw_eq(Unreserved(Level)).required()?;
                let isolation_level = self.isolation_level()?;
                Ok(Some(TransactionMode::IsolationLevel(isolation_level)))
            },
            Unreserved(Read) => {
                let result = self.buffer.consume(|tok|
                    match tok.keyword().map(KeywordDetails::keyword) {
                        kw @ Some(Reserved(Only) | Unreserved(Write)) => kw,
                        _ => None
                    }
                ).required()?;

                match result {
                    Reserved(Only) => Ok(Some(TransactionMode::ReadOnly)),
                    Unreserved(Write) => Ok(Some(TransactionMode::ReadWrite)),
                    _ => unreachable!(),
                }
            },
            _ => Ok(None)
        }
    }

    /// Alias: `iso_level`
    fn isolation_level(&mut self) -> ReqResult<IsolationLevel> {
        use UnreservedKeyword::{Committed, Read, Repeatable, Serializable, Uncommitted};

        let result = self.buffer.consume(|tok|
            tok.keyword().and_then(KeywordDetails::unreserved)
                .filter(|kw| matches!(kw, Read | Repeatable | Serializable))
        ).required()?;

        match result {
            Serializable => Ok(IsolationLevel::Serializable),
            Repeatable => {
                self.buffer.consume_kw_eq(Unreserved(Read)).required()?;
                Ok(IsolationLevel::RepeatableRead)
            },
            Read => {
                let result = self.buffer.consume(|tok|
                    tok.keyword().and_then(KeywordDetails::unreserved)
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

    fn var_name(&mut self) -> ReqResult<Vec<Cow<'static, str>>> {

        list_production!(
            gather { self.col_id() }
            delim { self.buffer.consume_eq(TokenKind::Dot) }
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

        let bit = self.buffer.consume_kw_eq(ColumnName(ColumnNameKeyword::Bit))?;
        if bit.is_some() {
            let varying = self.buffer.consume_kw_eq(Unreserved(UnreservedKeyword::Varying))
                .replace_eof(Ok(None))?;
            // TODO: self.expr_list()
        }

        todo!()
    }

    /// Alias: `Numeric`<p/>
    /// Inline: `opt_float`
    fn numeric(&mut self) -> OptResult<SystemType> {
        use ColumnNameKeyword::{Bigint, Boolean, Dec, Decimal, Float, Int, Integer, Numeric, Precision, Real, Smallint};
        use UnreservedKeyword::Double;

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
                    ColumnName(
                          Int
                        | Integer
                        | Smallint
                        | Bigint
                        | Real
                        | Float
                        | Decimal
                        | Dec
                        | Numeric
                        | Boolean
                    )
                    | Unreserved(Double)
                )
            )
        )?;

        let kw = match kw {
            None => return Ok(None),
            Some(ColumnName(Smallint)) => return Ok(Some(Int2)),
            Some(ColumnName(Int | Integer)) => return Ok(Some(Int4)),
            Some(ColumnName(Bigint)) => return Ok(Some(Int8)),
            Some(ColumnName(Real)) => return Ok(Some(Float4)),
            Some(ColumnName(Boolean)) => return Ok(Some(Bool)),
            Some(kw) => kw,
        };

        if kw == Unreserved(Double) {
            self.buffer.consume_kw_eq(ColumnName(Precision)).required()?;
            return Ok(Some(Float8))
        }

        if kw == ColumnName(Float) {
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

    fn a_expr(&mut self) -> OptResult<AstNode> {
        use TokenKind::Plus;

        // TODO
        self.buffer.consume_eq(Plus)?;
        Ok(Some(Literal(NullLiteral)))
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
        use ColumnNameKeyword::{Char, Character, National, Nchar, Varchar};

        /*
        character :
            VARCHAR
          | (CHARACTER | CHAR_P | NCHAR) (VARYING)?
          | NATIONAL ( CHARACTER | CHAR_P) (VARYING)?
        */

        let char_type = self.buffer.consume(|tok|
            tok.keyword()
                .and_then(KeywordDetails::col_name)
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
                        tok.keyword().and_then(KeywordDetails::col_name),
                        Some(Character | Char)
                    )
                )
                .required()?;
        }

        let varying = self.buffer.consume_kw_eq(Unreserved(UnreservedKeyword::Varying))
            .replace_eof(Ok(None))?;

        let char_type = if varying.is_some() {
            CharacterSystemType::Varchar
        }
        else {
            CharacterSystemType::Bpchar
        };

        Ok(Some(char_type))
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
        use TokenKind::NumberLiteral;

        let loc = self.buffer.current_location();
        let source = self.buffer.source();

        self.buffer.consume(|tok| {

            let NumberLiteral { radix } = tok else { return None };
            let radix = *radix;

            loc.slice(source)
                .iter()
                .map(|d| (*d - b'0') as i32)
                .try_fold(
                    0i32,
                    |acc, n| acc.checked_mul(radix)?.checked_add(n)
                )
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
    fn role_id(&mut self) -> OptResult<Cow<'static, str>> {

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
            use ColumnNameKeyword::NoneKw;
            use ReservedKeyword::{CurrentRole, CurrentUser, SessionUser};

            let Some(kw) = tok.keyword() else {
                return Ok(None)
            };

            match kw.keyword() {
                ColumnName(NoneKw) => Err(ReservedRoleSpec("none")),
                Reserved(CurrentRole) => Ok(Some(RoleSpec::CurrentRole)),
                Reserved(CurrentUser) => Ok(Some(RoleSpec::CurrentUser)),
                Reserved(SessionUser) => Ok(Some(RoleSpec::SessionUser)),
                Reserved(_) => Ok(None),
                _ => Ok(Some(
                    RoleSpec::Name(kw.text().into())
                )),
            }
        })
    }

    /// Alias: `ColId`
    #[inline(always)]
    fn col_id(&mut self) -> OptResult<Cow<'static, str>> {
        self.ident_or_keyword(|kw|
               kw.unreserved().is_some()
            || kw.col_name().is_some()
        )
    }

    #[inline(always)]
    fn type_function_name(&mut self) -> OptResult<Cow<'static, str>> {
        self.ident_or_keyword(|kw|
               kw.unreserved().is_some()
            || kw.type_func_name().is_some()
        )
    }

    /// Alias: `NonReservedWord`
    #[inline(always)]
    fn non_reserved_word(&mut self) -> OptResult<Cow<'static, str>> {
        self.ident_or_keyword(|kw|
               kw.unreserved().is_some()
            || kw.col_name().is_some()
            || kw.type_func_name().is_some()
        )
    }

    /// Alias: `ColLabel`
    #[inline(always)]
    fn col_label(&mut self) -> OptResult<Cow<'static, str>> {
        self.ident_or_keyword(|_| true)
    }

    /// Alias: `BareColLabel`
    #[inline(always)]
    fn bare_col_label(&mut self) -> OptResult<Cow<'static, str>> {
        self.ident_or_keyword(KeywordDetails::bare)
    }

    fn ident_or_keyword<P>(&mut self, pred: P) -> OptResult<Cow<'static, str>>
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
    /// * `SCONST` as `StringLiteral`
    /// * `USCONST` as `StringLiteral`
    #[inline(always)]
    fn string(&mut self) -> OptResult<String> {
        StringParser(self).parse()
    }

    /// Aliases:
    /// * `BCONST` as `BitStringLiteral`
    /// * `XCONST` as `BitStringLiteral`
    #[inline(always)]
    fn bit_string(&mut self) -> OptResult<BitBox> {
        BitStringParser(self).parse()
    }

    /// Alias: `IDENT`
    #[inline(always)]
    fn identifier(&mut self) -> OptResult<String> {
        IdentifierParser(self).parse()
    }

    /// Production: `UESCAPE SCONST`
    fn uescape(&mut self) -> Result<u8, ParserErrorKind> {
        use UnreservedKeyword::Uescape;

        // Try to consume UESCAPE + the string following it.
        // see [base_yylex](https://github.com/postgres/postgres/blob/1c61fd8b527954f0ec522e5e60a11ce82628b681/src/backend/parser/parser.c#L256)

        let uescape = self.buffer.consume_kw_eq(Unreserved(Uescape));

        match uescape {
            Ok(None) | Err(None) => return Ok(b'\\'),
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
fn uescape_escape(source: &[u8]) -> Option<u8> {

    if source.len() != 3
        || source[0] != b'\''
        || source[2] != b'\''
    {
        return None
    }

    let escape = source[1];

    if is_hex_digit(escape)
        || is_whitespace(escape)
        || escape == b'+'
        || escape == b'\''
        || escape == b'"'
    {
        return None
    }

    Some(escape)
}

#[cfg(test)]
mod tests {
    use super::SystemType::{Bool, Float4, Float8, Int2, Int4, Int8};
    use super::*;
    use postgres_basics::guc::BackslashQuote;

    pub(in crate::parser) const DEFAULT_CONFIG: ParserConfig = ParserConfig::new(true, BackslashQuote::SafeEncoding);

    #[test]
    fn test_toplevel_stmt() {
        let sources = [
            // TODO: begin
            "start transaction", // stmt
            "end transaction",
        ];

        for source in sources {
            let mut parser = Parser::new(source.as_bytes(), DEFAULT_CONFIG);
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
            //       prepare, reindex, revoke, security, set, truncate, vacuum
            "abort transaction",
            "alter group some_group add user public",
            "close all",
            "commit and no chain",
            "deallocate all",
            "discard all",
            "listen ident",
            "load 'test string'",
            "notify test_ident, 'test-payload'",
            "reassign owned by public, test_role to target_role",
            "release savepoint test_ident",
            "rollback to test_ident",
            "savepoint test_ident",
            "show all",
            "start transaction read only, read write deferrable",
            "unlisten *",
        ];

        for source in sources {
            let mut parser = Parser::new(source.as_bytes(), DEFAULT_CONFIG);
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
        let mut parser = Parser::new(b"transaction work", DEFAULT_CONFIG);
        assert_eq!(Ok(()), parser.opt_transaction());
        assert_eq!(Ok(()), parser.opt_transaction());
    }

    #[test]
    fn test_opt_transaction_chain() {
        let mut parser = Parser::new(b"", DEFAULT_CONFIG);
        assert_eq!(Ok(false), parser.opt_transaction_chain());

        let mut parser = Parser::new(b"and no chain", DEFAULT_CONFIG);
        assert_eq!(Ok(false), parser.opt_transaction_chain());

        let mut parser = Parser::new(b"and chain", DEFAULT_CONFIG);
        assert_eq!(Ok(true), parser.opt_transaction_chain());
    }

    #[test]
    fn test_opt_transaction_mode_list() {
        let mut parser = Parser::new(b"no_match", DEFAULT_CONFIG);
        assert_eq!(Ok(None), parser.opt_transaction_mode_list());

        let mut parser = Parser::new(
            b"read only , read write isolation level read committed",
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
            b"\
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
            b"\
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
        let mut parser = Parser::new(b"test.qualified.name", DEFAULT_CONFIG);
        let expected = vec![
            "test".into(),
            "qualified".into(),
            "name".into()
        ];

        assert_eq!(Ok(expected), parser.var_name());
    }

    #[test]
    fn test_numeric() {

        let source = b"boolean smallint int integer bigint real float float(17) float(44) double precision";
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
    fn test_i32_literal_paren() {
        let mut parser = Parser::new(b" (123 )", DEFAULT_CONFIG);
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
            let mut parser = Parser::new(source.as_bytes(), DEFAULT_CONFIG);
            let actual = parser.character();
            assert_eq!(
                expected,
                actual,
                r"expected {expected:?} for source {source:?} but actually got {actual:?}",
            );
        }
    }

    #[test]
    fn test_signed_i32_literal() {
        let mut parser = Parser::new(b"-123 +321", DEFAULT_CONFIG);
        let actual = parser.signed_i32_literal().unwrap().unwrap();
        assert_eq!(-123, actual);
        let actual = parser.signed_i32_literal().unwrap().unwrap();
        assert_eq!(321, actual);
    }

    #[test]
    fn test_i32_literal() {
        let mut parser = Parser::new(b"123", DEFAULT_CONFIG);
        let actual = parser.i32_literal().unwrap().unwrap();
        assert_eq!(123, actual);
    }

    #[test]
    fn test_role_list() {
        let source = b"public , CuRrEnT_rOlE,CURRENT_USER, session_user ,coalesce,xxYYzz none";
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

        let mut parser = Parser::new(b"coalesce xxyyzz", DEFAULT_CONFIG);
        let actual = parser.role_id().unwrap().unwrap();
        assert_eq!("coalesce", actual);
        let actual = parser.role_id().unwrap().unwrap();
        assert_eq!("xxyyzz", actual);

        let mut parser = Parser::new(b"none", DEFAULT_CONFIG);
        let actual = parser.role_id().unwrap_err().unwrap();
        assert_eq!(ReservedRoleSpec("none"), actual);

        let mut parser = Parser::new(b"public", DEFAULT_CONFIG);
        let actual = parser.role_id().unwrap_err().unwrap();
        assert_eq!(ReservedRoleSpec("public"), actual);

        let mut parser = Parser::new(b"current_role", DEFAULT_CONFIG);
        let actual = parser.role_id().unwrap_err().unwrap();
        assert_eq!(ForbiddenRoleSpec("CURRENT_ROLE"), actual);

        let mut parser = Parser::new(b"current_user", DEFAULT_CONFIG);
        let actual = parser.role_id().unwrap_err().unwrap();
        assert_eq!(ForbiddenRoleSpec("CURRENT_USER"), actual);

        let mut parser = Parser::new(b"session_user", DEFAULT_CONFIG);
        let actual = parser.role_id().unwrap_err().unwrap();
        assert_eq!(ForbiddenRoleSpec("SESSION_USER"), actual);
    }

    #[test]
    fn test_role_spec() {
        let source = b"public CuRrEnT_rOlE CURRENT_USER session_user coalesce xxyyzz";
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

        let mut parser = Parser::new(b"collate", DEFAULT_CONFIG);
        let actual = parser.role_spec();
        assert_eq!(Ok(None), actual);

        let mut parser = Parser::new(b"none", DEFAULT_CONFIG);
        let actual = parser.role_spec().unwrap_err().unwrap();
        assert_eq!(ReservedRoleSpec("none"), actual);
    }

    #[test]
    fn test_col_id() {
        let source = b"cascaded xxyyzz coalesce";
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
        let source = b"before xxyyzz collation";
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
        let source = b"breadth xxyyzz boolean authorization";
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
        let source = b"sequence xxyyzz character";
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
        let source = b"sequence xxyyzz";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.bare_col_label().unwrap().unwrap();
        assert_eq!("sequence", actual);
        let actual = parser.bare_col_label().unwrap().unwrap();
        assert_eq!("xxyyzz", actual);
    }

    #[test]
    fn test_string() {
        let source = b"'test string'";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.string().unwrap().unwrap();

        assert_eq!("test string", actual.as_str());
    }

    #[test]
    fn test_identifier() {
        let source = b"tEsT_iDeNtIfIeR";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.identifier().unwrap().unwrap();

        assert_eq!("test_identifier", actual);
    }

    #[test]
    fn test_uescape() {
        let source = b"UESCAPE '!'";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.uescape().unwrap();

        assert_eq!(b'!', actual);
    }
}

use self::{
    ast_node::CharacterSystemType,
    bit_string_parser::BitStringParser,
    error::ParserErrorKind::*,
    ident_parser::IdentifierParser,
    result::{OptionalResult, RequiredResult},
    string_parser::StringParser,
    token_buffer::{TokenBuffer, TokenConsumer},
    AstLiteral::NullLiteral,
    AstNode::{ListenStmt, Literal, LoadStmt},
    SystemType::{Bool, Float4, Float8, Int2, Int4, Int8},
};
use crate::lexer::Keyword::{ColumnName, Reserved, Unreserved};
use crate::lexer::UnreservedKeyword::{Read, Repeatable, Serializable};
use crate::lexer::{
    ColumnNameKeyword,
    KeywordDetails,
    Lexer,
    ReservedKeyword,
    TokenKind,
    UnreservedKeyword
};
use bitvec::boxed::BitBox;
use postgres_basics::ascii::{is_hex_digit, is_whitespace};
use postgres_basics::{Located, Location};
use std::borrow::Cow;
use std::mem;
use TokenKind::{Comma, Semicolon};
