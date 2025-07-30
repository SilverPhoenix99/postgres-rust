pub(super) fn merge_action(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        MERGE_ACTION '(' ')'
    */

    if ! matches!(stream.peek2(), Ok((K(Kw::MergeAction), Op(OpenParenthesis)))) {
        return no_match(stream)
    }

    skip_prefix(2, CloseParenthesis).parse(stream)?;
    Ok(MergeAction)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use scan::Error::NoMatch;
    use test_case::test_case;

    #[test_case("merge_action()" => Ok(MergeAction))]
    #[test_case("merge_action" => matches Err(NoMatch(_)))]
    #[test_case("merge_action 1" => matches Err(NoMatch(_)))]
    fn test_merge_action(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, merge_action)
    }
}

use crate::combinators::foundation::skip_prefix;
use crate::combinators::foundation::Combinator;
use crate::no_match;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword as K;
use crate::stream::TokenValue::Operator as Op;
use pg_ast::ExprNode;
use pg_ast::ExprNode::MergeAction;
use pg_lexer::Keyword as Kw;
use pg_lexer::OperatorKind::CloseParenthesis;
use pg_lexer::OperatorKind::OpenParenthesis;
