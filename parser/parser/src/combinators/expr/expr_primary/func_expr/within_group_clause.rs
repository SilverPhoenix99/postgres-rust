pub(super) fn within_group_clause() -> impl Combinator<Output = Vec<SortBy>> {

    /*
        WITHIN GROUP_P '(' sort_clause ')'
    */

    (Within, Group)
        .and_right(parser(|stream| between!(paren : stream =>
            sort_clause(stream)
        )))
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

use crate::combinators::foundation::between;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::combinators::sort_clause;
use pg_ast::SortBy;
use pg_lexer::Keyword::Group;
use pg_lexer::Keyword::Within;
