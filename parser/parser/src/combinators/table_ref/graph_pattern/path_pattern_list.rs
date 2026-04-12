pub(super) fn path_pattern_list(ctx: &mut ParserContext) -> scan::Result<Vec<Vec<GraphElementPatternKind>>> {

    /*
        path_term ( ',' path_term )*
    */

    many!(sep = Comma, path_term).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_path_pattern_list() {
        assert_matches!(
            test_parser!(
                source = "-> <-[]-, - <-",
                parser = path_pattern_list
            ),
            Ok(_)
        )
    }
}

use super::path_term;
use crate::combinators::core::Combinator;
use crate::context::ParserContext;
use crate::many;
use pg_ast::GraphElementPatternKind;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
