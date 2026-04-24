pub(super) fn labeled_expr_list(ctx: &mut ParserContext) -> scan::Result<Vec<NamedValue>> {

    /*
        labeled_expr ( ',' labeled_expr )*
    */

    many!(sep = Comma, labeled_expr).parse(ctx)
}

fn labeled_expr(ctx: &mut ParserContext) -> scan::Result<NamedValue> {

    /*
        a_expr ( AS col_label )?
    */

    let (value, name) = seq!(
        a_expr,
        seq!(As, col_label).optional()
    ).parse(ctx)?;

    let name = name.map(|(_, name)| name);

    let mut value = NamedValue::new(value);
    value.set_name(name);

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode::IntegerConst;
    use test_case::test_matrix;

    #[test_matrix("1" => Ok(
        NamedValue::new(IntegerConst(1))
    ))]
    #[test_matrix("2 as x" => Ok(
        NamedValue::new(IntegerConst(2)).with_name("x")
    ))]
    fn test_labeled_expr(source: &str) -> scan::Result<NamedValue> {
        test_parser!(source, labeled_expr)
    }
}

use crate::combinators::col_label;
use crate::combinators::core::Combinator;
use crate::combinators::expr::a_expr;
use crate::many;
use crate::seq;
use crate::ParserContext;
use pg_ast::NamedValue;
use pg_lexer::Keyword::As;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
