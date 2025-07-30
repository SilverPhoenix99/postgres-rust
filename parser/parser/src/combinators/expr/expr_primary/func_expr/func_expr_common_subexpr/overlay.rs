pub(super) fn overlay(stream: &mut TokenStream) -> scan::Result<OverlayFunc> {
    use crate::stream::TokenValue::Keyword as K;
    use crate::stream::TokenValue::Operator as Op;

    /*
        OVERLAY '(' ( overlay_args )? ')'
    */

    if ! matches!(stream.peek2(), Ok((K(Overlay), Op(OpenParenthesis)))) {
        return no_match(stream)
    }

    let args = skip_prefix(1, between_paren(overlay_args.optional()))
        .parse(stream)?;

    let args = args.unwrap_or_default();
    Ok(args)
}

fn overlay_args(stream: &mut TokenStream) -> scan::Result<OverlayFunc> {

    /*
          func_arg_list
        | a_expr overlay_list
    */

    let mut args: Vec<FuncArgExpr> = func_arg_list(stream)?
        .into_iter()
        .map(|(arg, _)| arg)
        .collect();

    if
    let [Unnamed(arg)] = args.as_mut_slice()
        && let Some((placing, from, r#for)) = overlay_list(stream).optional()?
    {
        let arg = mem::replace(arg, NullConst);
        let args = OverlaySqlArgs::new(arg, placing, from, r#for);
        let args = OverlayFunc::SqlSyntax(args);
        return Ok(args);
    }

    let args = OverlayFunc::ExplicitCall(Some(args));
    Ok(args)
}

fn overlay_list(stream: &mut TokenStream) -> scan::Result<(ExprNode, ExprNode, Option<ExprNode>)> {

    /*
        PLACING a_expr FROM a_expr ( FOR a_expr )?
    */

    let (_, placing, (from, r#for)) = (Placing, a_expr, from_for_args)
        .parse(stream)?;

    Ok((placing, from, r#for))
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

    #[test_case("overlay()" => Ok(OverlayFunc::ExplicitCall(None)))]
    #[test_case("overlay(1)" => Ok(
        OverlayFunc::ExplicitCall(
            Some(vec![
                Unnamed(IntegerConst(1))
            ])
        )
    ))]
    #[test_case("overlay" => matches Err(NoMatch(_)))]
    #[test_case("overlay 1" => matches Err(NoMatch(_)))]
    fn test_overlay(source: &str) -> scan::Result<OverlayFunc> {
        test_parser!(source, overlay)
    }

    #[test_case("'foo'" => Ok(
        OverlayFunc::ExplicitCall(
            Some(vec![
                Unnamed(StringConst("foo".into()))
            ])
        )
    ))]
    #[test_case("'foo', bar := 1, baz => 2" => Ok(
        OverlayFunc::ExplicitCall(
            Some(vec![
                Unnamed(StringConst("foo".into())),
                FuncArgExpr::NamedValue {
                    name: "bar".into(),
                    value: IntegerConst(1)
                },
                FuncArgExpr::NamedValue {
                    name: "baz".into(),
                    value: IntegerConst(2)
                },
            ])
        )
    ))]
    #[test_case("'foo' placing 'bar' from 1" => Ok(
        OverlayFunc::SqlSyntax(
            OverlaySqlArgs::new(
                StringConst("foo".into()),
                StringConst("bar".into()),
                IntegerConst(1),
                None
            )
        )
    ))]
    #[test_case("'foo' placing 'bar' from 1 for 2" => Ok(
        OverlayFunc::SqlSyntax(
            OverlaySqlArgs::new(
                StringConst("foo".into()),
                StringConst("bar".into()),
                IntegerConst(1),
                Some(IntegerConst(2))
            )
        )
    ))]
    fn test_overlay_args(source: &str) -> scan::Result<OverlayFunc> {
        test_parser!(source, overlay_args)
    }
}

use super::from_for_args;
use crate::combinators::expr::a_expr;
use crate::combinators::foundation::between_paren;
use crate::combinators::foundation::skip_prefix;
use crate::combinators::foundation::Combinator;
use crate::combinators::func_arg_list;
use crate::no_match;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
use core::mem;
use pg_ast::ExprNode;
use pg_ast::ExprNode::NullConst;
use pg_ast::FuncArgExpr;
use pg_ast::FuncArgExpr::Unnamed;
use pg_ast::OverlayFunc;
use pg_ast::OverlaySqlArgs;
use pg_lexer::Keyword::Overlay;
use pg_lexer::Keyword::Placing;
use pg_lexer::OperatorKind::OpenParenthesis;
