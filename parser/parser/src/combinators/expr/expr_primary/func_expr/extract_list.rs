/// Aliases: `extract_list`
pub(super) fn extract_args(stream: &mut TokenStream) -> scan::Result<ExtractFunc> {

    /*
        extract_arg FROM a_expr
    */

    let (field, _, target) = (extract_arg, FromKw, a_expr).parse(stream)?;

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

    or((
        Kw::Year.map(|_| Year),
        Kw::Month.map(|_| Month),
        Kw::Day.map(|_| Day),
        Kw::Hour.map(|_| Hour),
        Kw::Minute.map(|_| Minute),
        Kw::Second.map(|_| Second),
        string.map(Named),
        identifier.map(Named),
    )).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::ExprNode;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::{IntegerConst, StringConst};
    use test_case::test_case;

    #[test_case("second from 1", Second, IntegerConst(1))]
    #[test_case("foo from 2", Named("foo".into()), IntegerConst(2))]
    #[test_case("'bar' from 'foo'", Named("bar".into()), StringConst("foo".into()))]
    fn test_extract_args(source: &str, field: ExtractArg, target: ExprNode) {
        test_parser!(source, extract_args, ExtractFunc::new(field, target))
    }

    #[test_case("year", Year)]
    #[test_case("month", Month)]
    #[test_case("day", Day)]
    #[test_case("hour", Hour)]
    #[test_case("minute", Minute)]
    #[test_case("second", Second)]
    #[test_case("foo", Named("foo".into()))]
    #[test_case("'bar'", Named("bar".into()))]
    fn test_extract_arg(source: &str, expected: ExtractArg) {
        test_parser!(source, extract_arg, expected)
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::identifier;
use crate::combinators::foundation::or;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
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
