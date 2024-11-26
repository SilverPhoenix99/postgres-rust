/// Alias: `Typename`
pub(super) fn typename() -> impl Combinator<Output = Type> {

    /*
        ( SETOF )? SimpleTypename opt_array_bounds
    */

    Setof.chain_result(|setof, stream| {
        if setof.is_ok() {
            simple_typename().required()
                .and_then(
                    array_bounds(),
                    |typename, array_bounds| {
                        typename.returning_table()
                            .with_array_bounds(array_bounds)
                    }
                )
                .parse(stream)
        }
        else {
            simple_typename()
                .and_then(
                    array_bounds(),
                    |typename, array_bounds| {
                        typename.with_array_bounds(array_bounds)
                    }
                )
                .parse(stream)
        }
    })
}

/// `Vec` **May** be empty
fn array_bounds() -> impl Combinator<Output = Vec<Option<i32>>> {
    opt_array_bounds()
        .optional()
        .map(Option::unwrap_or_default)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use crate::parser::ast_node::{SetOf, TypeName};
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test_case("int", Type::new(TypeName::Int4, vec![], SetOf::Record))]
    #[test_case("int[]", Type::new(TypeName::Int4, vec![None], SetOf::Record))]
    #[test_case("setof int", Type::new(TypeName::Int4, vec![], SetOf::Table))]
    #[test_case("setof int[]", Type::new(TypeName::Int4, vec![None], SetOf::Table))]
    #[test_case("setof double precision[10][]", Type::new(TypeName::Float8, vec![Some(10), None], SetOf::Table))]
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
                type_modifiers: vec![],
            },
            vec![],
            SetOf::Table,
        );

        assert_eq!(Ok(expected), typename().parse(&mut stream));
    }
}

use crate::lexer::Keyword::Setof;
use crate::parser::ast_node::Type;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::opt_array_bounds::opt_array_bounds;
use crate::parser::simple_typename::simple_typename;
