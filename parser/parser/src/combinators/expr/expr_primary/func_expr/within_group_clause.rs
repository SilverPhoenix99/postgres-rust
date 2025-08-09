pub(in crate::combinators::expr::expr_primary)
fn within_group_clause(stream: &mut TokenStream) -> scan::Result<Vec<SortBy>> {

    /*
        WITHIN GROUP_P '(' sort_clause ')'
    */

    let (.., Located(sorts, _)) = seq!(Within, Group, paren!(sort_clause))
        .parse(stream)?;

    Ok(sorts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_ast::ExprNode::IntegerConst;
    use pg_combinators::test_parser;

    #[test]
    fn test_within_group_clause() {
        test_parser!(
            source = "within group (order by 1)",
            parser = within_group_clause,
            expected = vec![
                SortBy::new(IntegerConst(1), None, None)
            ]
        );
    }
}

use crate::combinators::sort_clause;
use pg_ast::SortBy;
use pg_basics::Located;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Group;
use pg_lexer::Keyword::Within;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
