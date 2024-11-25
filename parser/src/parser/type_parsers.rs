#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum TypeNameKind {
    Simple,
    Const
}

impl Parser<'_> {

    /// Alias: `Typename`
    pub(in crate::parser) fn type_name(&mut self) -> ScanResult<SystemType> {

        /*
            ( SETOF )? SimpleTypename opt_array_bounds
        */

        let setof = Setof.maybe_match().parse(&mut self.buffer)?.is_some();

        let typ = self.simple_typename();

        let typ = if setof {
            // this means the `SETOF` keyword was present, so the production has already started
            typ.required()?
        }
        else {
            typ?
        };

        let array_bounds = opt_array_bounds()
            .optional()
            .map(|bounds| bounds.unwrap_or_default())
            .parse(&mut self.buffer)?;

        let mut typ = typ.with_array_bounds(array_bounds);
        if setof {
            typ = typ.returning_table();
        }

        Ok(typ)
    }

    /// Alias: `SimpleTypename`
    pub(in crate::parser) fn simple_typename(&mut self) -> ScanResult<TypeName> {

        /*
              JSON
            | BOOLEAN
            | SMALLINT
            | INT | INTEGER
            | BIGINT
            | REAL
            | DOUBLE PRECISION
            | DECIMAL opt_type_modifiers
            | DEC opt_type_modifiers
            | NUMERIC opt_type_modifiers
            | FLOAT opt_paren_i32
            | BIT opt_varying opt_type_modifiers
            | VARCHAR opt_paren_i32
            | (CHAR | CHARACTER | NCHAR) opt_varying opt_paren_i32
            | NATIONAL (CHAR | CHARACTER) opt_varying opt_paren_i32
            | TIMESTAMP opt_paren_i32 opt_timezone
            | TIME opt_paren_i32 opt_timezone
            | INTERVAL ( '(' ICONST ')' | opt_interval )?
            | (IDENT | unreserved_keyword) ( attrs )? opt_type_modifiers
            | type_func_name_keyword ( attrs )? opt_type_modifiers
        */

        self.parse_type(Simple)
    }

    /// Alias: `ConstTypename`
    pub(in crate::parser) fn const_typename(&mut self) -> ScanResult<TypeName> {

        /*
              JSON
            | BOOLEAN
            | SMALLINT
            | INT | INTEGER
            | BIGINT
            | REAL
            | DOUBLE PRECISION
            | DECIMAL opt_type_modifiers
            | DEC opt_type_modifiers
            | NUMERIC opt_type_modifiers
            | FLOAT opt_paren_i32
            | BIT opt_varying opt_type_modifiers
            | VARCHAR opt_paren_i32
            | (CHAR | CHARACTER | NCHAR) opt_varying opt_paren_i32
            | NATIONAL (CHAR | CHARACTER) opt_varying opt_paren_i32
            | TIMESTAMP opt_paren_i32 opt_timezone
            | TIME opt_paren_i32 opt_timezone
            | INTERVAL ( '(' ICONST ')' )?        ## NB: `opt_interval()` is checked in `a_expr_const()` instead of here
            | (IDENT | unreserved_keyword) ( attrs )? opt_type_modifiers
            | type_func_name_keyword opt_type_modifiers
            | col_name_keyword attrs opt_type_modifiers
        */

        self.parse_type(Const)
    }

    /// Inline:
    /// * `SimpleTypename` and `ConstTypename`
    ///     * `GenericType`
    ///     * `ConstDatetime`
    ///     * `Numeric`
    ///         * `opt_float`
    ///     * `Character` and `ConstCharacter`
    ///     * `Bit` and `ConstBit`
    ///         * `BitWithLength` and `BitWithoutLength`
    ///     * `func_name` (partially)
    fn parse_type(&mut self, kind: TypeNameKind) -> ScanResult<TypeName> {

        /*
            There's a difference here between C-PG's `AexprConst` and this production.
            While C-PG needs to work around a couple of reduce/reduce conflicts, we ignore them here.
            It will still emit Syntax errors like C-PG, but their location and specific message will differ.

            See https://github.com/postgres/postgres/blob/14e87ffa5c543b5f30ead7413084c25f7735039f/src/backend/parser/gram.y#L17315-L17335
        */

        match self.buffer.peek2() {
            (Ok(Kw(kw)), Ok(Op(Dot))) if kind == Const && kw.details().category() == ColumnName => {

                let name = attrs(col_label()).parse(&mut self.buffer)?;

                let modifiers = opt_type_modifiers().parse(&mut self.buffer)?;
                return Ok(GenericTypeName::new(name, modifiers).into())
            },
            _ => {}
        }

        let name = attrs(identifier().map(From::from))
            .maybe_match()
            .parse(&mut self.buffer)?;

        if let Some(name) = name {
            let modifiers = opt_type_modifiers().parse(&mut self.buffer)?;
            return Ok(GenericTypeName::new(name, modifiers).into())
        }

        consume!{self
            Ok {
                Kw(JsonKw) => Ok(Json),
                Kw(Boolean) => Ok(Bool),
                Kw(Smallint) => Ok(Int2),
                Kw(Int | Integer) => Ok(Int4),
                Kw(Bigint) => Ok(Int8),
                Kw(Real) => Ok(Float4),
                Kw(Dec | Decimal | NumericKw) => {
                    let modifiers = opt_type_modifiers().parse(&mut self.buffer)?;
                    Ok(Numeric(modifiers))
                },
                Kw(Float) => {
                    // `opt_float`:
                    match i32_literal_paren().optional().parse(&mut self.buffer)? {
                        None | Some(25..=53) => Ok(Float8),
                        Some(1..=24) => Ok(Float4),
                        Some(num @ ..=0) => {
                            let loc = self.buffer.current_location();
                            let err = ParserError::new(FloatPrecisionUnderflow(num), loc);
                            Err(err.into())
                        },
                        Some(num @ 54..) => {
                            let loc = self.buffer.current_location();
                            let err = ParserError::new(FloatPrecisionOverflow(num), loc);
                            Err(err.into())
                        },
                    }
                },
                Kw(BitKw) => {
                    let varying = opt_varying().parse(&mut self.buffer)?;
                    let mut modifiers = opt_type_modifiers().parse(&mut self.buffer)?;

                    if varying {
                        Ok(Varbit(modifiers))
                    }
                    else {
                        if modifiers.is_empty() && kind == Simple {
                            modifiers = vec![IntegerConst(1)];
                        }
                        Ok(Bit(modifiers))
                    }
                },
                Kw(VarcharKw) => self.character_type(true, kind),
                Kw(Char | Character | Nchar) => {
                    let varying = opt_varying().parse(&mut self.buffer)?;
                    self.character_type(varying, kind)
                },
                Kw(National), Kw(Char | Character) => {
                    let varying = opt_varying().parse(&mut self.buffer)?;
                    self.character_type(varying, kind)
                },
                Kw(TimestampKw) => {
                    let precision = i32_literal_paren().optional().parse(&mut self.buffer)?;
                    if opt_timezone().parse(&mut self.buffer)? {
                        Ok(TimestampTz { precision })
                    }
                    else {
                        Ok(Timestamp { precision })
                    }
                },
                Kw(TimeKw) => {
                    let precision = i32_literal_paren().optional().parse(&mut self.buffer)?;
                    if opt_timezone().parse(&mut self.buffer)? {
                        Ok(TimeTz { precision })
                    }
                    else {
                        Ok(Time { precision })
                    }
                },
                Kw(IntervalKw) => {
                    let precision = i32_literal_paren().optional().parse(&mut self.buffer)?;
                    if precision.is_none() && kind == Simple {
                        let range = opt_interval().parse(&mut self.buffer)?;
                        Ok(Interval(range))
                    }
                    else {
                        Ok(Interval(IntervalRange::Full { precision }))
                    }
                },
                Kw(kw) if kw.details().category() == Unreserved => {
                    if kw == Double && Precision.optional().parse(&mut self.buffer)?.is_some() {
                        // `Double` conflicts with, and has lower precedence than, any other `Keyword::Unreserved`.
                        // If it's followed by `Precision`, then it's a Float8.
                        // Otherwise, it's a plain `Unreserved` keyword, which can be its own User Defined Type.
                        Ok(Float8)
                    }
                    else {
                        let prefix = kw.details().text().into();
                        let name = self.attrs(prefix)?;
                        let modifiers = opt_type_modifiers().parse(&mut self.buffer)?;

                        Ok(GenericTypeName::new(name, modifiers).into())
                    }
                },
                Kw(kw) if kw.details().category() == TypeFuncName => {
                    let name = kw.details().text().into();
                    let name = if kind == Simple {
                        self.attrs(name)?
                    }
                    else {
                        vec![name]
                    };

                    let modifiers = opt_type_modifiers().parse(&mut self.buffer)?;

                    Ok(GenericTypeName::new(name, modifiers).into())
                },
            }
            Err {
                Ok(_) => {
                    let loc = self.buffer.current_location();
                    NoMatch(loc)
                },
                Err(err) => err.into(),
            }
        }
    }

    /// Aliases:
    /// * `Character`
    /// * `ConstCharacter`
    ///
    /// Inline:
    /// * `CharacterWithLength` and `CharacterWithoutLength`
    ///     * `character`
    fn character_type(&mut self, varying: bool, kind: TypeNameKind) -> ScanResult<TypeName> {

        /*
              VARCHAR opt_paren_i32
            | (CHAR | CHARACTER | NCHAR) opt_varying opt_paren_i32
            | NATIONAL (CHAR | CHARACTER) opt_varying opt_paren_i32
        */

        let mut length = i32_literal_paren().optional().parse(&mut self.buffer)?;

        if varying {
            Ok(Varchar { max_length: length })
        }
        else {
            if length.is_none() && kind == Simple {
                length = Some(1);
            }
            Ok(Bpchar { length })
        }
    }
}

