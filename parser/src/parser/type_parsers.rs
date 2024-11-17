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

        let setof = self.buffer.consume_kw_eq(Setof).no_match_to_option()?.is_some();

        let typ = self.simple_typename();

        let typ = if setof {
            // this means the `SETOF` keyword was present, so the production has already started
            typ.required(fn_info!())?
        }
        else {
            typ?
        };

        let array_bounds = self.opt_array_bounds()
            .optional()?
            .unwrap_or_default();

        let mut typ = typ.with_array_bounds(array_bounds);
        if setof {
            typ = typ.returning_table();
        }

        Ok(typ)
    }

    /// Post-condition: Vec is **Not** empty
    fn opt_array_bounds(&mut self) -> ScanResult<Vec<Option<i32>>> {

        /*
              ARRAY ( '[' ICONST ']' )?
            | ( '[' ( ICONST )? ']' )*
        */

        if self.buffer.consume_kw_eq(Array).no_match_to_option()?.is_some() {
            // one-dimensional arrays

            /*
                ARRAY ( '[' ICONST ']' )?
            */

            if self.buffer.consume_op(OpenBracket).optional()?.is_none() {
                // it's just `ARRAY`
                return Ok(vec![None])
            }

            /*
                '[' ICONST ']'
            */
            let dim_len = self.i32_literal().required(fn_info!())?;
            self.buffer.consume_op(CloseBracket).required(fn_info!())?;
            return Ok(vec![Some(dim_len)])
        }

        /*
            ( '[' ( ICONST )? ']' )*
        */

        let mut elements = Vec::new();

        while self.buffer.consume_op(OpenBracket).optional()?.is_some() {
            let dim_len = self.i32_literal().optional()?;
            self.buffer.consume_op(CloseBracket).required(fn_info!())?;
            elements.push(dim_len);
        }

        if elements.is_empty() {
            let loc = self.buffer.current_location();
            return Err(NoMatch(loc))
        }

        Ok(elements)
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
            | INTERVAL ( '(' ICONST ')' | opt_interval)?
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
            It will still emit Syntax errors like C-PG, but their location and specific will differ.

            See https://github.com/postgres/postgres/blob/14e87ffa5c543b5f30ead7413084c25f7735039f/src/backend/parser/gram.y#L17315-L17335
        */

        match self.buffer.peek2() {
            (Ok(Kw(kw)), Ok(Op(Dot))) if kind == Const && kw.details().category() == ColumnName => {
                let prefix = self.ident_or_keyword(|_| true)?;

                let name = self.attrs(prefix)?;
                let modifiers = self.opt_type_modifiers()?;
                return Ok(GenericTypeName::new(name, modifiers).into())
            },
            _ => {}
        }

        let ident = identifier(fn_info!()).parse(&mut self.buffer);
        if let Some(ident) = ident.no_match_to_option()? {
            let name = self.attrs(ident.into())?;
            let modifiers = self.opt_type_modifiers()?;
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
                    let modifiers = self.opt_type_modifiers()?;
                    Ok(Numeric(modifiers))
                },
                Kw(Float) => {
                    // `opt_float`:
                    match self.i32_literal_paren().optional()? {
                        None | Some(25..=53) => Ok(Float8),
                        Some(1..=24) => Ok(Float4),
                        Some(num @ ..=0) => {
                            let loc = self.buffer.current_location();
                            let err = ParserError::new(FloatPrecisionUnderflow(num), fn_info!(), loc);
                            Err(err.into())
                        },
                        Some(num @ 54..) => {
                            let loc = self.buffer.current_location();
                            let err = ParserError::new(FloatPrecisionOverflow(num), fn_info!(), loc);
                            Err(err.into())
                        },
                    }
                },
                Kw(BitKw) => {
                    let varying = self.opt_varying()?;
                    let mut modifiers = self.opt_type_modifiers()?;

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
                    let varying = self.opt_varying()?;
                    self.character_type(varying, kind)
                },
                Kw(National), Kw(Char | Character) => {
                    let varying = self.opt_varying()?;
                    self.character_type(varying, kind)
                },
                Kw(TimestampKw) => {
                    let precision = self.i32_literal_paren().optional()?;
                    if self.opt_timezone()? {
                        Ok(TimestampTz { precision })
                    }
                    else {
                        Ok(Timestamp { precision })
                    }
                },
                Kw(TimeKw) => {
                    let precision = self.i32_literal_paren().optional()?;
                    if self.opt_timezone()? {
                        Ok(TimeTz { precision })
                    }
                    else {
                        Ok(Time { precision })
                    }
                },
                Kw(IntervalKw) => {
                    let precision = self.i32_literal_paren().optional()?;
                    if precision.is_none() && kind == Simple {
                        let range = self.opt_interval()?;
                        Ok(Interval(range))
                    }
                    else {
                        Ok(Interval(IntervalRange::Full { precision }))
                    }
                },
                Kw(kw) if kw.details().category() == Unreserved => {
                    if kw == Double && self.buffer.consume_kw_eq(Precision).optional()?.is_some() {
                        // `Double` conflicts with, and has lower precedence than, any other `Keyword::Unreserved`.
                        // If it's followed by `Precision`, then it's a Float8.
                        // Otherwise, it's a plain `Unreserved` keyword, which can be its own User Defined Type.
                        Ok(Float8)
                    }
                    else {
                        let prefix = kw.details().text().into();
                        let name = self.attrs(prefix)?;
                        let modifiers = self.opt_type_modifiers()?;

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

                    let modifiers = self.opt_type_modifiers()?;

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

        let mut length = self.i32_literal_paren().optional()?;

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

    /// Post-condition: Vec **May** be empty
    fn opt_type_modifiers(&mut self) -> ParseResult<TypeModifiers> {

        /*
            ( '(' expr_list ')' )?
        */

        let modifiers = self.expr_list_paren()
            .optional()?
            .unwrap_or_default();

        Ok(modifiers)
    }

    fn opt_varying(&mut self) -> ParseResult<bool> {

        /*
            ( VARYING )?
        */

        let varying = self.buffer.consume_kw_eq(Varying)
            .optional()?
            .is_some();
        Ok(varying)
    }

    fn opt_timezone(&mut self) -> ParseResult<bool> {

        /*
            ( (WITH | WITHOUT) TIME ZONE )?
        */

        let result = consume!{self
            Ok {
                Kw(With), Kw(TimeKw), Kw(Zone) => Ok(true),
                Kw(Without), Kw(TimeKw), Kw(Zone) => Ok(false),
                _ => Ok(false)
            }
            Err {
                Err(err) => err,
            }
        };

        let result = result.optional()?.unwrap_or(false);

        Ok(result)
    }

    pub(in crate::parser) fn opt_interval(&mut self) -> ParseResult<IntervalRange> {
        use IntervalRange::*;

        /*
              YEAR
            | YEAR TO MONTH
            | MONTH
            | DAY
            | DAY TO HOUR
            | DAY TO MINUTE
            | DAY TO SECOND ( '(' ICONST ')' )?
            | HOUR
            | HOUR TO MINUTE
            | HOUR TO SECOND ( '(' ICONST ')' )?
            | MINUTE
            | MINUTE TO SECOND ( '(' ICONST ')' )?
            | SECOND ( '(' ICONST ')' )?
        */


        let result = consume!{self
            Ok {
                Kw(SecondKw) => {
                    let precision = self.i32_literal_paren().optional()?;
                    Ok(Second { precision })
                },
                Kw(MinuteKw) => {
                    if self.buffer.consume_kw_eq(To).optional()?.is_some() {
                        self.buffer.consume_kw_eq(SecondKw).required(fn_info!())?;
                        let precision = self.i32_literal_paren().optional()?;
                        Ok(MinuteToSecond { precision })
                    }
                    else {
                        Ok(Minute)
                    }
                },
                Kw(HourKw) => {
                    if self.buffer.consume_kw_eq(To).optional()?.is_some() {
                        let result = consume!{self
                            Ok {
                                Kw(MinuteKw) => Ok(HourToMinute),
                                Kw(SecondKw) => {
                                    let precision = self.i32_literal_paren().optional()?;
                                    Ok(HourToSecond { precision })
                                },
                            }
                            Err {
                                Ok(_) => {
                                    let loc = self.buffer.current_location();
                                    syntax_err(fn_info!(), loc)
                                },
                                Err(Eof(loc)) => syntax_err(fn_info!(), loc),
                                Err(NotEof(err)) => err,
                            }
                        };
                        result.map_err(NotEof)
                    }
                    else {
                        Ok(Hour)
                    }
                },
                Kw(DayKw) => {
                    if self.buffer.consume_kw_eq(To).optional()?.is_some() {
                        let result = consume!{self
                            Ok {
                                Kw(HourKw) => Ok(DayToHour),
                                Kw(MinuteKw) => Ok(DayToMinute),
                                Kw(SecondKw) => {
                                    let precision = self.i32_literal_paren().optional()?;
                                    Ok(DayToSecond { precision })
                                },
                            }
                            Err {
                                Ok(_) => {
                                    let loc = self.buffer.current_location();
                                    syntax_err(fn_info!(), loc)
                                },
                                Err(Eof(loc)) => syntax_err(fn_info!(), loc),
                                Err(NotEof(err)) => err,
                            }
                        };
                        result.map_err(NotEof)
                    }
                    else {
                        Ok(Day)
                    }
                },
                Kw(MonthKw) => Ok(Month),
                Kw(YearKw) => {
                    if self.buffer.consume_kw_eq(To).optional()?.is_some() {
                        self.buffer.consume_kw_eq(MonthKw).required(fn_info!())?;
                        Ok(YearToMonth)
                    }
                    else {
                        Ok(Year)
                    }
                },
                _ => Ok(Default::default()),
            }
            Err {
                Err(err) => err,
            }
        };
        let result = result.optional()?.unwrap_or_default();
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::SetOf;
    use crate::parser::tests::DEFAULT_CONFIG;
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

        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        let actual = parser.opt_array_bounds();

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

    #[test_case("",                  IntervalRange::default())]
    #[test_case("year",              IntervalRange::Year)]
    #[test_case("year to month",     IntervalRange::YearToMonth)]
    #[test_case("month",             IntervalRange::Month)]
    #[test_case("day",               IntervalRange::Day)]
    #[test_case("day to hour",       IntervalRange::DayToHour)]
    #[test_case("day to second",     IntervalRange::DayToSecond { precision: None })]
    #[test_case("day to second(7)",  IntervalRange::DayToSecond { precision: Some(7) })]
    #[test_case("hour",              IntervalRange::Hour)]
    #[test_case("hour to minute",    IntervalRange::HourToMinute)]
    #[test_case("hour to second",    IntervalRange::HourToSecond { precision: None })]
    #[test_case("hour to second(5)", IntervalRange::HourToSecond { precision: Some(5) })]
    #[test_case("second",            IntervalRange::Second { precision: None })]
    #[test_case("second(3)",         IntervalRange::Second { precision: Some(3) })]
    fn test_opt_interval(source: &str, expected: IntervalRange) {

        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        let actual = parser.opt_interval();

        assert_eq!(
            Ok(expected),
            actual,
            r"expected {expected:?} for source {source:?} but actually got {actual:?}"
        );
    }
}

use crate::{
    lexer::{
        Keyword::{
            Array,
            Bigint,
            Bit as BitKw,
            Boolean,
            Char,
            Character,
            Day as DayKw,
            Dec,
            Decimal,
            Double,
            Float,
            Hour as HourKw,
            Int,
            Integer,
            Interval as IntervalKw,
            Json as JsonKw,
            Minute as MinuteKw,
            Month as MonthKw,
            National,
            Nchar,
            Numeric as NumericKw,
            Precision,
            Real,
            Second as SecondKw,
            Setof,
            Smallint,
            Time as TimeKw,
            Timestamp as TimestampKw,
            To,
            Varchar as VarcharKw,
            Varying,
            With,
            Without,
            Year as YearKw,
            Zone,
        },
        KeywordCategory::{ColumnName, TypeFuncName, Unreserved},
        OperatorKind::{CloseBracket, Dot, OpenBracket},
        RawTokenKind::{Keyword as Kw, Operator as Op},
    },
    parser::{
        ast_node::{
            ExprNode::IntegerConst,
            GenericTypeName,
            IntervalRange,
            SystemType,
            TypeModifiers,
            TypeName::{self, *},
        },
        combinators::{identifier, ParserFunc},
        consume_macro::consume,
        error::syntax_err,
        result::{
            EofErrorKind::{Eof, NotEof},
            Optional,
            Required,
            ScanErrorKind::NoMatch,
            ScanResult,
            ScanResultTrait
        },
        ParseResult,
        Parser,
        ParserError,
        ParserErrorKind::{FloatPrecisionOverflow, FloatPrecisionUnderflow}
    },
};
use postgres_basics::fn_info;
use TypeNameKind::*;
