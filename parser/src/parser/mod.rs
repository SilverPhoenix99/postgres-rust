mod ast_node;
mod ident_parser;
mod config;
mod error;
mod string_parser;
mod token_buffer;
mod result;
mod parse_report;
mod warning;

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
    error::ParserErrorKind,
    parse_report::ParseReport,
    result::{OptResult, ReqResult},
    warning::ParserWarning,
};

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

pub struct ParserResult {
    pub result: Result<Vec<AstNode>, Located<ParserErrorKind>>,
    pub warnings: Vec<Located<ParserWarning>>,
}

pub struct Parser<'src> {
    buffer: TokenBuffer<'src>,
    config: ParserConfig,
    warnings: Vec<Located<ParserWarning>>,
}

impl<'src> Parser<'src> {

    pub fn new(source: &'src [u8], config: ParserConfig) -> Self {
        let lexer = Lexer::new(source, config.standard_conforming_strings());
        Self {
            buffer: TokenBuffer::new(lexer),
            config,
            warnings: Vec::new(),
        }
    }

    pub fn with_lexer(lexer: Lexer<'src>, config: ParserConfig) -> Self {
        Self {
            buffer: TokenBuffer::new(lexer),
            config,
            warnings: Vec::new(),
        }
    }

    /// Not reentrant!
    pub fn parse(&mut self) -> ParserResult {

        // match mode {
        //     ParseMode::TypeName => {}
        //     ParseMode::PlpgsqlExpr => {}
        //     ParseMode::PlpgsqlAssign1 => {}
        //     ParseMode::PlpgsqlAssign2 => {}
        //     ParseMode::PlpgsqlAssign3 => {}
        //     ParseMode::Default => {} // if no match
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

    /// Alias: `ConstTypename`
    fn const_typename(&mut self) -> OptResult<NumericType> {

        /*
        ConstTypename :
            numeric // Numeric
          | character ( '(' ICONST ')' )? // ConstCharacter
          | BIT (VARYING)? ( '(' expr_list ')' )? // ConstBit
          | TIMESTAMP ( '(' ICONST ')' )? ( (WITH_LA | WITHOUT_LA) TIME ZONE )? // ConstDatetime
          | TIME ( '(' ICONST ')' )? ( (WITH_LA | WITHOUT_LA) TIME ZONE )?      // ConstDatetime
          | JSON
        */

        todo!()
    }

    /// Alias: `Numeric`<p/>
    /// Inline: `opt_float`
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
            // opt_float: '(' ICONST ')'
            return match self.i32_literal_paren().replace_eof(Ok(None))? {
                None => Ok(Some(Float8)),
                Some(num @ ..=0) => Err(Some(FloatPrecisionUnderflow(num))),
                Some(1..=24) => Ok(Some(Float4)),
                Some(25..=53) => Ok(Some(Float8)),
                Some(num @ 54..) => Err(Some(FloatPrecisionOverflow(num))),
            }
        }

        let type_mods = self.opt_type_modifiers()?;
        Ok(Some(NumericType::Numeric(type_mods)))
    }

    fn opt_type_modifiers(&mut self) -> OptResult<Vec<AstNode>> {

        // '(' expr_list ')'

        if self.buffer.consume_eq(OpenParenthesis)?.is_none() {
            return Ok(None)
        }

        let exprs = self.expr_list()?;

        self.buffer.consume_eq(OpenParenthesis).required()?;

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

        todo!()
    }

    /// Production: `'(' ICONST ')'`
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

        let Some(char_type) = char_type else {
            return Ok(None)
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
    fn uescape(&mut self) -> Result<u8, ParserErrorKind> {

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
    use super::*;
    use crate::parser::AstNode::StringLiteral;
    use postgres_basics::guc::BackslashQuote;

    const DEFAULT_CONFIG: ParserConfig = ParserConfig::new(true, BackslashQuote::SafeEncoding, ParseMode::Default);

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

        assert_eq!(StringLiteral("test string".into()), actual);
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
    ast_node::{
        CharacterType::Bpchar,
        NumericType::*,
    },
    error::ParserErrorKind::*,
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
use postgres_basics::{
    ascii::{is_hex_digit, is_whitespace},
    Located,
};
use std::borrow::Cow;
