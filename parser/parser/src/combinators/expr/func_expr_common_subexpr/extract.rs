pub(super) fn extract(stream: &mut TokenStream) -> scan::Result<ExtractFunc> {

    /*
        EXTRACT '(' extract_list ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, expr) = seq!(skip(1), paren!(extract_args))
        .parse(stream)?;

    Ok(expr)
}

/// Aliases: `extract_list`
fn extract_args(stream: &mut TokenStream) -> scan::Result<ExtractFunc> {

    /*
        extract_arg FROM a_expr
    */

    let (field, _, target) = seq!(extract_arg, FromKw, a_expr)
        .parse(stream)?;

    let expr = ExtractFunc::new(field, target);
    Ok(expr)
}

fn extract_arg(stream: &mut TokenStream) -> scan::Result<ExtractArg> {

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
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::{IntegerConst, StringConst};
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("extract(year from 'foo')" => Ok(
        ExtractFunc::new(
            ExtractArg::Year,
            StringConst("foo".into())
        )
    ))]
    fn test_extract_func(source: &str) -> scan::Result<ExtractFunc> {
        test_parser!(source, extract)
    }

    #[test_case("second from 1" => Ok(ExtractFunc::new(
        Second,
        IntegerConst(1)
    )))]
    #[test_case("foo from 2" => Ok(ExtractFunc::new(
        Named("foo".into()),
        IntegerConst(2)
    )))]
    #[test_case("'bar' from 'foo'" => Ok(ExtractFunc::new(
        Named("bar".into()),
        StringConst("foo".into())
    )))]
    fn test_extract_args(source: &str) -> scan::Result<ExtractFunc> {
        test_parser!(source, extract_args)
    }

    #[test_case("year" => Ok(Year))]
    #[test_case("month" => Ok(Month))]
    #[test_case("day" => Ok(Day))]
    #[test_case("hour" => Ok(Hour))]
    #[test_case("minute" => Ok(Minute))]
    #[test_case("second" => Ok(Second))]
    #[test_case("foo" => Ok(Named("foo".into())))]
    #[test_case("'bar'" => Ok(Named("bar".into())))]
    fn test_extract_arg(source: &str) -> scan::Result<ExtractArg> {
        test_parser!(source, extract_arg)
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::identifier;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::skip;
use crate::combinators::foundation::string;
use pg_ast::ExtractArg;
use pg_ast::ExtractArg::Day;
use pg_ast::ExtractArg::Hour;
use pg_ast::ExtractArg::Minute;
use pg_ast::ExtractArg::Month;
use pg_ast::ExtractArg::Named;
use pg_ast::ExtractArg::Second;
use pg_ast::ExtractArg::Year;
use pg_ast::ExtractFunc;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::FromKw;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
