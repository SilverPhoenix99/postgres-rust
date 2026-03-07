/// Alias: `SimpleTypename`
pub(in crate::combinators) fn simple_typename(ctx: &mut ParserContext) -> scan::Result<TypeName> {

    alt!(
        Kw::Json.map(|_| Json),
        numeric,
        bit(Some(1)), // BitWithoutLength: `bit` defaults to `bit(1)`
        character(Some(1)), // CharacterWithoutLength: `char` defaults to `char(1)`
        timestamp,
        time,
        interval_type.map(From::from),
        generic_type
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::IntegerConst;
    use test_case::test_case;

    #[test_case("json" => Ok(Json))]
    // Quick checks
    #[test_case("int"         => matches Ok(_))]
    #[test_case("interval"    => matches Ok(_))]
    #[test_case("timestamp"   => matches Ok(_))]
    #[test_case("time"        => matches Ok(_))]
    #[test_case("identif(33)" => matches Ok(_))]
    // Still quick checks, but confirming the default lengths are set to 1
    #[test_case("bit" => Ok(TypeName::Bit(Some(vec![IntegerConst(1)]))))]
    #[test_case("char" => Ok(TypeName::Bpchar { length: Some(1) }))]
    fn test_simple_typename(source: &str) -> scan::Result<TypeName> {
        let mut ctx = ParserContext::new(source);
        simple_typename(&mut ctx)
    }
}

use super::bit;
use super::character;
use super::generic_type;
use super::interval_type;
use super::numeric;
use super::time;
use super::timestamp;
use pg_ast::TypeName;
use pg_ast::TypeName::Json;
use pg_combinators::alt;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword as Kw;
use pg_parser_core::scan;
