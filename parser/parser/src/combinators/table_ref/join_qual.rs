pub(super) fn join_qual(ctx: &mut ParserContext) -> scan::Result<(JoinQual, Option<Alias>)> {

    /*
        USING '(' name_list ')' ( alias_clause )?
      | ON a_expr
    */

    alt!(
        seq!(Kw::Using, paren!(name_list), alias_clause.optional())
            .map(|(_, name, alias)| {
                (JoinQual::Using(name), alias)
            }),
        seq!(Kw::On, a_expr)
            .map(|(_, expr)|
                (JoinQual::On(Box::new(expr)), None)
            )
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode::BooleanConst;
    use test_case::test_matrix;

    #[test_matrix("using (foo, bar) as qux" => Ok((
        JoinQual::Using(vec!["foo".into(), "bar".into()]),
        Some("qux".into())
    )))]
    #[test_matrix("using (baz)" => Ok((
        JoinQual::Using(vec!["baz".into()]),
        None
    )))]
    #[test_matrix("on true" => Ok((
        JoinQual::On(Box::new(BooleanConst(true))),
        None
    )))]
    fn test_join_qual(source: &str) -> scan::Result<(JoinQual, Option<Alias>)> {
        test_parser!(source, join_qual)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::expr::a_expr;
use crate::combinators::name_list;
use crate::combinators::table_ref::alias_clause;
use crate::context::ParserContext;
use crate::paren;
use crate::seq;
use pg_ast::Alias;
use pg_ast::JoinQual;
use pg_lexer::Keyword as Kw;
use pg_parser_core::scan;
