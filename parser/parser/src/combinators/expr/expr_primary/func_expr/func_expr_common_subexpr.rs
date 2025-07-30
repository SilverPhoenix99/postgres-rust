pub(super) fn func_expr_common_subexpr(stream: &mut TokenStream) -> scan::Result<ExprNode> {
    use crate::stream::TokenValue::Keyword as K;
    use crate::stream::TokenValue::Operator as Op;

    /*
          COLLATION FOR '(' a_expr ')'
        | CURRENT_DATE
        | CURRENT_TIME ( '(' ICONST ')' )?
        | CURRENT_TIMESTAMP ( '(' ICONST ')' )?
        | LOCALTIME ( '(' ICONST ')' )?
        | LOCALTIMESTAMP ( '(' ICONST ')' )?
        | CURRENT_ROLE
        | CURRENT_USER
        | SESSION_USER
        | SYSTEM_USER
        | USER
        | CURRENT_CATALOG
        | CURRENT_SCHEMA
        | CAST '(' a_expr AS Typename ')'
        | COALESCE '(' expr_list ')'
        | EXTRACT '(' extract_list ')'
        | GREATEST '(' expr_list ')'
        | LEAST '(' expr_list ')'
        | NORMALIZE '(' a_expr ( ',' unicode_normal_form )? ')'
        | NULLIF '(' a_expr ',' a_expr ')'
        | POSITION '(' b_expr IN b_expr ')'
        | TREAT '(' a_expr AS Typename ')'
        | TRIM '(' trim_args ')'
        | MERGE_ACTION '(' ')'
        | OVERLAY '(' ( overlay_args )? ')'
        | SUBSTRING '(' ( substring_args )? ')'
    */

    match stream.peek2() {

        // TypeFuncName conflicts:

        // `current_schema()` is valid syntax, so exclude that case.
        Ok((K(Kw::CurrentSchema), Op(OpenParenthesis))) => return no_match(stream),
        Ok((K(Kw::CurrentSchema), _)) => {
            stream.skip(1); // Consume the `current_schema` keyword.
            return Ok(CurrentSchema)
        },

        Ok((K(Collation), K(For))) => return collation_for(stream),

        // ColumnName conflicts:

        Ok((K(Coalesce), Op(OpenParenthesis))) => return coalesce_expr(stream),

        Ok((K(Extract), Op(OpenParenthesis))) => return extract_func(stream).map(From::from),

        Ok((K(Greatest), Op(OpenParenthesis))) => return greatest_func(stream),

        Ok((K(Least), Op(OpenParenthesis))) => return least_func(stream),

        Ok((K(MergeAction), Op(OpenParenthesis))) => {
            skip_prefix(2, CloseParenthesis)
                .parse(stream)?;
            return Ok(MergeSupportFunc)
        },

        Ok((K(Normalize), Op(OpenParenthesis))) => return normalize_func(stream).map(From::from),

        Ok((K(Nullif), Op(OpenParenthesis))) => return nullif(stream),

        Ok((K(Position), Op(OpenParenthesis))) => return position(stream).map(From::from),

        Ok((K(Kw::Treat), Op(OpenParenthesis))) => return treat(stream),

        Ok((K(Trim), Op(OpenParenthesis))) => return trim(stream).map(From::from),

        Ok((K(Overlay), Op(OpenParenthesis))) => return overlay(stream).map(From::from),

        Ok((K(Substring), Op(OpenParenthesis))) => return substring(stream).map(From::from),

        _ => {}
    }

    // If we reach here, it could be that there are 1 or fewer tokens left in the stream,
    // or there are more tokens, but they didn't match any of the above patterns.

    // Broken down into smaller combinators, due to large Rust type names.
    or((
        Kw::CurrentSchema.map(|_| CurrentSchema),
        Kw::CurrentCatalog.map(|_| CurrentCatalog),
        time,
        role,
        cast_expr.map(From::from),
    )).parse(stream)
}

