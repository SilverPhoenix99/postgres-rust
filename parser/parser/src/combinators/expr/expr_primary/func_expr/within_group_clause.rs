pub(in crate::combinators::expr::expr_primary)
fn within_group_clause(ctx: &mut ParserContext) -> scan::Result<Vec<SortBy>> {

    /*
        WITHIN GROUP_P '(' sort_clause ')'
    */

    let (.., Located(sorts, _)) = seq!(Within, Group, paren!(sort_clause))
        .parse(ctx)?;

    Ok(sorts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
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

use crate::combinators::core::Combinator;
use crate::combinators::sort_clause;
use crate::paren;
use crate::seq;
use crate::ParserContext;
use pg_ast::SortBy;
use pg_basics::Located;
use pg_lexer::Keyword::Group;
use pg_lexer::Keyword::Within;
use pg_parser_core::scan;
