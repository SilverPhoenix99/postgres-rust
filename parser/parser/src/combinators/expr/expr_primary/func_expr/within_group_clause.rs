pub(super) fn within_group_clause() -> impl Combinator<Output = Vec<SortBy>> {

    /*
        WITHIN GROUP_P '(' sort_clause ')'
    */

    and(Within, Group)
        .and_right(between_paren(
            sort_clause()
        ))
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
            parser = within_group_clause(),
            expected = vec![
                SortBy::new(IntegerConst(1), None, None)
            ]
        );
    }
}

use crate::combinators::between_paren;
use crate::combinators::foundation::and;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::sort_clause;
use pg_ast::SortBy;
use pg_lexer::Keyword::Group;
use pg_lexer::Keyword::Within;
