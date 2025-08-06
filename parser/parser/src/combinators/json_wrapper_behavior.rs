pub(super) fn json_wrapper_behavior(stream: &mut TokenStream) -> scan::Result<JsonWrapperBehavior> {

    /*
        (* `ARRAY` is a noise word *)

          WITHOUT ( ARRAY )? WRAPPER
        | WITH CONDITIONAL ( ARRAY )? WRAPPER
        | WITH ( UNCONDITIONAL )? ( ARRAY )? WRAPPER
    */

    let (behavior, ..) = seq!(wrapper_behavior, Array.optional(), Wrapper)
        .parse(stream)?;

    Ok(behavior)
}

fn wrapper_behavior(stream: &mut TokenStream) -> scan::Result<JsonWrapperBehavior> {

    alt!(
        Kw::Without.map(|_| Without),
        seq!(
            With,
            alt!(
                Kw::Conditional.map(|_| Conditional),
                Kw::Unconditional.optional().map(|_| Unconditional),
            )
        ).map(|(_, behavior)| behavior),
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
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

use pg_ast::JsonWrapperBehavior;
use pg_ast::JsonWrapperBehavior::Conditional;
use pg_ast::JsonWrapperBehavior::Unconditional;
use pg_ast::JsonWrapperBehavior::Without;
use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Array;
use pg_lexer::Keyword::With;
use pg_lexer::Keyword::Wrapper;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
