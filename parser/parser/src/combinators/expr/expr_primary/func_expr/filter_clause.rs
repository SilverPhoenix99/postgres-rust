pub(in crate::combinators::expr::expr_primary)
fn filter_clause(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        FILTER '(' WHERE a_expr ')'
    */

    let (_, (_, expr)) = seq!(
        Filter,
        paren!(seq!(Where, a_expr))
    ).parse(stream)?;

    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_filter_clause() {
        test_parser!(
            source = "filter (where true)",
            parser = filter_clause,
            expected = ExprNode::BooleanConst(true)
        )
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use pg_ast::ExprNode;
use pg_lexer::Keyword::Filter;
use pg_lexer::Keyword::Where;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
