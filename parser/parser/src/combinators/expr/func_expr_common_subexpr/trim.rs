pub(super) fn trim(stream: &mut TokenStream) -> scan::Result<TrimFunc> {

    /*
        TRIM '(' trim_args ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, expr) = seq!(skip(1), paren!(trim_args))
        .parse(stream)?;

    Ok(expr)
}

fn trim_args(stream: &mut TokenStream) -> scan::Result<TrimFunc> {

    /*
          LEADING   trim_list
        | TRAILING  trim_list
        | ( BOTH )? trim_list
    */

    let (trim_side, args) = alt!(

        seq!(Kw::Leading.map(|_| Leading), trim_list),

        seq!(Kw::Trailing.map(|_| Trailing), trim_list),

        seq!(
            Kw::Both.map(|_| Both)
                .optional()
                .map(Option::unwrap_or_default),
            trim_list
        )

    ).parse(stream)?;

    let expr = TrimFunc::new(trim_side, args);
    Ok(expr)
}

fn trim_list(stream: &mut TokenStream) -> scan::Result<Vec<ExprNode>> {

    /*
          FROM expr_list
        | a_expr ( ( FROM | ',') expr_list )?
    */

    alt!(
        seq!(FromKw, expr_list).map(|(_, args)| args),
        seq!(a_expr,
            seq!(
                alt!(
                    Comma.map(|_| true),  // Prepend
                    FromKw.map(|_| false) // Append
                ),
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
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::StringConst;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("trim('foo' from 'bar')" => Ok(
        TrimFunc::new(
            Both,
            vec![StringConst("bar".into()), StringConst("foo".into())]
        )
    ))]
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
use crate::combinators::foundation::alt;
use pg_ast::ExprNode;
use pg_ast::TrimFunc;
use pg_ast::TrimSide::Both;
use pg_ast::TrimSide::Leading;
use pg_ast::TrimSide::Trailing;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::skip;
use pg_combinators::Combinator;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::FromKw;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
