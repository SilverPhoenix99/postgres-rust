pub(super) fn where_clause(ctx: &mut ParserContext) -> scan::Result<ExprNode> {

    /*
        WHERE a_expr
    */

    let (_, expr) = seq!(Where, a_expr).parse(ctx)?;

    Ok(expr)
}

use crate::combinators::core::Combinator;
use crate::combinators::expr::a_expr;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::ExprNode;
use pg_lexer::Keyword::Where;
use pg_parser_core::scan;
