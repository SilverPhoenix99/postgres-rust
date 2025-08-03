pub(crate) fn json_name_and_value_list(stream: &mut TokenStream) -> scan::Result<Vec<JsonKeyValue>> {

    /*
        json_name_and_value ( ',' json_name_and_value )*
    */

    many_sep(Comma, json_name_and_value).parse(stream)
}

pub(crate) fn json_name_and_value(stream: &mut TokenStream) -> scan::Result<JsonKeyValue> {

    /*
        a_expr (VALUE | ':') json_value_expr

        The original grammar had `c_expr VALUE`,
        but it seems likely it was to avoid conflicts with LALR(1) parsing.
        To avoid conflicts between `a_expr` and `c_expr`, `a_expr` is used instead of `c_expr`.
    */

    let (name, _, value) = seq!(
        a_expr,
        alt!(Value.skip(), Colon.skip()),
        json_value_expr
    ).parse(stream)?;

    let json = JsonKeyValue::new(name, value);
    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{
        ExprNode::{IntegerConst, StringConst},
        JsonFormat,
        JsonValueExpr,
    };
    use test_case::test_case;

    #[test_case("'foo' : 1" => Ok(JsonKeyValue::new(
        StringConst("foo".into()),
        JsonValueExpr::new(
            IntegerConst(1),
            JsonFormat::default()
        )
    )))]
    #[test_case("'foo' : 1 format json" => Ok(JsonKeyValue::new(
        StringConst("foo".into()),
        JsonValueExpr::new(
            IntegerConst(1),
            JsonFormat::text()
        )
    )))]
    fn test_json_name_and_value(source: &str) -> scan::Result<JsonKeyValue> {
        test_parser!(source, json_name_and_value)
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::json_value_expr;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::JsonKeyValue;
use pg_lexer::Keyword::Value;
use pg_lexer::OperatorKind::Colon;
use pg_lexer::OperatorKind::Comma;
