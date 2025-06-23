/// Alias: `Typename`
pub(super) fn typename() -> impl Combinator<Output = Type> {

    /*
        ( SETOF )? SimpleTypename opt_array_bounds
    */

    match_first! {
        (Setof, record_typename())
            .map(|(_, typename)| {
                typename.returning_table()
            }),
        record_typename(),
    }
}

fn record_typename() -> impl Combinator<Output = Type> {

    /*
        SimpleTypename opt_array_bounds
    */

    (simple_typename(), opt_array_bounds())
        .map(|(typename, array_bounds)| {
            typename.with_array_bounds(array_bounds)
        })
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use pg_ast::Type;
    #[allow(unused_imports)]
    use pg_ast::{SetOf, TypeName};
    use test_case::test_case;

    #[test_case("int", Type::new(TypeName::Int4, None, SetOf::Record))]
    #[test_case("int[]", Type::new(TypeName::Int4, Some(vec![None]), SetOf::Record))]
    #[test_case("setof int", Type::new(TypeName::Int4, None, SetOf::Table))]
    #[test_case("setof int[]", Type::new(TypeName::Int4, Some(vec![None]), SetOf::Table))]
    #[test_case("setof double precision[10][]", Type::new(TypeName::Float8, Some(vec![Some(10), None]), SetOf::Table))]
    fn test_typename(source: &str, expected: Type) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(expected), typename().parse(&mut stream));
    }

    #[test]
    fn test_generic_typename() {
        let source = "setof some_.qualified_name";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = Type::new(
            TypeName::Generic {
                name: vec!["some_".into(), "qualified_name".into()],
                type_modifiers: None,
            },
            None,
            SetOf::Table,
        );

        assert_eq!(Ok(expected), typename().parse(&mut stream));
    }
}

use crate::combinators::foundation::match_first;
use crate::combinators::foundation::Combinator;
use crate::combinators::opt_array_bounds;
use crate::combinators::simple_typename;
use pg_ast::Type;
use pg_lexer::Keyword::Setof;
