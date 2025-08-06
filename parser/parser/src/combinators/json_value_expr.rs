pub(super) fn json_value_expr_list(stream: &mut TokenStream) -> scan::Result<Vec<JsonValueExpr>> {

    /*
        json_value_expr ( ',' json_value_expr )*
    */

    many!(sep = Comma, json_value_expr).parse(stream)
}

pub(super) fn json_value_expr(stream: &mut TokenStream) -> scan::Result<JsonValueExpr> {

    /*
        a_expr ( json_format_clause )?
    */

    let (expr, format) = seq!(a_expr, json_format_clause.optional())
        .parse(stream)?;

    let format = format.unwrap_or_default();
    let expr = JsonValueExpr::new(expr, format);

    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pg_ast::{
        ExprNode::StringConst,
        JsonEncoding::UTF8,
        JsonFormat,
    };
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("'foo'" => Ok(
        JsonValueExpr::new(
            StringConst("foo".into()),
            JsonFormat::default()
        )
    ))]
    #[test_case("'foo' format json" => Ok(
        JsonValueExpr::new(
            StringConst("foo".into()),
            JsonFormat::text()
        )
    ))]
    #[test_case("'foo' format json encoding utf8" => Ok(
        JsonValueExpr::new(
            StringConst("foo".into()),
            JsonFormat::text().with_encoding(UTF8)
        )
    ))]
    fn test_json_value_expr(source: &str) -> scan::Result<JsonValueExpr> {
        test_parser!(source, json_value_expr)
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::many;
use crate::combinators::foundation::seq;
use crate::combinators::json_format_clause;
use pg_ast::JsonValueExpr;
use pg_combinators::Combinator;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
