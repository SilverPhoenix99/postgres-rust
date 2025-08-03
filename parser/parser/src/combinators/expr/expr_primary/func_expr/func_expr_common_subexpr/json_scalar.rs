pub(super) fn json_scalar(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        JSON_SCALAR '(' a_expr ')'
    */

    if ! matches!(stream.peek2(), Ok((K(Kw::JsonScalar), Op(OpenParenthesis)))) {
        return no_match(stream)
    }

    let (_, arg) = seq!(skip(1), paren(a_expr))
        .parse(stream)?;

    let arg = Box::new(arg);
    Ok(JsonScalar(arg))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::IntegerConst,
        scan::Error::NoMatch,
    };

    #[test_case("json_scalar(1)" => Ok(
        JsonScalar(Box::new(
            IntegerConst(1)
        ))
    ))]
    #[test_case("json_scalar" => matches Err(NoMatch(_)))]
    #[test_case("json_scalar 1" => matches Err(NoMatch(_)))]
    fn test_json_scalar(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, json_scalar)
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use crate::combinators::foundation::Combinator;
use crate::no_match;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword as K;
use crate::stream::TokenValue::Operator as Op;
use pg_ast::ExprNode;
use pg_ast::ExprNode::JsonScalar;
use pg_lexer::Keyword as Kw;
use pg_lexer::OperatorKind::OpenParenthesis;
