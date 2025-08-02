pub(super) fn relation_expr_list(stream: &mut TokenStream) -> scan::Result<Vec<RelationExpr>> {

    /*
        relation_expr ( ',' relation_expr )*
    */

    many_sep(Comma, relation_expr)
        .parse(stream)
}

pub(super) fn relation_expr(stream: &mut TokenStream) -> scan::Result<RelationExpr> {

    /*
          non_inherited_relation_expr
        | inherited_relation_expr
    */

    or((
        non_inherited_relation_expr,
        inherited_relation_expr
    )).parse(stream)
}

fn non_inherited_relation_expr(stream: &mut TokenStream) -> scan::Result<RelationExpr> {

    /*
          ONLY qualified_name
        | ONLY '(' qualified_name ')'
    */

    let (_, name) = (
        Only,
        or((
            between_paren(qualified_name),
            qualified_name
        ))
    ).parse(stream)?;

    let expr = RelationExpr::new(name, false);
    Ok(expr)
}

fn inherited_relation_expr(stream: &mut TokenStream) -> scan::Result<RelationExpr> {

    /*
        qualified_name ( '*' )?
    */

    let (name, _) = (qualified_name, Mul.optional())
        .parse(stream)?;

    let expr = RelationExpr::new(name, true);
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::RelationName;
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

use crate::combinators::foundation::between_paren;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::qualified_name;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::RelationExpr;
use pg_lexer::Keyword::Only;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::Mul;
