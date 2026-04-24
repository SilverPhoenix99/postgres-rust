#[derive(Debug, PartialEq)]
pub(super) struct SampleClause {
    pub function_name: QualifiedName,
    pub args: Vec<ExprNode>,
    pub repeatable_expr: Option<ExprNode>,
}

pub(super) fn tablesample_clause(ctx: &mut ParserContext) -> scan::Result<SampleClause> {

    /*
        TABLESAMPLE func_name '(' expr_list ')' ( repeatable_clause )?
    */

    let (_, function_name, args, repeatable_expr) = seq!(
        Tablesample,
        func_name,
        paren!(expr_list),
        repeatable_clause.optional()
    ).parse(ctx)?;

    Ok(SampleClause {
        function_name,
        args,
        repeatable_expr
    })
}

/// Alias: `opt_repeatable_clause`
fn repeatable_clause(ctx: &mut ParserContext) -> scan::Result<ExprNode> {

    /*
        REPEATABLE '(' a_expr ')'
    */

    let (_, expr) = seq!(Repeatable, paren!(a_expr)).parse(ctx)?;

    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode::IntegerConst;
    use test_case::test_matrix;

    #[test_matrix("tablesample foo(1) repeatable (10)" => Ok(
        SampleClause {
            function_name: vec!["foo".into()],
            args: vec![IntegerConst(1)],
            repeatable_expr: Some(IntegerConst(10)),
        }
    ))]
    #[test_matrix("tablesample bar(2)" => Ok(
        SampleClause {
            function_name: vec!["bar".into()],
            args: vec![IntegerConst(2)],
            repeatable_expr: None,
        }
    ))]
    fn test_tablesample_clause(source: &str) -> scan::Result<SampleClause> {
        test_parser!(source, tablesample_clause)
    }

    #[test_matrix("repeatable (1)" => Ok(IntegerConst(1)))]
    fn test_repeatable_clause(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, repeatable_clause)
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::expr::a_expr;
use crate::combinators::expr_list;
use crate::combinators::func_name;
use crate::context::ParserContext;
use crate::paren;
use crate::seq;
use pg_ast::ExprNode;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Repeatable;
use pg_lexer::Keyword::Tablesample;
use pg_parser_core::scan;
