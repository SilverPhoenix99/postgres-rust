pub(super) fn relation_expr_list(ctx: &mut ParserContext) -> scan::Result<Vec<RelationExpr>> {

    /*
        relation_expr ( ',' relation_expr )*
    */

    many!(sep = Comma, relation_expr)
        .parse(ctx)
}

pub(super) fn relation_expr(ctx: &mut ParserContext) -> scan::Result<RelationExpr> {

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

    let expr = RelationExpr::new(name)
        .with_inherited(false);

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

    let expr = RelationExpr::new(name)
        .with_inherited(true);
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    #[allow(unused_imports)]
    use pg_ast::RelationName;
    use test_case::test_case;

    #[test_case("foo"
        => Ok(RelationExpr::new("foo"))
        ; "inherited without wildcard"
    )]
    #[test_case("foo *"
        => Ok(RelationExpr::new("foo"))
        ; "inherited with wildcard"
    )]
    #[test_case("only foo"
        => Ok(RelationExpr::new("foo").with_inherited(false))
        ; "non-inherited without parens"
    )]
    #[test_case("only(foo)"
        => Ok(RelationExpr::new("foo").with_inherited(false))
        ; "non-inherited with parens"
    )]
    fn test_relation_expr(source: &str) -> scan::Result<RelationExpr> {
        test_parser!(source, relation_expr)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::qualified_name;
use crate::many;
use crate::paren;
use crate::seq;
use crate::ParserContext;
use pg_ast::RelationExpr;
use pg_lexer::Keyword::Only;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::Mul;
use pg_parser_core::scan;
