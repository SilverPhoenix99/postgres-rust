/// Alias: `GenericType`
///
/// Includes `DOUBLE PRECISION` due to conflict with `Unreserved` keywords.
pub(super) fn generic_type(ctx: &mut ParserContext) -> scan::Result<TypeName> {

    /*
          DOUBLE PRECISION
        | type_function_name ( attrs )? ( type_modifiers )?
    */

    // `Double` conflicts with, and has lower precedence than, any other `Keyword::Unreserved`.
    // If it's followed by `Precision`, then it's a Float8.
    // Otherwise, it's a plain `Unreserved` keyword, which can be its own User Defined Type.
    if matches!(ctx.stream_mut().peek2(), Ok((Keyword(Double), Keyword(Precision)))) {
        ctx.stream_mut().skip(2);
        return Ok(Float8)
    }

    let (name, type_modifiers) = seq!(
        attrs!(type_function_name),
        type_modifiers.optional()
    ).parse(ctx)?;

    Ok(Generic { name, type_modifiers })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::expr_list;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::IntegerConst;
    use test_case::test_case;

    #[test_case("double precision"      => Ok(Float8))]
    #[test_case("identif.attrib"        => Ok(TypeName::Generic { name: vec!["identif".into(), "attrib".into()], type_modifiers: None }))]
    #[test_case("identif(33)"           => Ok(TypeName::Generic { name: vec!["identif".into()], type_modifiers: Some(vec![IntegerConst(33)]) }))]
    #[test_case("double"                => Ok(TypeName::Generic { name: vec!["double".into()], type_modifiers: None }))]
    #[test_case("double.unreserved"     => Ok(TypeName::Generic { name: vec!["double".into(), "unreserved".into()], type_modifiers: None }))]
    #[test_case("double.unreserved(55)" => Ok(TypeName::Generic { name: vec!["double".into(), "unreserved".into()], type_modifiers: Some(vec![IntegerConst(55)]) }))]
    #[test_case("full.type_func_name"   => Ok(TypeName::Generic { name: vec!["full".into(), "type_func_name".into()], type_modifiers: None }))]
    fn test_generic_type(source: &str) -> scan::Result<TypeName> {
        let mut ctx = ParserContext::new(source, expr_list);
        generic_type(&mut ctx)
    }
}

use crate::type_modifiers;
use pg_ast::TypeName;
use pg_ast::TypeName::Float8;
use pg_ast::TypeName::Generic;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Double;
use pg_lexer::Keyword::Precision;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenValue;
use pg_sink_combinators::attrs;
use pg_sink_combinators::type_function_name;
use TokenValue::Keyword;
