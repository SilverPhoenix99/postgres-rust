pub(in crate::combinators::expr::expr_primary)
fn within_group_clause(stream: &mut TokenStream) -> scan::Result<Vec<SortBy>> {

    /*
        WITHIN GROUP_P '(' sort_clause ')'
    */

    let (.., (sorts, _)) = seq!(Within, Group, paren(sort_clause))
        .parse(stream)?;

    Ok(sorts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::ExprNode::IntegerConst;

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

use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::sort_clause;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::SortBy;
use pg_lexer::Keyword::Group;
use pg_lexer::Keyword::Within;
