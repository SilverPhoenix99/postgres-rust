pub(super) fn extract(ctx: &mut ParserContext) -> scan::Result<ExtractFunc> {

    /*
        EXTRACT '(' extract_list ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, expr) = seq!(skip(1), paren!(extract_args))
        .parse(ctx)?;

    Ok(expr)
}

/// Aliases: `extract_list`
fn extract_args(ctx: &mut ParserContext) -> scan::Result<ExtractFunc> {

    /*
        extract_arg FROM a_expr
    */

    let (field, _, target) = seq!(extract_arg, FromKw, a_expr)
        .parse(ctx)?;

    let expr = ExtractFunc::new(field, target);
    Ok(expr)
}

fn extract_arg(ctx: &mut ParserContext) -> scan::Result<ExtractArg> {

    /*
        YEAR
      | MONTH
      | DAY
      | HOUR
      | MINUTE
      | SECOND
      | identifier
      | string
    */

    alt!(
        Kw::Year.map(|_| Year),
        Kw::Month.map(|_| Month),
        Kw::Day.map(|_| Day),
        Kw::Hour.map(|_| Hour),
        Kw::Minute.map(|_| Minute),
        Kw::Second.map(|_| Second),
        string.map(Named),
        identifier.map(Named),
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode::{IntegerConst, StringConst};
    use test_case::test_matrix;

    #[test_matrix("extract(year from 'foo')" => Ok(
        ExtractFunc::new(
            Year,
            StringConst("foo".into())
        )
    ))]
    fn test_extract_func(source: &str) -> scan::Result<ExtractFunc> {
        test_parser!(source, extract)
    }

    #[test_matrix("second from 1" => Ok(ExtractFunc::new(
        Second,
        IntegerConst(1)
    )))]
    #[test_matrix("foo from 2" => Ok(ExtractFunc::new(
        Named("foo".into()),
        IntegerConst(2)
    )))]
    #[test_matrix("'bar' from 'foo'" => Ok(ExtractFunc::new(
        Named("bar".into()),
        StringConst("foo".into())
    )))]
    fn test_extract_args(source: &str) -> scan::Result<ExtractFunc> {
        test_parser!(source, extract_args)
    }

    #[test_matrix("year" => Ok(Year))]
    #[test_matrix("month" => Ok(Month))]
    #[test_matrix("day" => Ok(Day))]
    #[test_matrix("hour" => Ok(Hour))]
    #[test_matrix("minute" => Ok(Minute))]
    #[test_matrix("second" => Ok(Second))]
    #[test_matrix("foo" => Ok(Named("foo".into())))]
    #[test_matrix("'bar'" => Ok(Named("bar".into())))]
    fn test_extract_arg(source: &str) -> scan::Result<ExtractArg> {
        test_parser!(source, extract_arg)
    }
}

use crate::alt;
use crate::combinators::core::identifier;
use crate::combinators::core::skip;
use crate::combinators::core::string;
use crate::combinators::core::Combinator;
use crate::combinators::expr::a_expr;
use crate::paren;
use crate::seq;
use crate::ParserContext;
use pg_ast::ExtractArg;
use pg_ast::ExtractArg::Day;
use pg_ast::ExtractArg::Hour;
use pg_ast::ExtractArg::Minute;
use pg_ast::ExtractArg::Month;
use pg_ast::ExtractArg::Named;
use pg_ast::ExtractArg::Second;
use pg_ast::ExtractArg::Year;
use pg_ast::ExtractFunc;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::FromKw;
use pg_parser_core::scan;
