pub(super) fn path_term(ctx: &mut ParserContext) -> scan::Result<Vec<GraphElementPatternKind>> {

    /*
        ( path_factor )+
    */

    many!(path_factor).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;

    #[test_case("->" => matches Ok(_); "one")]
    #[test_case("-[]-> <- -[]- -" => matches Ok(_); "four")]
    fn test_path_term(source: &str) -> scan::Result<Vec<GraphElementPatternKind>> {
        test_parser!(source, path_term)
    }
}

use super::path_factor;
use crate::combinators::core::Combinator;
use crate::context::ParserContext;
use crate::many;
use pg_ast::GraphElementPatternKind;
use pg_parser_core::scan;
