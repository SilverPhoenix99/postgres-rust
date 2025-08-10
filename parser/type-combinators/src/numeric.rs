/// Alias: `Numeric`
///
/// It doesn't include `DOUBLE PRECISION` (See [generic_type()](crate::simple_typename::generic_type))
pub(super) fn numeric(ctx: &mut ParserContext) -> scan::Result<TypeName> {

    /*
          BOOLEAN
        | INT
        | INTEGER
        | SMALLINT
        | BIGINT
        | REAL
        | FLOAT ( '(' ICONST ')' )?
        | NUMERIC ( type_modifiers )?
        | DEC ( type_modifiers )?
        | DECIMAL ( type_modifiers )?
    */

    alt!(
        decimal,
        float,
        int,
        Boolean.map(|_| Bool),
        Smallint.map(|_| Int2),
        Bigint.map(|_| Int8),
        Real.map(|_| Float4),
    ).parse(ctx)
}

fn int(ctx: &mut ParserContext) -> scan::Result<TypeName> {

    /*
        INT | INTEGER
    */

    alt!(Int, Integer).parse(ctx)?;
    Ok(Int4)
}

fn decimal(ctx: &mut ParserContext) -> scan::Result<TypeName> {

    /*
          NUMERIC ( type_modifiers )?
        | DEC ( type_modifiers )?
        | DECIMAL ( type_modifiers )?
    */

    let (_, typ) = seq!(
        alt!(Dec, Decimal, Kw::Numeric),
        type_modifiers
            .optional()
            .map(Numeric),
    ).parse(ctx)?;

    Ok(typ)
}

/// Inlined: `opt_float`
fn float(ctx: &mut ParserContext) -> scan::Result<TypeName> {

    /*
        FLOAT ( '(' ICONST ')' )?
    */

    let (_, Located(precision, loc)) = seq!(
        Float,
        located!(precision.optional())
    ).parse(ctx)?;

    match precision {
        None | Some(25..=53) => Ok(Float8),
        Some(1..=24) => Ok(Float4),
        Some(num @ ..=0) => Err(FloatPrecisionUnderflow(num).at_location(loc).into()),
        Some(num @ 54..) => Err(FloatPrecisionOverflow(num).at_location(loc).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::expr_list;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::IntegerConst,
        pg_elog::Error::Parser,
        pg_parser_core::scan::Error::ScanErr,
    };

    #[test_case("boolean"     => Ok(Bool))]
    #[test_case("smallint"    => Ok(Int2))]
    #[test_case("int"         => Ok(Int4))]
    #[test_case("integer"     => Ok(Int4))]
    #[test_case("bigint"      => Ok(Int8))]
    #[test_case("real"        => Ok(Float4))]
    #[test_case("float"       => Ok(Float8))]
    #[test_case("float(17)"   => Ok(Float4))]
    #[test_case("float(44)"   => Ok(Float8))]
    #[test_case("decimal"     => Ok(Numeric(None)))]
    #[test_case("decimal(10)" => Ok(Numeric(Some(vec![IntegerConst(10)]))))]
    #[test_case("dec"         => Ok(Numeric(None)))]
    #[test_case("dec(20)"     => Ok(Numeric(Some(vec![IntegerConst(20)]))))]
    #[test_case("numeric"     => Ok(Numeric(None)))]
    #[test_case("numeric(30)" => Ok(Numeric(Some(vec![IntegerConst(30)]))))]
    // Test error cases
    #[test_case("float(0)" => matches Err(ScanErr(
        Located(Parser(FloatPrecisionUnderflow(0)), _)
    )))]
    #[test_case("float(99)" => matches Err(ScanErr(
        Located(Parser(FloatPrecisionOverflow(99)), _)
    )))]
    fn test_numeric(source: &str) -> scan::Result<TypeName> {
        let mut ctx = ParserContext::new(source, expr_list);
        numeric(&mut ctx)
    }
}

use crate::type_modifiers;
use pg_ast::TypeName;
use pg_ast::TypeName::Bool;
use pg_ast::TypeName::Float4;
use pg_ast::TypeName::Float8;
use pg_ast::TypeName::Int2;
use pg_ast::TypeName::Int4;
use pg_ast::TypeName::Int8;
use pg_ast::TypeName::Numeric;
use pg_basics::IntoLocated;
use pg_basics::Located;
use pg_combinators::alt;
use pg_combinators::located;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_elog::parser::Error::FloatPrecisionOverflow;
use pg_elog::parser::Error::FloatPrecisionUnderflow;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Bigint;
use pg_lexer::Keyword::Boolean;
use pg_lexer::Keyword::Dec;
use pg_lexer::Keyword::Decimal;
use pg_lexer::Keyword::Float;
use pg_lexer::Keyword::Int;
use pg_lexer::Keyword::Integer;
use pg_lexer::Keyword::Real;
use pg_lexer::Keyword::Smallint;
use pg_parser_core::scan;
use pg_sink_combinators::precision;
