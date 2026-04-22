pub(super) fn exists_expr(ctx: &mut ParserContext) -> scan::Result<ExprNode> {

    /*
        EXISTS '(' SelectStmt ')'
    */

    if !matches!(ctx.stream_mut().peek_n::<2>(), Ok([Keyword(Exists), Operator(OpenParenthesis)])) {
        return no_match(ctx)
    }

    let (_, stmt) = seq!(skip(1), paren!(select_stmt)).parse(ctx)?;

    Ok(ExprNode::Exists(stmt))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;

    #[test_case("exists (select 1)" => ignore["select_stmt not implemented yet"] matches Ok(_))]
    fn test_exists_expr(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, exists_expr)
    }
}

use crate::combinators::core::skip;
use crate::combinators::core::Combinator;
use crate::combinators::stmt::select_stmt;
use crate::context::ParserContext;
use crate::no_match;
use crate::paren;
use crate::seq;
use pg_ast::ExprNode;
use pg_lexer::Keyword::Exists;
use pg_lexer::OperatorKind::OpenParenthesis;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenValue::Keyword;
use pg_parser_core::stream::TokenValue::Operator;