/// Post-condition: Vec is **Not** empty
fn opt_array_bounds() -> impl Combinator<Output = Vec<Option<i32>>> {

    /*
          ARRAY ( '[' ICONST ']' )?
        | ( '[' ( ICONST )? ']' )*
    */

    match_first!{
        Array
            .and_right(
                between(
                    OpenBracket,
                    i32_literal(),
                    CloseBracket
                )
                .optional()
            )
            .map(|dim| vec![dim]),
        many(
            between(
                OpenBracket,
                i32_literal().optional(),
                CloseBracket
            )
        )
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::SetOf;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test_case("json", scalar(Json))]
    #[test_case("json[]", array(Json, vec![None]))]
    #[test_case("setof json", scalar(Json).returning_table())]
    #[test_case("setof json[]", array(Json, vec![None]).returning_table())]
    fn test_type_name(source: &str, expected: SystemType) {

        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        let actual = parser.type_name();

        assert_eq!(
            Ok(expected.clone()),
            actual,
            r"expected {expected:?} for source {source:?} but actually got {actual:?}"
        );
    }

    fn scalar(name: TypeName) -> SystemType {
        array(name, vec![])
    }

    fn array(name: TypeName, array_bounds: Vec<Option<i32>>) -> SystemType {
        SystemType::new(name, array_bounds, SetOf::Scalar)
    }

    #[test_case("array", vec![None])]
    #[test_case("array[7]", vec![Some(7)])]
    #[test_case("[]", vec![None])]
    #[test_case("[9]", vec![Some(9)])]
    #[test_case("[5][]", vec![Some(5), None])]
    #[test_case("[3][4]", vec![Some(3), Some(4)])]
    fn test_opt_array_bounds(source: &str, expected: Vec<Option<i32>>) {

        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = opt_array_bounds().parse(&mut stream);

        assert_eq!(
            Ok(expected.clone()),
            actual,
            r"expected {expected:?} for source {source:?} but actually got {actual:?}"
        );
    }

    #[test_case("json",                           Simple, Json)]
    #[test_case("boolean",                        Simple, Bool)]
    #[test_case("smallint",                       Simple, Int2)]
    #[test_case("int",                            Simple, Int4)]
    #[test_case("integer",                        Simple, Int4)]
    #[test_case("bigint",                         Simple, Int8)]
    #[test_case("real",                           Simple, Float4)]
    #[test_case("float",                          Simple, Float8)]
    #[test_case("float(17)",                      Simple, Float4)]
    #[test_case("float(44)",                      Simple, Float8)]
    #[test_case("double precision",               Simple, Float8)]
    #[test_case("decimal",                        Simple, Numeric(vec![]))]
    // TODO: #[test_case("decimal(10)",           Simple, Numeric(vec![...]))]
    #[test_case("dec",                            Simple, Numeric(vec![]))]
    // TODO: #[test_case("dec(20)",               Simple, Numeric(vec![...]))]
    #[test_case("numeric",                        Simple, Numeric(vec![]))]
    // TODO: #[test_case("numeric(30)",           Simple, Numeric(vec![...]))]
    #[test_case("time",                           Simple, Time { precision: None })]
    #[test_case("time(5)",                        Simple, Time { precision: Some(5) })]
    #[test_case("time without time zone",         Simple, Time { precision: None })]
    #[test_case("time(7) without time zone",      Simple, Time { precision: Some(7) })]
    #[test_case("time with time zone",            Simple, TimeTz { precision: None })]
    #[test_case("time(9) with time zone",         Simple, TimeTz { precision: Some(9) })]
    #[test_case("timestamp",                      Simple, Timestamp { precision: None })]
    #[test_case("timestamp(5)",                   Simple, Timestamp { precision: Some(5) })]
    #[test_case("timestamp without time zone",    Simple, Timestamp { precision: None })]
    #[test_case("timestamp(7) without time zone", Simple, Timestamp { precision: Some(7) })]
    #[test_case("timestamp with time zone",       Simple, TimestampTz { precision: None })]
    #[test_case("timestamp(9) with time zone",    Simple, TimestampTz { precision: Some(9) })]
    #[test_case("varchar",                        Simple, Varchar { max_length: None })]
    #[test_case("char varying",                   Simple, Varchar { max_length: None })]
    #[test_case("varchar(3)",                     Simple, Varchar { max_length: Some(3) })]
    #[test_case("char varying(5)",                Simple, Varchar { max_length: Some(5) })]
    #[test_case("character varying",              Simple, Varchar { max_length: None })]
    #[test_case("character varying(2)",           Simple, Varchar { max_length: Some(2) })]
    #[test_case("nchar varying",                  Simple, Varchar { max_length: None })]
    #[test_case("nchar varying(7)",               Simple, Varchar { max_length: Some(7) })]
    #[test_case("national char varying",          Simple, Varchar { max_length: None })]
    #[test_case("national char varying(5)",       Simple, Varchar { max_length: Some(5) })]
    #[test_case("national character varying",     Simple, Varchar { max_length: None })]
    #[test_case("national character varying(3)",  Simple, Varchar { max_length: Some(3) })]
    #[test_case("bit",                            Simple, Bit(vec![IntegerConst(1)]))]
    // TODO: #[test_case("bit(modif)",            Simple, Bit(vec![...]))]
    #[test_case("char",                           Simple, Bpchar { length: Some(1) })]
    #[test_case("char",                           Const,  Bpchar { length: None })]
    #[test_case("char(4)",                        Simple, Bpchar { length: Some(4) })]
    #[test_case("character",                      Simple, Bpchar { length: Some(1) })]
    #[test_case("character",                      Const,  Bpchar { length: None })]
    #[test_case("character(2)",                   Simple, Bpchar { length: Some(2) })]
    #[test_case("nchar",                          Simple, Bpchar { length: Some(1) })]
    #[test_case("nchar",                          Const,  Bpchar { length: None })]
    #[test_case("nchar(9)",                       Simple, Bpchar { length: Some(9) })]
    #[test_case("national char",                  Simple, Bpchar { length: Some(1) })]
    #[test_case("national char",                  Const,  Bpchar { length: None })]
    #[test_case("national char(7)",               Simple, Bpchar { length: Some(7) })]
    #[test_case("national character",             Simple, Bpchar { length: Some(1) })]
    #[test_case("national character",             Const,  Bpchar { length: None })]
    #[test_case("national character(8)",          Simple, Bpchar { length: Some(8) })]
    #[test_case("bit",                            Const,  Bit(vec![]))]
    #[test_case("bit varying",                    Simple, Varbit(vec![]))]
    #[test_case("bit varying",                    Const,  Varbit(vec![]))]
    #[test_case("interval",                       Simple, IntervalRange::default().into())]
    #[test_case("interval day",                   Simple, IntervalRange::Day.into())]
    #[test_case("interval",                       Const,  IntervalRange::default().into())]
    #[test_case("interval(5)",                    Simple, IntervalRange::Full { precision: Some(5) }.into())]
    #[test_case("interval(7)",                    Const,  IntervalRange::Full { precision: Some(7) }.into())]
    #[test_case("identif.attrib",                 Simple, GenericTypeName::new(vec!["identif".into(), "attrib".into()], vec![]).into())]
    // TODO: #[test_case("identif(modif)",        Simple, GenericTypeName::new(vec!["identif".into(), "attrib".into()], vec![...]).into())]
    #[test_case("double",                         Simple, GenericTypeName::new(vec!["double".into()], vec![]).into())]
    #[test_case("double.unreserved",              Simple, GenericTypeName::new(vec!["double".into(), "unreserved".into()], vec![]).into())]
    // TODO: #[test_case("double.unreserved(modif)", Simple, GenericTypeName::new(vec!["double".into(), "unreserved".into()], vec![]).into())]
    #[test_case("authorization",                  Const,  GenericTypeName::new(vec!["authorization".into()], vec![]).into())]
    // TODO: #[test_case("authorization(modif)",  Const,  GenericTypeName::new(vec!["authorization".into()], vec![...]).into())]
    #[test_case("full.type_func_name",            Simple, GenericTypeName::new(vec!["full".into(), "type_func_name".into()], vec![]).into())]
    // TODO: #[test_case("full.type_func_name(modif)",     Const,  GenericTypeName::new(vec!["full".into(), "type_func_name".into()], vec![...]).into())]
    #[test_case("dec.col_name",                   Const,  GenericTypeName::new(vec!["dec".into(), "col_name".into()], vec![]).into())]
    // TODO: #[test_case("dec.col_name(modif)",   Const,  GenericTypeName::new(vec!["dec".into(), "col_name".into()], vec![...]).into())]
    fn test_simple_typename(source: &str, kind: TypeNameKind, expected: TypeName) {

        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        let actual = parser.parse_type(kind);

        assert_eq!(
            Ok(expected.clone()),
            actual,
            "source:   {source:?}\n\
             expected: Ok({expected:?})\n\
             actual:   {actual:?}"
        );
    }
}

