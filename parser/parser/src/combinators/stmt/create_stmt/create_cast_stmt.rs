pub(super) fn create_cast_stmt() -> impl Combinator<Output = CreateCastStmt> {

    /*
        typecast cast_conversion cast_context
    */

    (typecast(), cast_conversion(), cast_context())
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
        (Without, Function).map(|_| WithoutFunction),

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
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::FunctionWithArgs;
    use pg_ast::TypeName::Int4;
    use pg_ast::TypeName::Int8;
    use pg_ast::Typecast;
    use test_case::test_case;

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

use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::function_with_argtypes;
use crate::combinators::stmt::typecast;
use pg_ast::CastConversion;
use pg_ast::CastConversion::WithFunction;
use pg_ast::CastConversion::WithInout;
use pg_ast::CastConversion::WithoutFunction;
use pg_ast::CoercionContext;
use pg_ast::CreateCastStmt;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::As;
use pg_lexer::Keyword::Function;
use pg_lexer::Keyword::Inout;
use pg_lexer::Keyword::With;
use pg_lexer::Keyword::Without;
