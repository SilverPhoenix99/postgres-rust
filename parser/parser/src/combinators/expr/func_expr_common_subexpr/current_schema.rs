pub(super) fn current_schema(ctx: &mut ParserContext) -> scan::Result<SqlFunction> {

    // `current_schema()` is valid syntax for `prefix_expr`, so exclude that case from here.
    if matches!(ctx.stream_mut().peek2(), Ok((K(Kw::CurrentSchema), Op(OpenParenthesis)))) {
        return no_match(ctx)
    }

    // If we reach here, it could be that there are 1 or fewer tokens left in the stream,
    // or there are more tokens, but they didn't match any of the above patterns.

    Kw::CurrentSchema.parse(ctx)?;
    Ok(CurrentSchema)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    #[allow(unused_imports)]
    use scan::Error::NoMatch;
    use test_case::test_case;

    #[test_case("current_schema 1" => Ok(CurrentSchema))]
    #[test_case("current_schema" => Ok(CurrentSchema))]
    #[test_case("current_schema(" => matches Err(NoMatch(_)))]
    fn test_current_schema(source: &str) -> scan::Result<SqlFunction> {
        test_parser!(source, current_schema)
    }
}

use crate::no_match;
use pg_ast::SqlFunction;
use pg_ast::SqlFunction::CurrentSchema;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword as Kw;
use pg_lexer::OperatorKind::OpenParenthesis;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenValue::Keyword as K;
use pg_parser_core::stream::TokenValue::Operator as Op;
