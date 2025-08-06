pub(super) fn grouping_func(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        GROUPING '(' expr_list ')'
    */

    let (Keyword(Grouping), Operator(OpenParenthesis)) = stream.peek2()? else {
        return no_match(stream);
    };

    let (_, args) = seq!(skip(1), paren!(expr_list))
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

use crate::combinators::expr_list::expr_list;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use crate::combinators::foundation::Combinator;
use crate::no_match;
use pg_ast::ExprNode;
use pg_ast::ExprNode::GroupingFunc;
use pg_lexer::Keyword::Grouping;
use pg_lexer::OperatorKind::OpenParenthesis;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
use pg_parser_core::stream::TokenValue::Keyword;
use pg_parser_core::stream::TokenValue::Operator;
