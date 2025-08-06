/// Alias: `Typename`
pub(super) fn typename(stream: &mut TokenStream) -> scan::Result<Type> {

    /*
        ( SETOF )? SimpleTypename ( array_bounds )?
    */

    alt!(
        seq!(Setof, record_typename)
            .map(|(_, typename)| {
                typename.returning_table()
            }),
        record_typename,
    ).parse(stream)
}

fn record_typename(stream: &mut TokenStream) -> scan::Result<Type> {

    /*
        SimpleTypename ( array_bounds )?
    */

    let (typename, array_bounds) = seq!(
        simple_typename,
        array_bounds.optional()
    ).parse(stream)?;

    let typename = typename.with_array_bounds(array_bounds);

    Ok(typename)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_ast::SetOf;
    use pg_ast::TypeName;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("int", Type::new(TypeName::Int4, None, SetOf::Record))]
    #[test_case("int[]", Type::new(TypeName::Int4, Some(vec![None]), SetOf::Record))]
    #[test_case("setof int", Type::new(TypeName::Int4, None, SetOf::Table))]
    #[test_case("setof int[]", Type::new(TypeName::Int4, Some(vec![None]), SetOf::Table))]
    #[test_case("setof double precision[10][]", Type::new(TypeName::Float8, Some(vec![Some(10), None]), SetOf::Table))]
    fn test_typename(source: &str, expected: Type) {
        test_parser!(source, typename, expected)
    }

    #[test]
    fn test_generic_typename() {
        test_parser!(
            source = "setof some_.qualified_name",
            parser = typename,
            expected = Type::new(
            TypeName::Generic {
                name: vec!["some_".into(), "qualified_name".into()],
                type_modifiers: None,
            },
            None,
            SetOf::Table,
        )
        )
    }
}

use crate::combinators::array_bounds;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::seq;
use crate::combinators::simple_typename;
use pg_ast::Type;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Setof;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
