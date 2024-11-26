impl Parser<'_> {

    /// Alias: `ConstTypename`
    ///
    /// Inlined:
    ///     * `GenericType`
    ///     * `ConstDatetime`
    ///     * `Numeric`
    ///         * `opt_float`
    ///     * `Character` and `ConstCharacter`
    ///     * `Bit` and `ConstBit`
    ///         * `BitWithLength` and `BitWithoutLength`
    ///     * `func_name` (partially)
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

        /*
            There's a difference here between C-PG's `AexprConst` and this production.
            While C-PG needs to work around a couple of reduce/reduce conflicts, we ignore them here.
            It will still emit Syntax errors like C-PG, but their location and specific message will differ.

            See https://github.com/postgres/postgres/blob/14e87ffa5c543b5f30ead7413084c25f7735039f/src/backend/parser/gram.y#L17315-L17335
        */

        match self.buffer.peek2() {
            (Ok(Kw(kw)), Ok(Op(Dot))) if kw.category() == ColumnName => {

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

        let p = match_first!(
            Keyword::Json.map(|_| Json),
            Boolean.map(|_| Bool),
            Smallint.map(|_| Int2),
            or(Int, Integer).map(|_| Int4),
            Bigint.map(|_| Int8),
            Real.map(|_| Float4),
            match_first!(Dec, Decimal, Keyword::Numeric)
                .and_right(opt_type_modifiers())
                .map(Numeric),
        );

        if let Some(typ) = p.parse(&mut self.buffer).optional()? {
            return Ok(typ)
        }

        consume!{self
            Ok {
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
                Kw(Keyword::Bit) => {
                    let varying = opt_varying().parse(&mut self.buffer)?;
                    let modifiers = opt_type_modifiers().parse(&mut self.buffer)?;

                    if varying {
                        Ok(Varbit(modifiers))
                    }
                    else {
                        Ok(Bit(modifiers))
                    }
                },
                Kw(Keyword::Varchar) => self.character_type(true),
                Kw(Char | Character | Nchar) => {
                    let varying = opt_varying().parse(&mut self.buffer)?;
                    self.character_type(varying)
                },
                Kw(National), Kw(Char | Character) => {
                    let varying = opt_varying().parse(&mut self.buffer)?;
                    self.character_type(varying)
                },
                Kw(Keyword::Timestamp) => {
                    let precision = i32_literal_paren().optional().parse(&mut self.buffer)?;
                    if opt_timezone().parse(&mut self.buffer)? {
                        Ok(TimestampTz { precision })
                    }
                    else {
                        Ok(Timestamp { precision })
                    }
                },
                Kw(Keyword::Time) => {
                    let precision = i32_literal_paren().optional().parse(&mut self.buffer)?;
                    if opt_timezone().parse(&mut self.buffer)? {
                        Ok(TimeTz { precision })
                    }
                    else {
                        Ok(Time { precision })
                    }
                },
                Kw(Keyword::Interval) => {
                    let precision = i32_literal_paren().optional().parse(&mut self.buffer)?;
                    Ok(Interval(IntervalRange::Full { precision }))
                },
                Kw(kw) if kw.category() == Unreserved => {
                    if kw == Double && Precision.optional().parse(&mut self.buffer)?.is_some() {
                        // `Double` conflicts with, and has lower precedence than, any other `Keyword::Unreserved`.
                        // If it's followed by `Precision`, then it's a Float8.
                        // Otherwise, it's a plain `Unreserved` keyword, which can be its own User Defined Type.
                        Ok(Float8)
                    }
                    else {
                        let prefix = parser(move |_| Ok(kw.into()));
                        let name = attrs(prefix).parse(&mut self.buffer)?;
                        let modifiers = opt_type_modifiers().parse(&mut self.buffer)?;

                        Ok(GenericTypeName::new(name, modifiers).into())
                    }
                },
                Kw(kw) if kw.category() == TypeFuncName => {
                    let name = kw.into();
                    let name = vec![name];

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
    fn character_type(&mut self, varying: bool) -> ScanResult<TypeName> {

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
            Ok(Bpchar { length })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use crate::parser::ast_node::ExprNode::IntegerConst;
    use crate::parser::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("json",                           Json)]
    #[test_case("boolean",                        Bool)]
    #[test_case("smallint",                       Int2)]
    #[test_case("int",                            Int4)]
    #[test_case("integer",                        Int4)]
    #[test_case("bigint",                         Int8)]
    #[test_case("real",                           Float4)]
    #[test_case("float",                          Float8)]
    #[test_case("float(17)",                      Float4)]
    #[test_case("float(44)",                      Float8)]
    #[test_case("double precision",               Float8)]
    #[test_case("decimal",                        Numeric(vec![]))]
    #[test_case("decimal(10)",                    Numeric(vec![IntegerConst(10)]))]
    #[test_case("dec",                            Numeric(vec![]))]
    #[test_case("dec(20)",                        Numeric(vec![IntegerConst(20)]))]
    #[test_case("numeric",                        Numeric(vec![]))]
    #[test_case("numeric(30)",                    Numeric(vec![IntegerConst(30)]))]
    #[test_case("time",                           Time { precision: None })]
    #[test_case("time(5)",                        Time { precision: Some(5) })]
    #[test_case("time without time zone",         Time { precision: None })]
    #[test_case("time(7) without time zone",      Time { precision: Some(7) })]
    #[test_case("time with time zone",            TimeTz { precision: None })]
    #[test_case("time(9) with time zone",         TimeTz { precision: Some(9) })]
    #[test_case("timestamp",                      Timestamp { precision: None })]
    #[test_case("timestamp(5)",                   Timestamp { precision: Some(5) })]
    #[test_case("timestamp without time zone",    Timestamp { precision: None })]
    #[test_case("timestamp(7) without time zone", Timestamp { precision: Some(7) })]
    #[test_case("timestamp with time zone",       TimestampTz { precision: None })]
    #[test_case("timestamp(9) with time zone",    TimestampTz { precision: Some(9) })]
    #[test_case("varchar",                        Varchar { max_length: None })]
    #[test_case("char varying",                   Varchar { max_length: None })]
    #[test_case("varchar(3)",                     Varchar { max_length: Some(3) })]
    #[test_case("char varying(5)",                Varchar { max_length: Some(5) })]
    #[test_case("character varying",              Varchar { max_length: None })]
    #[test_case("character varying(2)",           Varchar { max_length: Some(2) })]
    #[test_case("nchar varying",                  Varchar { max_length: None })]
    #[test_case("nchar varying(7)",               Varchar { max_length: Some(7) })]
    #[test_case("national char varying",          Varchar { max_length: None })]
    #[test_case("national char varying(5)",       Varchar { max_length: Some(5) })]
    #[test_case("national character varying",     Varchar { max_length: None })]
    #[test_case("national character varying(3)",  Varchar { max_length: Some(3) })]
    #[test_case("bit",                            Bit(vec![]))]
    #[test_case("bit(77)",                        Bit(vec![IntegerConst(77)]))]
    #[test_case("bit varying",                    Varbit(vec![]))]
    #[test_case("char",                           Bpchar { length: None })]
    #[test_case("char(4)",                        Bpchar { length: Some(4) })]
    #[test_case("character",                      Bpchar { length: None })]
    #[test_case("character(2)",                   Bpchar { length: Some(2) })]
    #[test_case("nchar",                          Bpchar { length: None })]
    #[test_case("nchar(9)",                       Bpchar { length: Some(9) })]
    #[test_case("national char",                  Bpchar { length: None })]
    #[test_case("national char(7)",               Bpchar { length: Some(7) })]
    #[test_case("national character",             Bpchar { length: None })]
    #[test_case("national character(8)",          Bpchar { length: Some(8) })]
    #[test_case("interval",                       IntervalRange::default().into())]
    #[test_case("interval(7)",                    IntervalRange::Full { precision: Some(7) }.into())]
    // FIXME: #[test_case("identif.attrib",                 GenericTypeName::new(vec!["identif".into(), "attrib".into()], vec![]).into())]
    // FIXME: #[test_case("identif(33)",                    GenericTypeName::new(vec!["identif".into(), "attrib".into()], vec![IntegerConst(33)]).into())]
    #[test_case("double",                         GenericTypeName::new(vec!["double".into()], vec![]).into())]
    #[test_case("double.unreserved",              GenericTypeName::new(vec!["double".into(), "unreserved".into()], vec![]).into())]
    #[test_case("double.unreserved(55)",          GenericTypeName::new(vec!["double".into(), "unreserved".into()], vec![IntegerConst(55)]).into())]
    #[test_case("authorization",                  GenericTypeName::new(vec!["authorization".into()], vec![]).into())]
    #[test_case("authorization(23)",              GenericTypeName::new(vec!["authorization".into()], vec![IntegerConst(23)]).into())]
    // FIXME: #[test_case("func_name",                      GenericTypeName::new(vec!["full".into(), "type_func_name".into()], vec![]).into())]
    // FIXME: #[test_case("func_name(73)",                  GenericTypeName::new(vec!["full".into(), "type_func_name".into()], vec![IntegerConst(73)]).into())]
    #[test_case("dec.col_name",                   GenericTypeName::new(vec!["dec".into(), "col_name".into()], vec![]).into())]
    #[test_case("dec.col_name(17)",               GenericTypeName::new(vec!["dec".into(), "col_name".into()], vec![IntegerConst(17)]).into())]
    fn test_const_typename(source: &str, expected: TypeName) {

        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        let actual = parser.const_typename();

        assert_eq!(
            Ok(expected.clone()),
            actual,
            "source:   {source:?}\n\
             expected: Ok({expected:?})\n\
             actual:   {actual:?}"
        );
    }
}

use crate::lexer::Keyword;
use crate::lexer::Keyword::Bigint;
use crate::lexer::Keyword::Boolean;
use crate::lexer::Keyword::Char;
use crate::lexer::Keyword::Character;
use crate::lexer::Keyword::Dec;
use crate::lexer::Keyword::Decimal;
use crate::lexer::Keyword::Double;
use crate::lexer::Keyword::Float;
use crate::lexer::Keyword::Int;
use crate::lexer::Keyword::Integer;
use crate::lexer::Keyword::National;
use crate::lexer::Keyword::Nchar;
use crate::lexer::Keyword::Precision;
use crate::lexer::Keyword::Real;
use crate::lexer::Keyword::Smallint;
use crate::lexer::KeywordCategory::ColumnName;
use crate::lexer::KeywordCategory::TypeFuncName;
use crate::lexer::KeywordCategory::Unreserved;
use crate::lexer::OperatorKind::Dot;
use crate::lexer::RawTokenKind::Keyword as Kw;
use crate::lexer::RawTokenKind::Operator as Op;
use crate::parser::ast_node::GenericTypeName;
use crate::parser::ast_node::IntervalRange;
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
use crate::parser::attrs;
use crate::parser::col_label;
use crate::parser::combinators::identifier;
use crate::parser::combinators::match_first;
use crate::parser::combinators::or;
use crate::parser::combinators::parser;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::consume_macro::consume;
use crate::parser::i32_literal_paren;
use crate::parser::opt_timezone;
use crate::parser::opt_type_modifiers;
use crate::parser::opt_varying;
use crate::parser::result::Optional;
use crate::parser::result::ScanErrorKind::NoMatch;
use crate::parser::result::ScanResult;
use crate::parser::Parser;
use crate::parser::ParserError;
use crate::parser::ParserErrorKind::FloatPrecisionOverflow;
use crate::parser::ParserErrorKind::FloatPrecisionUnderflow;