fn time(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    or((
        Kw::CurrentDate.map(|_| CurrentDate),

        (Kw::CurrentTime, precision.optional())
            .map(|(_, precision)| CurrentTime { precision }),

        (Kw::CurrentTimestamp, precision.optional())
            .map(|(_, precision)| CurrentTimestamp { precision }),

        (Kw::Localtime, precision.optional())
            .map(|(_, precision)| LocalTime { precision }),

        (Kw::Localtimestamp, precision.optional())
            .map(|(_, precision)| LocalTimestamp { precision }),

    )).parse(stream)
}

fn role(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    or((
        Kw::CurrentRole.map(|_| CurrentRole),
        Kw::CurrentUser.map(|_| CurrentUser),
        Kw::SessionUser.map(|_| SessionUser),
        Kw::SystemUser.map(|_| SystemUser),
        Kw::User.map(|_| User),
    )).parse(stream)
}

fn collation_for(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        COLLATION FOR '(' a_expr ')'
    */

    let expr = skip_prefix(2, between_paren(a_expr))
        .parse(stream)?;

    let expr = Box::new(expr);
    Ok(CollationFor(expr))
}

fn coalesce_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        COALESCE '(' expr_list ')'
    */

    let args = skip_prefix(1, expr_list_paren)
        .parse(stream)?;

    Ok(CoalesceExpr(args))
}

fn extract_func(stream: &mut TokenStream) -> scan::Result<ExtractFunc> {

    /*
        EXTRACT '(' extract_list ')'
    */

    let expr = skip_prefix(1, between_paren(extract_args))
        .parse(stream)?;

    Ok(expr)
}

fn greatest_func(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        GREATEST '(' expr_list ')'
    */

    let args = skip_prefix(1, expr_list_paren)
        .parse(stream)?;

    Ok(GreatestFunc(args))
}

fn least_func(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        LEAST '(' expr_list ')'
    */

    let args = skip_prefix(1, expr_list_paren)
        .parse(stream)?;

    Ok(LeastFunc(args))
}

fn normalize_func(stream: &mut TokenStream) -> scan::Result<NormalizeFunc> {

    /*
        NORMALIZE '(' a_expr ( ',' unicode_normal_form )? ')'
    */

    let (expr, normal_form) = skip_prefix(1,
        between_paren((
            a_expr,
            (Comma, unicode_normal_form).optional()
        ))
    ).parse(stream)?;

    let normal_form = normal_form
        .map(|(_, normal_form)| normal_form);

    let expr = NormalizeFunc::new(expr, normal_form);
    Ok(expr)
}

fn nullif(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        NULLIF '(' a_expr ',' a_expr ')'
    */

    let (left, _, right) = skip_prefix(1,
        between_paren((a_expr, Comma, a_expr))
    ).parse(stream)?;

    let operands = Box::new((left, right));
    Ok(NullIf(operands))
}

/// Inlined: `position_list`
fn position(stream: &mut TokenStream) -> scan::Result<PositionFunc> {

    /*
        POSITION '(' b_expr IN b_expr ')'

        A "plain syntax" option is deliberately not offered
        for position(), because the reversal of the arguments
        creates too much risk of confusion.
    */

    let (needle, _, haystack) = skip_prefix(1,
        between_paren((b_expr, In, b_expr))
    ).parse(stream)?;

    let expr = PositionFunc::new(needle, haystack);
    Ok(expr)
}

fn treat(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        TREAT '(' a_expr AS Typename ')'

        Converts the expression of a particular type to a target type,
        which is defined to be a subtype of the original expression.
        In SQL99, this is intended for use with structured UDTs,
        but let's make this a generally useful form allowing stronger
        coercions than are handled by implicit casting.
    */

    let (expr, _, typename) = skip_prefix(1,
        between_paren((a_expr, As, typename))
    ).parse(stream)?;

    let cast = TypecastExpr::new(expr, typename);
    let expr = Treat(Box::new(cast));
    Ok(expr)
}

