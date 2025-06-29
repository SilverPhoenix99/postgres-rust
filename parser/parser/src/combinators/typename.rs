/// Alias: `Typename`
pub(super) fn typename(stream: &mut TokenStream) -> scan::Result<Type> {

    /*
        ( SETOF )? SimpleTypename opt_array_bounds
    */

    or((
        (Setof, record_typename)
            .map(|(_, typename)| {
                typename.returning_table()
            }),
        record_typename,
    )).parse(stream)
}

fn record_typename(stream: &mut TokenStream) -> scan::Result<Type> {

    /*
        SimpleTypename opt_array_bounds
    */

    let (typename, array_bounds) = (simple_typename, opt_array_bounds)
        .parse(stream)?;

    let typename = typename.with_array_bounds(array_bounds);

    Ok(typename)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::SetOf;
    use pg_ast::TypeName;
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

use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::opt_array_bounds;
use crate::combinators::simple_typename;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::Type;
use pg_lexer::Keyword::Setof;
