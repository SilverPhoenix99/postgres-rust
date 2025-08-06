pub(super) fn merge_action(stream: &mut TokenStream) -> scan::Result<SqlFunction> {

    /*
        MERGE_ACTION '(' ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    seq!(skip(2), CloseParenthesis).parse(stream)?;
    Ok(MergeAction)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("merge_action()" => Ok(MergeAction))]
    fn test_merge_action(source: &str) -> scan::Result<SqlFunction> {
        test_parser!(source, merge_action)
    }
}

use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use crate::combinators::foundation::Combinator;
use pg_ast::SqlFunction;
use pg_ast::SqlFunction::MergeAction;
use pg_lexer::OperatorKind::CloseParenthesis;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
