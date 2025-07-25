pub(super) fn explicit_row(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        ROW '(' ( expr_list )? ')'
    */

    let (Keyword(Kw::Row), Operator(OpenParenthesis)) = stream.peek2()? else {
        return no_match(stream)
    };

    stream.next(); // "row"

    let col_values = between_paren(expr_list.optional())
        .parse(stream)
        .required()?;

    Ok(Row(col_values))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::{IntegerConst, StringConst};
    use test_case::test_case;

    #[test_case("row()", Row(None))]
    #[test_case("row(1)", Row(Some(vec![IntegerConst(1)])))]
    #[test_case("row(1, 'foo')", Row(Some(vec![IntegerConst(1), StringConst("foo".into())])))]
    fn test_explicit_row(source: &str, expected: ExprNode) {
        test_parser!(source, explicit_row, expected)
    }
}

use crate::combinators::expr_list;
use crate::combinators::foundation::between_paren;
use crate::combinators::foundation::Combinator;
use crate::no_match;
use crate::result::Required;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword;
use crate::stream::TokenValue::Operator;
use pg_ast::ExprNode;
use pg_ast::ExprNode::Row;
use pg_lexer::Keyword as Kw;
use pg_lexer::OperatorKind::OpenParenthesis;
