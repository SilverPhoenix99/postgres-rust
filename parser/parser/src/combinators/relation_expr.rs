#[derive(Debug, Clone, PartialEq, Eq, Into)]
pub(crate) struct RelationExpr {
    name: RelationName,
    inherited: bool,
}

impl RelationExpr {
    pub fn new<T: Into<RelationName>>(name: T) -> Self {
        Self {
            name: name.into(),
            inherited: true
        }
    }

    pub fn name(&self) -> &RelationName {
        &self.name
    }

    pub fn set_inherited(&mut self, inherited: bool) -> &mut Self {
        self.inherited = inherited;
        self
    }

    pub fn with_inherited(mut self, inherited: bool) -> Self {
        self.inherited = inherited;
        self
    }

    pub fn is_inherited(&self) -> bool {
        self.inherited
    }
}

impl From<RelationName> for RelationExpr {
    fn from(name: RelationName) -> Self {
        Self { name, inherited: true }
    }
}

impl From<Str> for RelationExpr {
    fn from(name: Str) -> Self {
        Self {
            name: name.into(),
            inherited: true
        }
    }
}

impl From<&'static str> for RelationExpr {
    fn from(name: &'static str) -> Self {
        Self {
            name: name.into(),
            inherited: true
        }
    }
}

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

pub(super) fn non_inherited_relation_expr(ctx: &mut ParserContext) -> scan::Result<RelationExpr> {

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
    use test_case::test_matrix;

    #[test_matrix("foo"
        => Ok(RelationExpr::new("foo"))
        ; "inherited without wildcard"
    )]
    #[test_matrix("foo *"
        => Ok(RelationExpr::new("foo"))
        ; "inherited with wildcard"
    )]
    #[test_matrix("only foo"
        => Ok(RelationExpr::new("foo").with_inherited(false))
        ; "non-inherited without parens"
    )]
    #[test_matrix("only(foo)"
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
use derive_more::Into;
use pg_ast::RelationName;
use pg_basics::Str;
use pg_lexer::Keyword::Only;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::Mul;
use pg_parser_core::scan;
