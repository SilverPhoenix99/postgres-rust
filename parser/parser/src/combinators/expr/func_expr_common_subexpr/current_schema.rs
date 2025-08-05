pub(super) fn current_schema(stream: &mut TokenStream) -> scan::Result<SqlFunction> {

    // `current_schema()` is valid syntax for `prefix_expr`, so exclude that case from here.
    if matches!(stream.peek2(), Ok((K(Kw::CurrentSchema), Op(OpenParenthesis)))) {
        return no_match(stream)
    }

    // If we reach here, it could be that there are 1 or fewer tokens left in the stream,
    // or there are more tokens, but they didn't match any of the above patterns.

    Kw::CurrentSchema.parse(stream)?;
    Ok(CurrentSchema)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
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

use crate::combinators::foundation::Combinator;
use crate::no_match;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword as K;
use crate::stream::TokenValue::Operator as Op;
use pg_ast::SqlFunction;
use pg_ast::SqlFunction::CurrentSchema;
use pg_lexer::Keyword as Kw;
use pg_lexer::OperatorKind::OpenParenthesis;
