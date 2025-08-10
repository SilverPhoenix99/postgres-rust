pub(super) fn explicit_row(ctx: &mut ParserContext) -> scan::Result<ExprNode> {

    /*
        ROW '(' ( expr_list )? ')'
    */

    let (Keyword(Kw::Row), Operator(OpenParenthesis)) = ctx.stream_mut().peek2()? else {
        return no_match(ctx)
    };

    let expr_list = ctx.expr_list();
    let (_, col_values) = seq!(skip(1), paren!(expr_list.optional()))
        .parse(ctx)?;

    Ok(Row(col_values))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinators::expr_list;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::{IntegerConst, StringConst};
    use test_case::test_case;

    #[test_case("row()" => Ok(Row(None)))]
    #[test_case("row(1)" => Ok(Row(Some(vec![IntegerConst(1)]))))]
    #[test_case("row(1, 'foo')" => Ok(Row(Some(vec![IntegerConst(1), StringConst("foo".into())]))))]
    fn test_explicit_row(source: &str) -> scan::Result<ExprNode> {
        let mut ctx = ParserContext::new(source, expr_list);
        explicit_row(&mut ctx)
    }
}

use crate::no_match;
use pg_ast::ExprNode;
use pg_ast::ExprNode::Row;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::skip;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword as Kw;
use pg_lexer::OperatorKind::OpenParenthesis;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenValue::Keyword;
use pg_parser_core::stream::TokenValue::Operator;
