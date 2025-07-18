pub(in crate::combinators::expr::expr_primary)
fn filter_clause(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        FILTER '(' WHERE a_expr ')'
    */

    let (_, (_, expr)) = (
        Filter,
        between_paren((Where, a_expr))
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
use crate::combinators::foundation::between_paren;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
use pg_lexer::Keyword::Filter;
use pg_lexer::Keyword::Where;
