pub(super) fn create_cast_stmt(stream: &mut TokenStream) -> scan::Result<CreateCastStmt> {

    /*
        typecast cast_conversion cast_context
    */

    let (typecast, conversion, coercion) = (
        typecast,
        cast_conversion,
        cast_context.optional()
            .map(Option::unwrap_or_default)
    ).parse(stream)?;

    let stmt = CreateCastStmt::new(typecast, conversion, coercion);
    Ok(stmt)
}

fn cast_conversion(stream: &mut TokenStream) -> scan::Result<CastConversion> {

    /*
          WITH FUNCTION function_with_argtypes
        | WITH INOUT
        | WITHOUT FUNCTION
    */

    or((
        (
            With,
            or((
                Inout.map(|_| WithInout),
                (Function, function_with_argtypes)
                    .map(|(_, signature)| WithFunction(signature))
            ))
        )
            .map(|(_, conversion)| conversion),
        (Without, Function).map(|_| WithoutFunction),
    )).parse(stream)
}

fn cast_context(stream: &mut TokenStream) -> scan::Result<CoercionContext> {

    /*
          AS (IMPLICIT | ASSIGNMENT)
    */

    let (_, context) = (
        As,
        or((
            Kw::Implicit.map(|_| CoercionContext::Implicit),
            Kw::Assignment.map(|_| CoercionContext::Assignment)
        ))
    ).parse(stream)?;

    Ok(context)
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
            parser = create_cast_stmt,
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
        test_parser!(source, cast_conversion, expected);
    }

    #[test_case("as implicit", CoercionContext::Implicit)]
    #[test_case("as assignment", CoercionContext::Assignment)]
    fn test_cast_context(source: &str, expected: CoercionContext) {
        test_parser!(source, cast_context, expected);
    }
}

use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::function_with_argtypes;
use crate::combinators::stmt::typecast;
use crate::scan;
use crate::stream::TokenStream;
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
