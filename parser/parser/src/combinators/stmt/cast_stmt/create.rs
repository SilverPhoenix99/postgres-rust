pub(in crate::combinators::stmt) fn create_cast_stmt(ctx: &mut ParserContext) -> scan::Result<CreateCastStmt> {

    /*
        typecast cast_conversion cast_context
    */

    let (typecast, conversion, coercion) = seq!(
        typecast,
        cast_conversion,
        cast_context.optional()
            .map(Option::unwrap_or_default)
    ).parse(ctx)?;

    let stmt = CreateCastStmt::new(typecast, conversion, coercion);
    Ok(stmt)
}

fn cast_conversion(ctx: &mut ParserContext) -> scan::Result<CastConversion> {

    /*
          WITH FUNCTION function_with_argtypes
        | WITH INOUT
        | WITHOUT FUNCTION
    */

    alt!(
        seq!(
            With,
            alt!(
                Inout.map(|_| WithInout),
                seq!(Function, function_with_argtypes)
                    .map(|(_, signature)| WithFunction(signature))
            )
        )
            .map(|(_, conversion)| conversion),
        seq!(Without, Function).map(|_| WithoutFunction),
    ).parse(ctx)
}

fn cast_context(ctx: &mut ParserContext) -> scan::Result<CoercionContext> {

    /*
          AS (IMPLICIT | ASSIGNMENT)
    */

    let (_, context) = seq!(
        As,
        alt!(
            Kw::Implicit.map(|_| Implicit),
            Kw::Assignment.map(|_| Assignment)
        )
    ).parse(ctx)?;

    Ok(context)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::FunctionWithArgs;
    use pg_ast::TypeName::Int4;
    use pg_ast::TypeName::Int8;
    use pg_ast::Typecast;
    use test_case::test_matrix;

    #[test]
    fn test_create_cast_stmt() {
        test_parser!(
            source = "cast (int as bigint) without function as assignment",
            parser = create_cast_stmt,
            expected = CreateCastStmt::new(
                Typecast::new(Int4, Int8),
                WithoutFunction,
                Assignment
            )
        )
    }

    #[test_matrix("with inout" => Ok(WithInout))]
    #[test_matrix("with function foo" => Ok(WithFunction(FunctionWithArgs::new(vec!["foo".into()], None))))]
    #[test_matrix("without function" => Ok(WithoutFunction))]
    fn test_cast_conversion(source: &str) -> scan::Result<CastConversion> {
        test_parser!(source, cast_conversion)
    }

    #[test_matrix("as implicit" => Ok(Implicit))]
    #[test_matrix("as assignment" => Ok(Assignment))]
    fn test_cast_context(source: &str) -> scan::Result<CoercionContext> {
        test_parser!(source, cast_context)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::function_with_argtypes;
use crate::combinators::stmt::typecast;
use crate::seq;
use crate::ParserContext;
use pg_ast::CastConversion;
use pg_ast::CastConversion::WithFunction;
use pg_ast::CastConversion::WithInout;
use pg_ast::CastConversion::WithoutFunction;
use pg_ast::CoercionContext;
use pg_ast::CoercionContext::Assignment;
use pg_ast::CoercionContext::Implicit;
use pg_ast::CreateCastStmt;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::As;
use pg_lexer::Keyword::Function;
use pg_lexer::Keyword::Inout;
use pg_lexer::Keyword::With;
use pg_lexer::Keyword::Without;
use pg_parser_core::scan;
