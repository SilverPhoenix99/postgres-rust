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
    use pg_combinators::test_parser;

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
use crate::no_match;
use pg_ast::ExprNode;
use pg_ast::ExprNode::GroupingFunc;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::skip;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Grouping;
use pg_lexer::OperatorKind::OpenParenthesis;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenValue::Keyword;
use pg_parser_core::stream::TokenValue::Operator;
