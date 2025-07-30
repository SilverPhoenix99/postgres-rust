type JsonArgument = (Str, JsonValueExpr);

/// Inlined: `json_arguments`
pub(super) fn json_passing_clause(stream: &mut TokenStream) -> scan::Result<Vec<JsonArgument>> {

    /*
        PASSING json_argument ( ',' json_argument )*
    */

    let (_, args) = (Passing, many_sep(Comma, json_argument))
        .parse(stream)?;

    Ok(args)
}

fn json_argument(stream: &mut TokenStream) -> scan::Result<JsonArgument> {

    /*
        json_value_expr AS col_label
    */

    let (value, _, key) = (json_value_expr, As, col_label)
        .parse(stream)?;

    Ok((key, value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::{IntegerConst, StringConst},
        scan::Error::NoMatch,
    };

    #[test_case("passing 1 as a, 2 as b" => Ok(vec![
        (
            Str::from("a"),
            JsonValueExpr::new(IntegerConst(1), Default::default())
        ),
        (
            Str::from("b"),
            JsonValueExpr::new(IntegerConst(2), Default::default())
        )
    ]))]
    fn test_json_passing_clause(source: &str) -> scan::Result<Vec<JsonArgument>> {
        test_parser!(source, json_passing_clause)
    }

    #[test_case("'foo' as bar" => Ok((
        "bar".into(),
        JsonValueExpr::new(
            StringConst("foo".into()),
            Default::default()
        )
    )))]
    fn test_json_argument(source: &str) -> scan::Result<JsonArgument> {
        test_parser!(source, json_argument)
    }
}

use crate::combinators::col_label;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::Combinator;
use crate::combinators::json_value_expr;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::JsonValueExpr;
use pg_basics::Str;
use pg_lexer::Keyword::As;
use pg_lexer::Keyword::Passing;
use pg_lexer::OperatorKind::Comma;
