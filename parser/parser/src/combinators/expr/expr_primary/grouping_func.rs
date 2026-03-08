pub(super) fn grouping_func(ctx: &mut ParserContext) -> scan::Result<ExprNode> {

    /*
        GROUPING '(' expr_list ')'
    */

    let (Keyword(Grouping), Operator(OpenParenthesis)) = ctx.stream_mut().peek2()? else {
        return no_match(ctx);
    };

    let (_, args) = seq!(skip(1), paren!(expr_list))
        .parse(ctx)?;

    Ok(GroupingFunc(args))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_ast::ColumnRef::SingleName;

    #[test]
    fn test_grouping_func() {
        let mut ctx = ParserContext::new("grouping(foo)");
        let actual = grouping_func(&mut ctx);
        assert_eq!(Ok(GroupingFunc(vec![SingleName("foo".into()).into()])), actual)
    }
}

use crate::combinators::core::skip;
use crate::combinators::core::Combinator;
use crate::combinators::expr_list;
use crate::no_match;
use crate::paren;
use crate::seq;
use crate::ParserContext;
use pg_ast::ExprNode;
use pg_ast::ExprNode::GroupingFunc;
use pg_lexer::Keyword::Grouping;
use pg_lexer::OperatorKind::OpenParenthesis;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenValue::Keyword;
use pg_parser_core::stream::TokenValue::Operator;
