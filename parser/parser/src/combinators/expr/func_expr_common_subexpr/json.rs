pub(super) fn json(ctx: &mut ParserContext) -> scan::Result<JsonFunc> {

    /*
        JSON '(' json_value_expr ( json_key_uniqueness_constraint )? ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, (value, unique)) = seq!(skip(1),
        paren!(seq!(
            json_value_expr,
            json_key_uniqueness_constraint.optional()
        ))
    ).parse(ctx)?;

    let unique = unique.unwrap_or_default();

    let func = JsonFunc::new(value)
        .with_unique(unique);
    Ok(func)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode::StringConst;
    use pg_ast::JsonEncoding;
    use pg_ast::JsonFormat;
    use pg_ast::JsonValueExpr;
    use test_case::test_matrix;

    #[test_matrix("json('foo')" => Ok(
        JsonFunc::new(StringConst("foo".into()))
    ))]
    #[test_matrix("json('bar' format json encoding UTF8 with unique keys)" => Ok(
        JsonFunc::new(
            JsonValueExpr::new(StringConst("bar".into()))
                .with_format(
                    JsonFormat::text()
                        .with_encoding(JsonEncoding::UTF8)
                )
        )
        .with_unique(true)
    ))]
    fn test_json(source: &str) -> scan::Result<JsonFunc> {
        test_parser!(source, json)
    }
}

use crate::combinators::core::skip;
use crate::combinators::core::Combinator;
use crate::combinators::json_key_uniqueness_constraint;
use crate::combinators::json_value_expr;
use crate::paren;
use crate::seq;
use crate::ParserContext;
use pg_ast::JsonFunc;
use pg_parser_core::scan;
