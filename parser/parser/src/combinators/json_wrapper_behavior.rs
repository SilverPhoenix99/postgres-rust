pub(super) fn json_wrapper_behavior(stream: &mut TokenStream) -> scan::Result<JsonWrapperBehavior> {

    /*
        (* `ARRAY` is a noise word *)

          WITHOUT ( ARRAY )? WRAPPER
        | WITH CONDITIONAL ( ARRAY )? WRAPPER
        | WITH ( UNCONDITIONAL )? ( ARRAY )? WRAPPER
    */

    let (behavior, ..) = (wrapper_behavior, Array.optional(), Wrapper)
        .parse(stream)?;

    Ok(behavior)
}

fn wrapper_behavior(stream: &mut TokenStream) -> scan::Result<JsonWrapperBehavior> {

    or((
        Kw::Without.map(|_| Without),
        (
            With,
            or((
                Kw::Conditional.map(|_| Conditional),
                Kw::Unconditional.optional().map(|_| Unconditional),
            ))
        ).map(|(_, behavior)| behavior),
    )).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("without wrapper" => Ok(Without))]
    #[test_case("with conditional array wrapper" => Ok(Conditional))]
    fn test_json_wrapper_behavior(source: &str) -> scan::Result<JsonWrapperBehavior> {
        test_parser!(source, json_wrapper_behavior)
    }

    #[test_case("without" => Ok(Without))]
    #[test_case("with conditional" => Ok(Conditional))]
    #[test_case("with" => Ok(Unconditional))]
    #[test_case("with unconditional" => Ok(Unconditional))]
    fn test_wrapper_behavior(source: &str) -> scan::Result<JsonWrapperBehavior> {
        test_parser!(source, wrapper_behavior)
    }
}

use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::JsonWrapperBehavior;
use pg_ast::JsonWrapperBehavior::Conditional;
use pg_ast::JsonWrapperBehavior::Unconditional;
use pg_ast::JsonWrapperBehavior::Without;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Array;
use pg_lexer::Keyword::With;
use pg_lexer::Keyword::Wrapper;
