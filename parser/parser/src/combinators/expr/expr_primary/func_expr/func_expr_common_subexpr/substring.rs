pub(super) fn substring(stream: &mut TokenStream) -> scan::Result<SubstringFunc> {
    use crate::stream::TokenValue::Keyword as K;
    use crate::stream::TokenValue::Operator as Op;

    /*
        SUBSTRING '(' ( substring_args )? ')'
    */

    if ! matches!(stream.peek2(), Ok((K(Substring), Op(OpenParenthesis)))) {
        return no_match(stream)
    }

    let (_, args) = seq!(skip(1), paren!(substring_args.optional()))
        .parse(stream)?;

    let args = args.unwrap_or_default();
    Ok(args)
}

fn substring_args(stream: &mut TokenStream) -> scan::Result<SubstringFunc> {

    /*
          func_arg_list
        | a_expr substring_list
    */

    let mut args: Vec<_> = func_arg_list(stream)?
        .into_iter()
        .map(|(arg, _)| arg)
        .collect();

    if
    let [arg] = args.as_mut_slice()
        && arg.name().is_none()
        && let Some((from, r#for)) = substring_list(stream).optional()?
    {
        let (_, arg) = mem::replace(arg, NamedValue::unnamed(NullConst)).into();
        let args = SubstringFunc::SqlSyntax(arg, from, r#for);
        return Ok(args)
    }

    Ok(SubstringFunc::ExplicitCall(Some(args)))
}

fn substring_list(stream: &mut TokenStream) -> scan::Result<(ExprNode, Option<ExprNode>)> {

    /*
          SIMILAR a_expr ESCAPE a_expr
        | FROM a_expr ( FOR a_expr )?
        | FOR a_expr ( FROM a_expr )?
    */

    alt!(
        similar_escape_args
            .map(|(similar, escape)| (similar, Some(escape))),
        from_for_args,
        for_from_args
    ).parse(stream)
}

fn similar_escape_args(stream: &mut TokenStream) -> scan::Result<(ExprNode, ExprNode)> {

    /*
        SIMILAR a_expr ESCAPE a_expr
    */

    let (_, similar, _, escape) = seq!(Similar, a_expr, Escape, a_expr)
        .parse(stream)?;

    Ok((similar, escape))
}

pub(super) fn from_for_args(stream: &mut TokenStream) -> scan::Result<(ExprNode, Option<ExprNode>)> {

    /*
        FROM a_expr ( FOR a_expr )?
    */

    /*
        Because data types aren't restricted here,
        the syntax without `FOR` can end up resolving to textregexsubstr().
        C-PG historically allowed that to happen, so continue
        to accept it.
    */

    let (_, from, r#for) = seq!(FromKw, a_expr, seq!(For, a_expr).optional())
        .parse(stream)?;

    let for_arg = r#for.map(|(_, expr)| expr);

    Ok((from, for_arg))
}

fn for_from_args(stream: &mut TokenStream) -> scan::Result<(ExprNode, Option<ExprNode>)> {

    /*
        FOR a_expr ( FROM a_expr )?

        not legal per SQL, but C-PG allows this
    */

    let (_, r#for, from) = seq!(For, a_expr, seq!(FromKw, a_expr).optional())
        .parse(stream)?;

    let args = match from {
        Some((_, from)) => (from, Some(r#for)),
        None => {

            /*
                Since there are no cases where this syntax allows
                a textual FOR value, the argument is forcibly cast
                to int4. The possible matches in pg_proc are
                substring(text,int4) and substring(text,text),
                and we don't want the parser to choose the latter,
                which it is likely to do if the second argument
                is unknown or doesn't have an implicit cast to int4.
            */

            let from = IntegerConst(1);
            let r#for = TypecastExpr::new(r#for, TypeName::Int4).into();
            (from, Some(r#for))
        },
    };

    Ok(args)
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

    #[test_case("substring()" => Ok(
        SubstringFunc::ExplicitCall(None)
    ))]
    #[test_case("substring('foo')" => Ok(
        SubstringFunc::ExplicitCall(
            Some(vec![
                NamedValue::unnamed(StringConst("foo".into()))
            ])
        )
    ))]
    #[test_case("substring" => matches Err(NoMatch(_)))]
    #[test_case("substring 1" => matches Err(NoMatch(_)))]
    fn test_substring(source: &str) -> scan::Result<SubstringFunc> {
        test_parser!(source, substring)
    }

    #[test_case("'foo'" => Ok(
        SubstringFunc::ExplicitCall(Some(vec![
            NamedValue::unnamed(StringConst("foo".into())),
        ]))
    ))]
    #[test_case("'foo', bar => 1" => Ok(
        SubstringFunc::ExplicitCall(Some(vec![
            NamedValue::unnamed(StringConst("foo".into())),
            NamedValue::new(Some("bar".into()), IntegerConst(1))
        ]))
    ))]
    #[test_case("'foo' similar 'bar' escape 'baz'" => Ok(
        SubstringFunc::SqlSyntax(
            StringConst("foo".into()),
            StringConst("bar".into()),
            Some(StringConst("baz".into()))
        )
    ))]
    #[test_case("'foo' from 1 for 2" => Ok(
        SubstringFunc::SqlSyntax(
            StringConst("foo".into()),
            IntegerConst(1),
            Some(IntegerConst(2))
        )
    ))]
    #[test_case("'foo' for 2 from 1" => Ok(
        SubstringFunc::SqlSyntax(
            StringConst("foo".into()),
            IntegerConst(1),
            Some(IntegerConst(2))
        )
    ))]
    #[test_case("'foo' from 1" => Ok(
        SubstringFunc::SqlSyntax(
            StringConst("foo".into()),
            IntegerConst(1),
            None
        )
    ))]
    #[test_case("'foo' for 2" => Ok(
        SubstringFunc::SqlSyntax(
            StringConst("foo".into()),
            IntegerConst(1),
            Some(
                TypecastExpr::new(
                    IntegerConst(2),
                    TypeName::Int4
                ).into()
            )
        )
    ))]
    fn test_substring_args(source: &str) -> scan::Result<SubstringFunc> {
        test_parser!(source, substring_args)
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use crate::combinators::foundation::Combinator;
use crate::combinators::func_arg_list;
use crate::no_match;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
use core::mem;
use pg_ast::ExprNode;
use pg_ast::ExprNode::IntegerConst;
use pg_ast::ExprNode::NullConst;
use pg_ast::NamedValue;
use pg_ast::SubstringFunc;
use pg_ast::TypeName;
use pg_ast::TypecastExpr;
use pg_lexer::Keyword::Escape;
use pg_lexer::Keyword::For;
use pg_lexer::Keyword::FromKw;
use pg_lexer::Keyword::Similar;
use pg_lexer::Keyword::Substring;
use pg_lexer::OperatorKind::OpenParenthesis;
