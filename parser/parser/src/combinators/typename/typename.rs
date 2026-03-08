/// Alias: `Typename`
pub(in crate::combinators) fn typename(ctx: &mut ParserContext) -> scan::Result<Type> {

    /*
        ( SETOF )? SimpleTypename ( array_bounds )?
    */

    let (set_of, type_name, array_bounds) = seq!(
        Setof.optional(),
        simple_typename,
        array_bounds.optional()
    ).parse(ctx)?;

    let mut r#type = Type::from(type_name);
    r#type
        .set_mult(set_of.is_some().into())
        .set_array_bounds(array_bounds);

    Ok(r#type)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{SetOf, TypeName};
    use test_case::test_case;

    #[test_case("int" => Ok(Type::from(TypeName::Int4)))]
    #[test_case("int[]" => Ok(
        Type::from(TypeName::Int4)
            .with_array_bounds(vec![None])
    ))]
    #[test_case("setof int" => Ok(
        Type::from(TypeName::Int4)
            .with_mult(SetOf::Table)
    ))]
    #[test_case("setof int[]" => Ok(
        Type::from(TypeName::Int4)
            .with_array_bounds(vec![None])
            .with_mult(SetOf::Table)
    ))]
    #[test_case("setof double precision[10][]" => Ok(
        Type::from(TypeName::Float8)
            .with_array_bounds(vec![Some(10), None])
            .with_mult(SetOf::Table)
    ))]
    #[test_case("setof some_.qualified_name" => Ok(
        Type::from(
            TypeName::Generic {
                name: vec!["some_".into(), "qualified_name".into()],
                type_modifiers: None,
            }
        )
        .with_mult(SetOf::Table)
    ))]
    fn test_typename(source: &str) -> scan::Result<Type> {
        test_parser!(source, typename)
    }
}

use super::array_bounds;
use super::simple_typename;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_ast::Type;
use pg_lexer::Keyword::Setof;
use pg_parser_core::scan;
