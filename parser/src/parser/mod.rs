mod ast_node;
mod ident_parser;
mod config;
mod error;
mod string_parser;
mod token_buffer;
mod result;

pub use self::{
    ast_node::{
        AstNode,
        CharacterType,
        NumericSpec,
        NumericType,
        RoleSpec,
        SystemType,
    },
    config::{ParseMode, ParserConfig},
    error::ParserError,
    result::{OptResult, ReqResult},
};
use self::{
    ast_node::{
        CharacterType::Bpchar,
        NumericType::*,
    },
    error::ParserError::*,
    ident_parser::IdentifierParser,
    result::{OptionalResult, RequiredResult},
    string_parser::{StringParser, StringParserResult},
    token_buffer::{TokenBuffer, TokenConsumer}
};
use crate::lexer::{
    ColumnNameKeyword::{
        Bigint,
        Boolean,
        Char,
        Character,
        Dec,
        Decimal,
        Float,
        Int,
        Integer,
        National,
        Nchar,
        NoneKw,
        Numeric,
        Precision,
        Real,
        Smallint,
        Varchar,
    },
    Keyword::{ColumnName, Reserved, Unreserved},
    KeywordDetails,
    Lexer,
    ReservedKeyword::{CurrentRole, CurrentUser, SessionUser},
    TokenKind::{CloseParenthesis, Comma, Minus, NumberLiteral, OpenParenthesis, Plus},
    UnreservedKeyword,
    UnreservedKeyword::{Double, Uescape},
};
use crate::string_decoders::ExtendedStringWarning;
use postgres_basics::{
    ascii::{is_hex_digit, is_whitespace},
    sql_state::{SqlState, WarningSqlState},
    Located,
};
use std::borrow::Cow;

macro_rules! list_production {
    (gather { $production:expr } delim { $separator:expr }) => {
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

pub enum ParserWarning {
    DeprecatedGlobalTemporaryTable,
    NonstandardEscape(ExtendedStringWarning),
}

impl ParserWarning {

    pub fn sqlstate(self) -> SqlState {
        match self {
            Self::DeprecatedGlobalTemporaryTable => SqlState::Warning(WarningSqlState::Warning),
            Self::NonstandardEscape(warn) => warn.sqlstate(),
        }
    }

    pub fn message(self) -> &'static str {
        match self {
            Self::DeprecatedGlobalTemporaryTable => "GLOBAL is deprecated in temporary table creation",
            Self::NonstandardEscape(warn) => warn.message()
        }
    }

    pub fn hint(self) -> Option<&'static str> {
        match self {
            Self::DeprecatedGlobalTemporaryTable => None,
            Self::NonstandardEscape(warn) => Some(warn.hint()),
        }
    }
}

pub struct ParserResult {
    pub result: Result<Vec<AstNode>, Located<ParserError>>,
    pub warnings: Vec<Located<ParserWarning>>,
}

pub struct Parser<'src> {
    buffer: TokenBuffer<'src>,
    source: &'src [u8],
    config: ParserConfig,
    warnings: Vec<Located<ParserWarning>>,
}

impl<'src> Parser<'src> {

    pub fn new(source: &'src [u8], config: ParserConfig) -> Self {
        let lexer = Lexer::new(source, config.standard_conforming_strings);
        Self {
            buffer: TokenBuffer::new(lexer),
            source,
            config,
            warnings: Vec::new(),
        }
    }

    /// Not reentrant!
    pub fn parse(&mut self, mode: ParseMode) -> ParserResult {

        // match mode {
        //     ParseMode::TypeName => {todo!()}
        //     ParseMode::PlpgsqlExpr => {todo!()}
        //     ParseMode::PlpgsqlAssign1 => {todo!()}
        //     ParseMode::PlpgsqlAssign2 => {todo!()}
        //     ParseMode::PlpgsqlAssign3 => {todo!()}
        //     ParseMode::Default => {todo!()} // if no match
        // }

        todo!()
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

    /// Alias: `Numeric`
    fn numeric(&mut self) -> OptResult<NumericType> {

        /*
        Numeric :
            INT_P
          | INTEGER
          | SMALLINT
          | BIGINT
          | REAL
          | FLOAT_P ( '(' ICONST ')' )?
          | DOUBLE_P PRECISION
          | DECIMAL_P opt_type_modifiers
          | DEC opt_type_modifiers
          | NUMERIC opt_type_modifiers
          | BOOLEAN_P
        */

        let kw = self.buffer.consume(|tok| {
            match tok.keyword().map(KeywordDetails::keyword)? {
                kw @ (
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
                ) => Some(kw),
                _ => None,
            }
        })?;

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
            self.buffer.consume(|tok|
                matches!(
                    tok.keyword().map(KeywordDetails::keyword),
                    Some(ColumnName(Precision))
                )
            ).required()?;

            return Ok(Some(Float8))
        }

        if kw == ColumnName(Float) {
            return match self.i32_literal_paren().replace_eof(Ok(None))? {
                None => Ok(Some(Float8)),
                Some(num) => {
                    match num {
                        ..=0 => Err(Some(FloatPrecisionUnderflow(num))),
                        1..=24 => Ok(Some(Float4)),
                        25..=53 => Ok(Some(Float8)),
                        54.. => Err(Some(FloatPrecisionOverflow(num))),
                    }
                }
            }
        }

        todo!("(DEC | DECIMAL | NUMERIC) opt_type_modifiers")
    }

