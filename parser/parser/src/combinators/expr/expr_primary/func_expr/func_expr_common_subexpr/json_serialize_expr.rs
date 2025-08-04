pub(super) fn json_serialize_expr(stream: &mut TokenStream) -> scan::Result<JsonSerializeExpr> {

    /*
        JSON_SERIALIZE '(' json_value_expr ( json_returning_clause )? ')'
    */

    if !matches!(stream.peek2(), Ok((K(JsonSerialize), Op(OpenParenthesis)))) {
        return no_match(stream);
    }

    let (_, (value, output)) = seq!(
        skip(1),
        paren!(seq!(
            json_value_expr,
            json_returning_clause.optional(),
        ))
    ).parse(stream)?;

    let mut expr = JsonSerializeExpr::new(value);
    expr.set_output(output);

    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::IntegerConst,
        pg_ast::JsonOutput,
        pg_ast::JsonValueExpr,
        pg_ast::TypeName::Int4,
        scan::Error::NoMatch,
    };

    #[test_case("json_serialize(1)" => Ok(JsonSerializeExpr::new(
        JsonValueExpr::from(IntegerConst(1))
    )))]
    #[test_case("json_serialize(1 returning int)" => Ok(
        JsonSerializeExpr::new(JsonValueExpr::from(IntegerConst(1)))
            .with_output(JsonOutput::from(Int4))
    ))]
    #[test_case("json_serialize" => matches Err(NoMatch(_)))]
    #[test_case("json_serialize 1" => matches Err(NoMatch(_)))]
    fn test_json_serialize_expr(source: &str) -> scan::Result<JsonSerializeExpr> {
        test_parser!(source, json_serialize_expr)
    }
}

use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use crate::combinators::foundation::Combinator;
use crate::combinators::json_returning_clause;
use crate::combinators::json_value_expr;
use crate::no_match;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword as K;
use crate::stream::TokenValue::Operator as Op;
use pg_ast::JsonSerializeExpr;
use pg_lexer::Keyword::JsonSerialize;
use pg_lexer::OperatorKind::OpenParenthesis;