fn trim(stream: &mut TokenStream) -> scan::Result<TrimFunc> {

    /*
        TRIM '(' trim_args ')'
    */

    let expr = skip_prefix(1, between_paren(trim_args))
        .parse(stream)?;

    Ok(expr)
}

fn trim_args(stream: &mut TokenStream) -> scan::Result<TrimFunc> {
    use TrimSide::*;

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

fn overlay(stream: &mut TokenStream) -> scan::Result<OverlayFunc> {

    /*
        OVERLAY '(' ( overlay_args )? ')'
    */

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

fn substring(stream: &mut TokenStream) -> scan::Result<SubstringFunc> {

    /*
        SUBSTRING '(' ( substring_args )? ')'
    */

    let args = skip_prefix(1, between_paren(substring_args.optional()))
        .parse(stream)?;

    let args = args.unwrap_or_default();
    Ok(args)
}

fn substring_args(stream: &mut TokenStream) -> scan::Result<SubstringFunc> {

    /*
          func_arg_list
        | a_expr substring_list
    */

    let mut args: Vec<FuncArgExpr> = func_arg_list(stream)?
        .into_iter()
        .map(|(arg, _)| arg)
        .collect();

    if
        let [Unnamed(arg)] = args.as_mut_slice()
        && let Some((from, r#for)) = substring_list(stream).optional()?
    {
        let arg = mem::replace(arg, NullConst);
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

    or((
        similar_escape_args
            .map(|(similar, escape)| (similar, Some(escape))),
        from_for_args,
        for_from_args
    )).parse(stream)
}

fn similar_escape_args(stream: &mut TokenStream) -> scan::Result<(ExprNode, ExprNode)> {

    /*
        SIMILAR a_expr ESCAPE a_expr
    */

    let (_, similar, _, escape) = (Similar, a_expr, Escape, a_expr)
        .parse(stream)?;

    Ok((similar, escape))
}

fn from_for_args(stream: &mut TokenStream) -> scan::Result<(ExprNode, Option<ExprNode>)> {

    /*
        FROM a_expr ( FOR a_expr )?
    */

    /*
        Because data types aren't restricted here,
        the syntax without `FOR` can end up resolving to textregexsubstr().
        C-PG historically allowed that to happen, so continue
        to accept it.
    */

    let (_, from, r#for) = (FromKw, a_expr, (For, a_expr).optional())
        .parse(stream)?;

    let for_arg = r#for.map(|(_, expr)| expr);

    Ok((from, for_arg))
}

fn for_from_args(stream: &mut TokenStream) -> scan::Result<(ExprNode, Option<ExprNode>)> {

    /*
        FOR a_expr ( FROM a_expr )?

        not legal per SQL, but C-PG allows this
    */

    let (_, r#for, from) = (For, a_expr, (FromKw, a_expr).optional())
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
        crate::scan::Error::NoMatch,
        pg_ast::ExprNode::StringConst,
        pg_ast::ExtractArg,
        pg_ast::ExtractFunc,
        pg_ast::TypeName,
        pg_ast::UnicodeNormalForm::CanonicalComposition,
    };

    #[test_case("current_schema 1" => Ok(CurrentSchema))]
    #[test_case("current_schema" => Ok(CurrentSchema))]
    #[test_case("current_catalog" => Ok(CurrentCatalog))]
    #[test_case("collation for ('foo')" => Ok(
        CollationFor(
            Box::new(StringConst("foo".into()))
        )
    ))]
    #[test_case("coalesce('foo', 'bar')" => Ok(
        CoalesceExpr(vec![
            StringConst("foo".into()),
            StringConst("bar".into())
        ])
    ))]
    #[test_case("extract(year from 'foo')" => Ok(
        ExtractFunc::new(
            ExtractArg::Year,
            StringConst("foo".into())
        ).into()
    ))]
    #[test_case("greatest(1, 2)" => Ok(
        GreatestFunc(vec![
            IntegerConst(1),
            IntegerConst(2)
        ])
    ))]
    #[test_case("least(1, 2)" => Ok(
        LeastFunc(vec![
            IntegerConst(1),
            IntegerConst(2)
        ])
    ))]
    #[test_case("merge_action()" => Ok(MergeSupportFunc))]
    #[test_case("normalize('foo')" => Ok(
        NormalizeFunc::new(
            StringConst("foo".into()),
            None
        ).into()
    ))]
    #[test_case("normalize('foo', nfc)" => Ok(
        NormalizeFunc::new(
            StringConst("foo".into()),
            Some(CanonicalComposition)
        ).into()
    ))]
    #[test_case("nullif(null, 'foo')" => Ok(
        NullIf(Box::new((
            ExprNode::NullConst,
            StringConst("foo".into())
        )))
    ))]
    #[test_case("position('f' in 'foo')" => Ok(
        PositionFunc::new(
            StringConst("f".into()),
            StringConst("foo".into())
        ).into()
    ))]
    #[test_case("treat(123 as int)" => Ok(
        Treat(Box::new(
            TypecastExpr::new(
                IntegerConst(123),
                TypeName::Int4
            )
        ))
    ))]
    #[test_case("trim('foo' from 'bar')" => Ok(
        TrimFunc::new(
            TrimSide::Both,
            vec![StringConst("bar".into()), StringConst("foo".into())]
        ).into()
    ))]
    #[test_case("overlay()" => Ok(
        OverlayFunc::ExplicitCall(None).into()
    ))]
    #[test_case("overlay(1)" => Ok(
        OverlayFunc::ExplicitCall(
            Some(vec![
                Unnamed(IntegerConst(1))
            ])
        ).into()
    ))]
    #[test_case("substring()" => Ok(
        SubstringFunc::ExplicitCall(None).into()
    ))]
    #[test_case("substring('foo')" => Ok(
        SubstringFunc::ExplicitCall(
            Some(vec![
                Unnamed(StringConst("foo".into()))
            ])
        ).into()
    ))]
    #[test_case("current_schema(" => matches Err(NoMatch(_)))]
    // These only quickly check that statements aren't missing:
    #[test_case("current_date" => matches Ok(_))]
    #[test_case("user" => matches Ok(_))]
    #[test_case("cast ('1' as int)" => matches Ok(_))]
    fn test_func_expr_common_subexpr(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, func_expr_common_subexpr)
    }

    #[test_case("current_date" => Ok(CurrentDate))]
    #[test_case("current_time" => Ok(CurrentTime { precision: None }))]
    #[test_case("current_time(3)" => Ok(CurrentTime { precision: Some(3) }))]
    #[test_case("current_timestamp" => Ok(CurrentTimestamp { precision: None }))]
    #[test_case("current_timestamp(7)" => Ok(CurrentTimestamp { precision: Some(7) }))]
    #[test_case("localtime" => Ok(LocalTime { precision: None }))]
    #[test_case("localtime(6)" => Ok(LocalTime { precision: Some(6) }))]
    #[test_case("localtimestamp" => Ok(LocalTimestamp { precision: None }))]
    #[test_case("localtimestamp(4)" => Ok(LocalTimestamp { precision: Some(4) }))]
    fn test_time(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, time)
    }

    #[test_case("CURRENT_role" => Ok(CurrentRole))]
    #[test_case("current_USER" => Ok(CurrentUser))]
    #[test_case("SESSION_USER" => Ok(SessionUser))]
    #[test_case("system_user" => Ok(SystemUser))]
    #[test_case("uSeR" => Ok(User))]
    fn test_role(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, role)
    }

    #[test_case("leading from 'foo'" => Ok(TrimFunc::new(
        TrimSide::Leading,
        vec![StringConst("foo".into())]
    )))]
    #[test_case("trailing 'foo' from 'bar'" => Ok(TrimFunc::new(
        TrimSide::Trailing,
        vec![StringConst("bar".into()), StringConst("foo".into())]
    )))]
    #[test_case("both 'foo'" => Ok(TrimFunc::new(
        TrimSide::Both,
        vec![StringConst("foo".into())]
    )))]
    #[test_case("'foo', 'bar'" => Ok(TrimFunc::new(
        TrimSide::Both,
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

    #[test_case("'foo'" => Ok(
        SubstringFunc::ExplicitCall(Some(vec![
            Unnamed(StringConst("foo".into())),
        ]))
    ))]
    #[test_case("'foo', bar => 1" => Ok(
        SubstringFunc::ExplicitCall(Some(vec![
            Unnamed(StringConst("foo".into())),
            FuncArgExpr::NamedValue {
                name: "bar".into(),
                value: IntegerConst(1)
            }
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

use super::extract_list::extract_args;
use crate::combinators::expr::a_expr;
use crate::combinators::expr::b_expr;
use crate::combinators::expr::expr_primary::cast_expr;
use crate::combinators::expr::unicode_normal_form;
use crate::combinators::expr_list;
use crate::combinators::expr_list_paren;
use crate::combinators::foundation::between_paren;
use crate::combinators::foundation::or;
use crate::combinators::foundation::skip_prefix;
use crate::combinators::foundation::Combinator;
use crate::combinators::func_arg_list;
use crate::combinators::precision;
use crate::combinators::typename;
use crate::no_match;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
use core::mem;
use pg_ast::ExprNode;
use pg_ast::ExprNode::CoalesceExpr;
use pg_ast::ExprNode::CollationFor;
use pg_ast::ExprNode::CurrentCatalog;
use pg_ast::ExprNode::CurrentDate;
use pg_ast::ExprNode::CurrentRole;
use pg_ast::ExprNode::CurrentSchema;
use pg_ast::ExprNode::CurrentTime;
use pg_ast::ExprNode::CurrentTimestamp;
use pg_ast::ExprNode::CurrentUser;
use pg_ast::ExprNode::GreatestFunc;
use pg_ast::ExprNode::IntegerConst;
use pg_ast::ExprNode::LeastFunc;
use pg_ast::ExprNode::LocalTime;
use pg_ast::ExprNode::LocalTimestamp;
use pg_ast::ExprNode::MergeSupportFunc;
use pg_ast::ExprNode::NullConst;
use pg_ast::ExprNode::NullIf;
use pg_ast::ExprNode::SessionUser;
use pg_ast::ExprNode::SystemUser;
use pg_ast::ExprNode::Treat;
use pg_ast::ExprNode::User;
use pg_ast::ExtractFunc;
use pg_ast::FuncArgExpr;
use pg_ast::FuncArgExpr::Unnamed;
use pg_ast::NormalizeFunc;
use pg_ast::OverlayFunc;
use pg_ast::OverlaySqlArgs;
use pg_ast::PositionFunc;
use pg_ast::SubstringFunc;
use pg_ast::TrimFunc;
use pg_ast::TrimSide;
use pg_ast::TypeName;
use pg_ast::TypecastExpr;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::As;
use pg_lexer::Keyword::Coalesce;
use pg_lexer::Keyword::Collation;
use pg_lexer::Keyword::Escape;
use pg_lexer::Keyword::Extract;
use pg_lexer::Keyword::For;
use pg_lexer::Keyword::FromKw;
use pg_lexer::Keyword::Greatest;
use pg_lexer::Keyword::In;
use pg_lexer::Keyword::Least;
use pg_lexer::Keyword::MergeAction;
use pg_lexer::Keyword::Normalize;
use pg_lexer::Keyword::Nullif;
use pg_lexer::Keyword::Overlay;
use pg_lexer::Keyword::Placing;
use pg_lexer::Keyword::Position;
use pg_lexer::Keyword::Similar;
use pg_lexer::Keyword::Substring;
use pg_lexer::Keyword::Trim;
use pg_lexer::OperatorKind::CloseParenthesis;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::OpenParenthesis;
