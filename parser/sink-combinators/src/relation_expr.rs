pub fn relation_expr_list(ctx: &mut ParserContext) -> scan::Result<Vec<RelationExpr>> {

    /*
        relation_expr ( ',' relation_expr )*
    */

    many!(sep = Comma, relation_expr)
        .parse(ctx)
}

pub fn relation_expr(ctx: &mut ParserContext) -> scan::Result<RelationExpr> {

    /*
          non_inherited_relation_expr
        | inherited_relation_expr
    */

    alt!(
        non_inherited_relation_expr,
        inherited_relation_expr
    ).parse(ctx)
}

fn non_inherited_relation_expr(ctx: &mut ParserContext) -> scan::Result<RelationExpr> {

    /*
          ONLY '(' qualified_name ')'
        | ONLY qualified_name
    */

    let (_, name) = seq!(
        Only,
        alt!(
            paren!(qualified_name),
            qualified_name
        )
    ).parse(ctx)?;

    let expr = RelationExpr::new(name, false);
    Ok(expr)
}

fn inherited_relation_expr(ctx: &mut ParserContext) -> scan::Result<RelationExpr> {

    /*
        qualified_name ( '*' )?
    */

    let (name, _) = seq!(
        qualified_name,
        Mul.optional()
    ).parse(ctx)?;

    let expr = RelationExpr::new(name, true);
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    #[allow(unused_imports)]
    use pg_sink_ast::RelationName;
    use test_case::test_case;

    #[test_case("foo"
        => Ok(RelationExpr::new(RelationName::new("foo", None), true))
        ; "inherited without wildcard"
    )]
    #[test_case("foo *"
        => Ok(RelationExpr::new(RelationName::new("foo", None), true))
        ; "inherited with wildcard"
    )]
    #[test_case("only foo"
        => Ok(RelationExpr::new(RelationName::new("foo", None), false))
        ; "non-inherited without parens"
    )]
    #[test_case("only(foo)"
        => Ok(RelationExpr::new(RelationName::new("foo", None), false))
        ; "non-inherited with parens"
    )]
    fn test_relation_expr(source: &str) -> scan::Result<RelationExpr> {
        test_parser!(source, relation_expr)
    }
}

use crate::qualified_name;
use pg_combinators::alt;
use pg_combinators::many;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Only;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::Mul;
use pg_parser_core::scan;
use pg_combinators::ParserContext;
use pg_sink_ast::RelationExpr;
