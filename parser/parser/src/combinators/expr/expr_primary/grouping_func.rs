pub(super) fn grouping_func(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        GROUPING '(' expr_list ')'
    */

    let (Keyword(Grouping), Operator(OpenParenthesis)) = stream.peek2()? else {
        return no_match(stream);
    };

    let args = skip_prefix(1, expr_list_paren)
        .parse(stream)?;

    Ok(GroupingFunc(args))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::ColumnRef::SingleName;

    #[test]
    fn test_grouping_func() {
        test_parser!(
            source = "grouping(foo)",
            parser = grouping_func,
            expected = GroupingFunc(vec![SingleName("foo".into()).into()])
        )
    }
}

use crate::combinators::expr_list_paren;
use crate::combinators::foundation::skip_prefix;
use crate::combinators::foundation::Combinator;
use crate::no_match;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword;
use crate::stream::TokenValue::Operator;
use pg_ast::ExprNode;
use pg_ast::ExprNode::GroupingFunc;
use pg_lexer::Keyword::Grouping;
use pg_lexer::OperatorKind::OpenParenthesis;
