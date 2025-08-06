pub(super) fn overlay(stream: &mut TokenStream) -> scan::Result<OverlayFunc> {

    /*
        OVERLAY '(' ( overlay_args )? ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, args) = seq!(skip(1), paren!(overlay_args.optional()))
        .parse(stream)?;

    let args = args.unwrap_or_default();
    Ok(args)
}

fn overlay_args(stream: &mut TokenStream) -> scan::Result<OverlayFunc> {

    /*
          func_arg_list
        | a_expr overlay_list
    */

    let mut args: Vec<_> = func_arg_list(stream)?
        .into_iter()
        .map(|(arg, _)| arg)
        .collect();

    if
        let [arg] = args.as_mut_slice()
        && arg.name().is_none()
        && let Some((placing, from, r#for)) = overlay_list(stream).optional()?
    {
        let (_, arg) = mem::replace(arg, NamedValue::unnamed(NullConst)).into();
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

    let (_, placing, (from, r#for)) = seq!(Placing, a_expr, from_for_args)
        .parse(stream)?;

    Ok((placing, from, r#for))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::{IntegerConst, StringConst};
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("overlay()" => Ok(OverlayFunc::ExplicitCall(None)))]
    #[test_case("overlay(1)" => Ok(
        OverlayFunc::ExplicitCall(
            Some(vec![
                NamedValue::unnamed(IntegerConst(1))
            ])
        )
    ))]
    fn test_overlay(source: &str) -> scan::Result<OverlayFunc> {
        test_parser!(source, overlay)
    }

    #[test_case("'foo'" => Ok(
        OverlayFunc::ExplicitCall(
            Some(vec![
                NamedValue::unnamed(StringConst("foo".into()))
            ])
        )
    ))]
    #[test_case("'foo', bar := 1, baz => 2" => Ok(
        OverlayFunc::ExplicitCall(
            Some(vec![
                NamedValue::unnamed(StringConst("foo".into())),
                NamedValue::new(Some("bar".into()), IntegerConst(1)),
                NamedValue::new(Some("baz".into()), IntegerConst(2)),
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
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use crate::combinators::func_arg_list;
use core::mem;
use pg_ast::ExprNode;
use pg_ast::ExprNode::NullConst;
use pg_ast::NamedValue;
use pg_ast::OverlayFunc;
use pg_ast::OverlaySqlArgs;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Placing;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
use pg_parser_core::Optional;
