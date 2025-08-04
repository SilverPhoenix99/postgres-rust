pub(super) fn explicit_row(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        ROW '(' ( expr_list )? ')'
    */

    let (Keyword(Kw::Row), Operator(OpenParenthesis)) = stream.peek2()? else {
        return no_match(stream)
    };

    let (_, col_values) = seq!(skip(1), paren!(expr_list.optional()))
        .parse(stream)?;

    Ok(Row(col_values))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::{IntegerConst, StringConst};
    use test_case::test_case;

    #[test_case("row()", None)]
    #[test_case("row(1)", Some(vec![IntegerConst(1)]))]
    #[test_case("row(1, 'foo')", Some(vec![IntegerConst(1), StringConst("foo".into())]))]
    fn test_explicit_row(source: &str, expected: Option<Vec<ExprNode>>) {
        test_parser!(source, explicit_row, Row(expected))
    }
}

use crate::combinators::expr_list;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use crate::combinators::foundation::Combinator;
use crate::no_match;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword;
use crate::stream::TokenValue::Operator;
use pg_ast::ExprNode;
use pg_ast::ExprNode::Row;
use pg_lexer::Keyword as Kw;
use pg_lexer::OperatorKind::OpenParenthesis;