    /// Production: '(' ICONST ')'
    fn i32_literal_paren(&mut self) -> OptResult<i32> {

        if self.buffer.consume_eq(OpenParenthesis)?.is_none() {
            return Ok(None)
        }

        let num = self.i32_literal().required()?;

        self.buffer.consume_eq(CloseParenthesis).required()?;

        Ok(Some(num))
    }

    fn character(&mut self) -> OptResult<CharacterType> {

        /*
        character :
            CHARACTER (VARYING)?
          | CHAR_P (VARYING)?
          | VARCHAR
          | NATIONAL CHARACTER (VARYING)?
          | NATIONAL CHAR_P (VARYING)?
          | NCHAR (VARYING)?
        */

        let char_type = self.buffer.consume(|tok| {

            tok.keyword().and_then(KeywordDetails::col_name)
                .filter(|col_name|
                    matches!(col_name, Varchar | National | Nchar | Character | Char)
                )
        })?;

        let char_type = match char_type {
            None => return Ok(None),
            Some(char_type) => char_type,
        };

        if matches!(char_type, Varchar) {
            return Ok(Some(CharacterType::Varchar(None)))
        }

        if matches!(char_type, National | Nchar) {

            self.buffer
                .consume(|tok| {
                    matches!(
                        tok.keyword().and_then(KeywordDetails::col_name),
                        Some(Character | Char)
                    )
                })
                .required()
                .optional()?;
        }

        let varying = self.buffer.consume(|tok| {
            matches!(
                tok.keyword().and_then(KeywordDetails::unreserved),
                Some(UnreservedKeyword::Varying)
            )
        })?;

        let char_type = if varying.is_some() {
            CharacterType::Varchar(None)
        }
        else {
            Bpchar(None)
        };

        Ok(Some(char_type))
    }

    /// Alias: `SignedIconst`
    fn signed_i32_literal(&mut self) -> OptResult<i32> {

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

        self.buffer.consume(|tok| {

            if let NumberLiteral { radix } = tok {
                let radix = *radix;
                let slice = loc.slice(self.source);
                return slice.iter()
                    .map(|d| (*d - b'0') as i32)
                    .try_fold(
                        0i32,
                        |acc, n| acc.checked_mul(radix)?.checked_add(n)
                    )
            }

            None
        })
    }

    fn role_list(&mut self) -> ReqResult<Vec<RoleSpec>> {

        list_production!(
            gather { self.role_spec() }
            delim  { self.buffer.consume_eq(Comma) }
        ).required()
    }

    /// Alias: `RoleId`
    fn role_id(&mut self) -> OptResult<Cow<'static, str>> {

        // Similar to role_spec, but only allows an identifier, i.e., disallows builtin roles

        match self.role_spec()? {
            None => Ok(None),
            Some(RoleSpec::Name(role)) => Ok(Some(role)),
            Some(RoleSpec::Public) => Err(Some(ReservedRoleSpec("public"))),
            Some(RoleSpec::CurrentRole) => Err(Some(ForbiddenRoleSpec("CURRENT_ROLE"))),
            Some(RoleSpec::CurrentUser) => Err(Some(ForbiddenRoleSpec("CURRENT_USER"))),
            Some(RoleSpec::SessionUser) => Err(Some(ForbiddenRoleSpec("SESSION_USER"))),
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

            let kw = match tok.keyword() {
                Some(kw) => kw,
                None => return Ok(None),
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
    /// * `BCONST` as `BitStringLiteral`
    /// * `XCONST` as `BitStringLiteral`
    fn string(&mut self) -> OptResult<AstNode> {

        let StringParserResult { result, warning } = StringParser(self).parse();

        if let Some((warning, loc)) = warning {
            let warning = ParserWarning::NonstandardEscape(warning);
            self.warnings.push((warning, loc));
        }

        result
    }

    /// Alias: `IDENT`
    #[inline(always)]
    fn identifier(&mut self) -> OptResult<String> {
        IdentifierParser(self).parse()
    }

    /// Production: `UESCAPE SCONST`
    fn uescape(&mut self) -> Result<u8, ParserError> {

        // Try to consume UESCAPE + the string following it.
        // see [base_yylex](https://github.com/postgres/postgres/blob/1c61fd8b527954f0ec522e5e60a11ce82628b681/src/backend/parser/parser.c#L256)

        let uescape = self.buffer.consume(|tok|
            matches!(tok.keyword().and_then(KeywordDetails::unreserved), Some(Uescape))
        );

        match uescape {
            Ok(None) | Err(None) => return Ok(b'\\'),
            Err(Some(err)) => return Err(err),
            Ok(Some(_)) => {/* it matched */}
        }

        let loc = self.buffer.current_location();

        let escape = self.buffer
            .consume(|tok| match tok.string_kind() {
                Some(_) => {
                    let slice = loc.slice(self.source);
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
