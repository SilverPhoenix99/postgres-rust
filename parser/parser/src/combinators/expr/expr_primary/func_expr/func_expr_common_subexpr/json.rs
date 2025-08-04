pub(super) fn json(stream: &mut TokenStream) -> scan::Result<JsonFunc> {

    /*
        JSON '(' json_value_expr ( json_key_uniqueness_constraint )? ')'
    */

    let (_, (value, unique)) = seq!(Json,
        paren!(seq!(
            json_value_expr,
            json_key_uniqueness_constraint.optional()
        ))
    ).parse(stream)?;

    let unique = unique.unwrap_or_default();

    let func = JsonFunc::new(value, unique);
    Ok(func)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{
        ExprNode::StringConst,
        JsonEncoding,
        JsonFormat,
        JsonValueExpr,
    };
    use test_case::test_case;

    #[test_case("json('foo')" => Ok(
        JsonFunc::new(
            JsonValueExpr::from(StringConst("foo".into())),
            false
        )
    ))]
    #[test_case("json('bar' format json encoding UTF8 with unique keys)" => Ok(
        JsonFunc::new(
            JsonValueExpr::new(
                StringConst("bar".into()),
                JsonFormat::text()
                    .with_encoding(JsonEncoding::UTF8)
            ),
            true
        )
    ))]
    fn test_json(source: &str) -> scan::Result<JsonFunc> {
        test_parser!(source, json)
    }
}

use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::json_key_uniqueness_constraint;
use crate::combinators::json_value_expr;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::JsonFunc;
use pg_lexer::Keyword::Json;
