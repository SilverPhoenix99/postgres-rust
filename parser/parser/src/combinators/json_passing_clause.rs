/// Alias: `json_passing_clause_opt`
///
/// Inlined: `json_arguments`
pub(super) fn json_passing_clause(ctx: &mut ParserContext) -> scan::Result<Vec<JsonArgument>> {

    /*
        PASSING json_argument ( ',' json_argument )*
    */

    let (_, args) = seq!(Passing, many!(sep = Comma, json_argument))
        .parse(ctx)?;

    Ok(args)
}

fn json_argument(ctx: &mut ParserContext) -> scan::Result<JsonArgument> {

    /*
        json_value_expr AS col_label
    */

    let (value, _, key) = seq!(json_value_expr, As, col_label)
        .parse(ctx)?;

    Ok((key, value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::{IntegerConst, StringConst},
        pg_ast::JsonValueExpr,
        scan::Error::NoMatch,
    };

    #[test_case("passing 1 as a, 2 as b" => Ok(vec![
        (
            "a".into(),
            JsonValueExpr::from(IntegerConst(1))
        ),
        (
            "b".into(),
            JsonValueExpr::from(IntegerConst(2))
        )
    ]))]
    fn test_json_passing_clause(source: &str) -> scan::Result<Vec<JsonArgument>> {
        test_parser!(source, json_passing_clause)
    }

    #[test_case("'foo' as bar" => Ok((
        "bar".into(),
        JsonValueExpr::from(StringConst("foo".into()))
    )))]
    fn test_json_argument(source: &str) -> scan::Result<JsonArgument> {
        test_parser!(source, json_argument)
    }
}

use crate::combinators::json_value_expr;
use pg_ast::JsonArgument;
use pg_combinators::many;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::As;
use pg_lexer::Keyword::Passing;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_sink_combinators::col_label;
