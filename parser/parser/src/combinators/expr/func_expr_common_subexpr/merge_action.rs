pub(super) fn merge_action(ctx: &mut ParserContext) -> scan::Result<SqlFunction> {

    /*
        MERGE_ACTION '(' ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    seq!(skip(2), CloseParenthesis).parse(ctx)?;
    Ok(MergeAction)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;

    #[test_case("merge_action()" => Ok(MergeAction))]
    fn test_merge_action(source: &str) -> scan::Result<SqlFunction> {
        test_parser!(source, merge_action)
    }
}

use crate::combinators::core::skip;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_ast::SqlFunction;
use pg_ast::SqlFunction::MergeAction;
use pg_lexer::OperatorKind::CloseParenthesis;
use pg_parser_core::scan;
