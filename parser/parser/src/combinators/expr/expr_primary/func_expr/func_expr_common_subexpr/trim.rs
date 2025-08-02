pub(super) fn trim(stream: &mut TokenStream) -> scan::Result<TrimFunc> {
    use crate::stream::TokenValue::Keyword as K;
    use crate::stream::TokenValue::Operator as Op;

    /*
        TRIM '(' trim_args ')'
    */

    if ! matches!(stream.peek2(), Ok((K(Trim), Op(OpenParenthesis)))) {
        return no_match(stream)
    }

    let expr = skip_prefix(1, paren(trim_args))
        .parse(stream)?;

    Ok(expr)
}

fn trim_args(stream: &mut TokenStream) -> scan::Result<TrimFunc> {

    /*
          LEADING   trim_list
        | TRAILING  trim_list
        | ( BOTH )? trim_list
    */

    let (trim_side, args) = or((

        (Kw::Leading.map(|_| Leading), trim_list),

        (Kw::Trailing.map(|_| Trailing), trim_list),

        (
            Kw::Both.map(|_| Both)
                .optional()
                .map(Option::unwrap_or_default),
            trim_list
        )

    )).parse(stream)?;

    let expr = TrimFunc::new(trim_side, args);
    Ok(expr)
}

fn trim_list(stream: &mut TokenStream) -> scan::Result<Vec<ExprNode>> {

    /*
          FROM expr_list
        | a_expr ( ( FROM | ',') expr_list )?
    */

    or((
        (FromKw, expr_list).map(|(_, args)| args),
        (a_expr,
            (
                or((
                    Comma.map(|_| true),  // Prepend
                    FromKw.map(|_| false) // Append
                )),
                expr_list
            ).optional()
                .map(|opt|
                    opt.unwrap_or_else(|| (false, Vec::with_capacity(1)))
                )
        )
            .map(|(arg, (prepend, mut args))| {
                if prepend {
                    args.insert(0, arg);
                }
                else {
                    args.push(arg);
                }
                args
            })
    )).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::StringConst,
        scan::Error::NoMatch,
    };

    #[test_case("trim('foo' from 'bar')" => Ok(
        TrimFunc::new(
            Both,
            vec![StringConst("bar".into()), StringConst("foo".into())]
        )
    ))]
    #[test_case("trim" => matches Err(NoMatch(_)))]
    #[test_case("trim 1" => matches Err(NoMatch(_)))]
    fn test_trim(source: &str) -> scan::Result<TrimFunc> {
        test_parser!(source, trim)
    }

    #[test_case("leading from 'foo'" => Ok(TrimFunc::new(
        Leading,
        vec![StringConst("foo".into())]
    )))]
    #[test_case("trailing 'foo' from 'bar'" => Ok(TrimFunc::new(
        Trailing,
        vec![StringConst("bar".into()), StringConst("foo".into())]
    )))]
    #[test_case("both 'foo'" => Ok(TrimFunc::new(
        Both,
        vec![StringConst("foo".into())]
    )))]
    #[test_case("'foo', 'bar'" => Ok(TrimFunc::new(
        Both,
        vec![StringConst("foo".into()), StringConst("bar".into())]
    )))]
    fn test_trim_args(source: &str) -> scan::Result<TrimFunc> {
        test_parser!(source, trim_args)
    }

    #[test_case("from 'foo'" => Ok(vec![StringConst("foo".into())]))]
    #[test_case("from 'foo', 'bar'" => Ok(vec![
        StringConst("foo".into()),
        StringConst("bar".into())
    ]))]
    #[test_case("'foo'" => Ok(vec![StringConst("foo".into())]))]
    #[test_case("'foo' from 'bar'" => Ok(vec![
        StringConst("bar".into()),
        StringConst("foo".into())
    ]))]
    #[test_case("'foo' from 'bar', 'baz'" => Ok(vec![
        StringConst("bar".into()),
        StringConst("baz".into()),
        StringConst("foo".into())
    ]))]
    #[test_case("'foo', 'bar'" => Ok(vec![
        StringConst("foo".into()),
        StringConst("bar".into())
    ]))]
    #[test_case("'foo', 'bar', 'baz'" => Ok(vec![
        StringConst("foo".into()),
        StringConst("bar".into()),
        StringConst("baz".into()),
    ]))]
    fn test_trim_list(source: &str) -> scan::Result<Vec<ExprNode>> {
        test_parser!(source, trim_list)
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::expr_list;
use crate::combinators::foundation::or;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::skip_prefix;
use crate::combinators::foundation::Combinator;
use crate::no_match;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
use pg_ast::TrimFunc;
use pg_ast::TrimSide::Both;
use pg_ast::TrimSide::Leading;
use pg_ast::TrimSide::Trailing;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::FromKw;
use pg_lexer::Keyword::Trim;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::OpenParenthesis;
