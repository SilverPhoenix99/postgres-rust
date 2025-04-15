pub(super) fn create_cast_stmt() -> impl Combinator<Output = CreateCastStmt> {

    /*
        typecast cast_conversion cast_context
    */

    sequence!(typecast(), cast_conversion(), cast_context())
        .map(|(typecast, conversion, coercion)|
            CreateCastStmt::new(typecast, conversion, coercion)
        )
}

fn cast_conversion() -> impl Combinator<Output = CastConversion> {

    /*
          WITH FUNCTION function_with_argtypes
        | WITH INOUT
        | WITHOUT FUNCTION
    */

    or(
        With
            .and_right(or(
                Inout.map(|_| WithInout),
                Function
                    .and_right(function_with_argtypes())
                    .map(WithFunction)
            )),
        and(Without, Function).map(|_| WithoutFunction),

    )
}

fn cast_context() -> impl Combinator<Output = CoercionContext> {

    /*
          ( AS (IMPLICIT | ASSIGNMENT) )?
    */

    As.and_right(or(
        Kw::Implicit.map(|_| CoercionContext::Implicit),
        Kw::Assignment.map(|_| CoercionContext::Assignment)
    ))
        .optional()
        .map(Option::unwrap_or_default)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use crate::parser::ast_node::FunctionWithArgs;
    use crate::parser::ast_node::{TypeName, Typecast};
    use crate::parser::tests::test_parser;
    use test_case::test_case;
    use TypeName::{Int4, Int8};

    #[test]
    fn test_create_cast_stmt() {
        test_parser!(
            source = "cast (int as bigint) without function as assignment",
            parser = create_cast_stmt(),
            expected = CreateCastStmt::new(
                Typecast::new(Int4, Int8),
                WithoutFunction,
                CoercionContext::Assignment
            )
        )
    }

    #[test_case("with inout", WithInout)]
    #[test_case("with function foo", WithFunction(FunctionWithArgs::new(vec!["foo".into()], None)))]
    #[test_case("without function", WithoutFunction)]
    fn test_cast_conversion(source: &str, expected: CastConversion) {
        test_parser!(source, cast_conversion(), expected);
    }

    #[test_case("as implicit", CoercionContext::Implicit)]
    #[test_case("as assignment", CoercionContext::Assignment)]
    #[test_case("", CoercionContext::Explicit)]
    #[test_case("something else", CoercionContext::Explicit)]
    fn test_cast_context(source: &str, expected: CoercionContext) {
        test_parser!(source, cast_context(), expected);
    }
}

use crate::lexer::Keyword as Kw;
use crate::lexer::Keyword::As;
use crate::lexer::Keyword::Function;
use crate::lexer::Keyword::Inout;
use crate::lexer::Keyword::With;
use crate::lexer::Keyword::Without;
use crate::parser::ast_node::CastConversion;
use crate::parser::ast_node::CastConversion::WithFunction;
use crate::parser::ast_node::CastConversion::WithInout;
use crate::parser::ast_node::CastConversion::WithoutFunction;
use crate::parser::ast_node::CoercionContext;
use crate::parser::ast_node::CreateCastStmt;
use crate::parser::combinators::foundation::and;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::function_with_argtypes;
use crate::parser::combinators::stmt::typecast;
