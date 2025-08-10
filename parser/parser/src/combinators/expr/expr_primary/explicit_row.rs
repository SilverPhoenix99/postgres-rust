pub(super) fn explicit_row(ctx: &mut ParserContext) -> scan::Result<ExprNode> {

    /*
        ROW '(' ( expr_list )? ')'
    */

    let (Keyword(Kw::Row), Operator(OpenParenthesis)) = ctx.stream_mut().peek2()? else {
        return no_match(ctx)
    };

    let (_, col_values) = seq!(skip(1), paren!(expr_list.optional()))
        .parse(ctx)?;

    Ok(Row(col_values))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::{IntegerConst, StringConst};
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("row()", None)]
    #[test_case("row(1)", Some(vec![IntegerConst(1)]))]
    #[test_case("row(1, 'foo')", Some(vec![IntegerConst(1), StringConst("foo".into())]))]
    fn test_explicit_row(source: &str, expected: Option<Vec<ExprNode>>) {
        test_parser!(source, explicit_row, Row(expected))
    }
}

use crate::combinators::expr_list;
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