use crate::lexer::Keyword::Array;
use crate::lexer::Keyword::Bigint;
use crate::lexer::Keyword::Bit as BitKw;
use crate::lexer::Keyword::Boolean;
use crate::lexer::Keyword::Char;
use crate::lexer::Keyword::Character;
use crate::lexer::Keyword::Dec;
use crate::lexer::Keyword::Decimal;
use crate::lexer::Keyword::Double;
use crate::lexer::Keyword::Float;
use crate::lexer::Keyword::Int;
use crate::lexer::Keyword::Integer;
use crate::lexer::Keyword::Interval as IntervalKw;
use crate::lexer::Keyword::Json as JsonKw;
use crate::lexer::Keyword::National;
use crate::lexer::Keyword::Nchar;
use crate::lexer::Keyword::Numeric as NumericKw;
use crate::lexer::Keyword::Precision;
use crate::lexer::Keyword::Real;
use crate::lexer::Keyword::Setof;
use crate::lexer::Keyword::Smallint;
use crate::lexer::Keyword::Time as TimeKw;
use crate::lexer::Keyword::Timestamp as TimestampKw;
use crate::lexer::Keyword::Varchar as VarcharKw;
use crate::lexer::KeywordCategory::ColumnName;
use crate::lexer::KeywordCategory::TypeFuncName;
use crate::lexer::KeywordCategory::Unreserved;
use crate::lexer::OperatorKind::CloseBracket;
use crate::lexer::OperatorKind::Dot;
use crate::lexer::OperatorKind::OpenBracket;
use crate::lexer::RawTokenKind::Keyword as Kw;
use crate::lexer::RawTokenKind::Operator as Op;
use crate::parser::ast_node::ExprNode::IntegerConst;
use crate::parser::ast_node::GenericTypeName;
use crate::parser::ast_node::IntervalRange;
use crate::parser::ast_node::SystemType;
use crate::parser::ast_node::TypeName;
use crate::parser::ast_node::TypeName::Bit;
use crate::parser::ast_node::TypeName::Bool;
use crate::parser::ast_node::TypeName::Bpchar;
use crate::parser::ast_node::TypeName::Float4;
use crate::parser::ast_node::TypeName::Float8;
use crate::parser::ast_node::TypeName::Int2;
use crate::parser::ast_node::TypeName::Int4;
use crate::parser::ast_node::TypeName::Int8;
use crate::parser::ast_node::TypeName::Interval;
use crate::parser::ast_node::TypeName::Json;
use crate::parser::ast_node::TypeName::Numeric;
use crate::parser::ast_node::TypeName::Time;
use crate::parser::ast_node::TypeName::TimeTz;
use crate::parser::ast_node::TypeName::Timestamp;
use crate::parser::ast_node::TypeName::TimestampTz;
use crate::parser::ast_node::TypeName::Varbit;
use crate::parser::ast_node::TypeName::Varchar;
use crate::parser::col_label;
use crate::parser::combinators::between;
use crate::parser::combinators::identifier;
use crate::parser::combinators::many;
use crate::parser::combinators::match_first;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::const_numeric_parsers::i32_literal;
use crate::parser::consume_macro::consume;
use crate::parser::i32_literal_paren;
use crate::parser::opt_interval::opt_interval;
use crate::parser::opt_type_modifiers;
use crate::parser::opt_varying;
use crate::parser::result::Required;
use crate::parser::result::ScanErrorKind::NoMatch;
use crate::parser::result::ScanResult;
use crate::parser::type_parsers::TypeNameKind::Const;
use crate::parser::type_parsers::TypeNameKind::Simple;
use crate::parser::Parser;
use crate::parser::ParserError;
use crate::parser::ParserErrorKind::FloatPrecisionOverflow;
use crate::parser::ParserErrorKind::FloatPrecisionUnderflow;
use crate::parser::{attrs, opt_timezone};
