pub(super) fn func_application(ctx: &mut ParserContext) -> scan::Result<FuncCall> {

    /*
        func_name func_application_args
    */

    let (name, args) = seq!(func_name, func_application_args)
        .parse(ctx)?;

    let func_call = FuncCall::new(name, args);
    Ok(func_call)
}

pub(super) fn func_application_args(ctx: &mut ParserContext) -> scan::Result<FuncArgsKind> {

    /*
        '(' ( func_call_args )? ')'
    */

    let args = paren!(func_call_args.optional())
        .parse(ctx)?
        .unwrap_or(Empty { order_within_group: None });

    Ok(args)
}

fn func_call_args(ctx: &mut ParserContext) -> scan::Result<FuncArgsKind> {

    /*
          '*'
        | ALL      func_arg_list ( sort_clause )?
        | DISTINCT func_arg_list ( sort_clause )?
        | variadic_func_arg_list ( sort_clause )?
    */

    alt!(
        wildcard_args,
        all_args,
        distinct_args,
        simple_args,
    ).parse(ctx)
}

fn wildcard_args(ctx: &mut ParserContext) -> scan::Result<FuncArgsKind> {

    let _ = Mul.parse(ctx)?;
    Ok(Wildcard { order_within_group: None })
}

fn all_args(ctx: &mut ParserContext) -> scan::Result<FuncArgsKind> {

    let (_, args, order) = seq!(
        Kw::All,
        func_arg_list,
        sort_clause.optional()
    ).parse(ctx)?;

    let args = All {
        args,
        order: order.map(|Located(order, loc)|
            Located(FuncArgsOrder::OrderBy(order), loc)
        )
    };

    Ok(args)
}

fn distinct_args(ctx: &mut ParserContext) -> scan::Result<FuncArgsKind> {

    let (_, args, order) = seq!(
        Kw::Distinct,
        func_arg_list,
        sort_clause.optional()
    ).parse(ctx)?;

    let args = args.into_iter()
        .map(|Located(arg, _)| arg)
        .collect();

    let order = order.map(|Located(order, _)| order);

    Ok(Distinct { args, order })
}

fn simple_args(ctx: &mut ParserContext) -> scan::Result<FuncArgsKind> {

    let ((args, variadic), order) = seq!(
        variadic_func_args,
        sort_clause.optional()
    ).parse(ctx)?;

    if variadic {

        let args = args.into_iter()
            .map(|Located(arg, _)| arg)
            .collect();

        let order = order.map(|Located(order, _)| order);

        return Ok(Variadic { args, order })
    }

    let order = order.map(|Located(order, loc)|
        Located(FuncArgsOrder::OrderBy(order), loc)
    );

    Ok(All { args, order })
}

fn variadic_func_args(ctx: &mut ParserContext) -> scan::Result<(Vec<Located<NamedValue>>, bool)> {

    /*
          func_arg_list
        | VARIADIC func_arg_expr
        | func_arg_list ',' VARIADIC func_arg_expr

        In this case, it's easier to allow VARIADIC for all arguments,
        and then check if none or only the last argument is VARIADIC.
    */

    let args = variadic_args(ctx)?;
    sanitize_variadic_args(args)
}

fn sanitize_variadic_args(
    args: Vec<(Located<NamedValue>, Option<Location>)>
)
    -> scan::Result<(Vec<Located<NamedValue>>, bool)>
{
    let (args, variadics): (Vec<_>, Vec<_>) = args.into_iter()
        .enumerate()
        .map(|(index, (arg, loc))|
            match loc {
                Some(loc) => (arg, Some((index, loc))),
                None => (arg, None),
            }
        )
        .unzip();

    // Strip all `None` from variadics.
    // NB: No point in checking more than 2 elements
    let variadics: Vec<(usize, Location)> = variadics.into_iter()
        .flatten()
        .take(2)
        .collect();

    if variadics.is_empty() {
        return Ok((args, false))
    }

    let (index, loc) = variadics.first().expect("Vec is not empty");

    if {
        variadics.len() > 1 // There can only be 1 variadic argument, if any.
        || *index != args.len() - 1 // Variadic argument can only be the last one
    } {
        return Err(syntax(loc.clone()))
    }

    Ok((args, true))
}

fn variadic_args(ctx: &mut ParserContext) -> scan::Result<Vec<(Located<NamedValue>, Option<Location>)>> {

    /*
        ( VARIADIC )? func_arg_expr ( ',' ( VARIADIC )? func_arg_expr )*
    */

    many!(sep = Comma, variadic_arg).parse(ctx)
}

fn variadic_arg(ctx: &mut ParserContext) -> scan::Result<(Located<NamedValue>, Option<Location>)> {

    /*
        ( VARIADIC )? func_arg_expr
    */

    alt!(
        seq!(located!(Kw::Variadic), func_arg_expr)
            .map(|(Located(_, loc), arg)| (arg, Some(loc))),
        func_arg_expr
            .map(|arg| (arg, None)),
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_ast::ExprNode::IntegerConst;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test]
    fn test_func_application() {
        test_parser!(
            source = "foo(*)",
            parser = func_application,
            expected = FuncCall::new(
                vec!["foo".into()],
                Wildcard { order_within_group: None }
            )
        )
    }

    #[test_case("(*)" => Ok(Wildcard { order_within_group: None }))]
    #[test_case("(distinct 1, 2)" => Ok(Distinct {
        args: vec![
            NamedValue::unnamed(IntegerConst(1)),
            NamedValue::unnamed(IntegerConst(2)),
        ],
        order: None
    }))]
    #[test_case("(variadic 1)" => Ok(Variadic {
        args: vec![NamedValue::unnamed(IntegerConst(1))],
        order: None
    }))]
    #[test_case("()" => Ok(Empty { order_within_group: None }))]
    fn test_func_application_args(source: &str) -> scan::Result<FuncArgsKind> {
        test_parser!(source, func_application_args)
    }

    #[test]
    fn test_func_application_args_all() {
        let mut ctx = ParserContext::from("(all 1, 2)");
        let actual = func_application_args(&mut ctx).unwrap();

        let All { args, order: None } = actual else {
            panic!("Expected All variant, but got {actual:?}");
        };

        let args = args.into_iter()
            .map(|Located(arg, _)| arg)
            .collect::<Vec<_>>();

        assert_eq!(
            vec![
                NamedValue::unnamed(IntegerConst(1)),
                NamedValue::unnamed(IntegerConst(2)),
            ],
            args
        )
    }

    #[test]
    fn test_func_application_args_simple() {
        let mut ctx = ParserContext::from("(1, 2, 3)");
        let actual = func_application_args(&mut ctx).unwrap();

        let All { args, order: None } = actual else {
            panic!("Expected All variant, but got {actual:?}");
        };

        let args = args.into_iter()
            .map(|Located(arg, _)| arg)
            .collect::<Vec<_>>();

        assert_eq!(
            vec![
                NamedValue::unnamed(IntegerConst(1)),
                NamedValue::unnamed(IntegerConst(2)),
                NamedValue::unnamed(IntegerConst(3)),
            ],
            args
        )
    }

    #[test_case("1, 2, variadic 3" => Ok((
        vec![
            NamedValue::unnamed(IntegerConst(1)),
            NamedValue::unnamed(IntegerConst(2)),
            NamedValue::unnamed(IntegerConst(3)),
        ],
        true
    )))]
    #[test_case("1, 2, 3" => Ok((
        vec![
            NamedValue::unnamed(IntegerConst(1)),
            NamedValue::unnamed(IntegerConst(2)),
            NamedValue::unnamed(IntegerConst(3)),
        ],
        false
    )))]
    fn test_variadic_func_args(source: &str) -> scan::Result<(Vec<NamedValue>, bool)> {
        test_parser!(
            source,
            variadic_func_args
                .map(|(args, is_variadic)| {
                    let args = args.into_iter()
                        .map(|Located(arg, _)| arg)
                        .collect::<Vec<_>>();

                    (args, is_variadic)
                })
        )
    }

    #[test]
    fn test_variadic_args() {

        test_parser!(
            source = "1, variadic 2, 3, variadic foo := 4, bar => 5",
            parser = variadic_args
                .map(|args|
                    args.into_iter()
                        .map(|(Located(arg, _), _)| arg)
                        .collect::<Vec<_>>()
                ),
            expected = vec![
                NamedValue::unnamed(IntegerConst(1)),
                NamedValue::unnamed(IntegerConst(2)),
                NamedValue::unnamed(IntegerConst(3)),
                NamedValue::new(Some("foo".into()), IntegerConst(4)),
                NamedValue::new(Some("bar".into()), IntegerConst(5)),
            ]
        )
    }

    #[test_case("1" => Ok((
        NamedValue::unnamed(IntegerConst(1)),
        false
    )))]
    #[test_case("VARIADIC 2" => Ok((
        NamedValue::unnamed(IntegerConst(2)),
        true
    )))]
    #[test_case("foo := 3" => Ok((
        NamedValue::new(Some("foo".into()), IntegerConst(3)),
        false
    )))]
    #[test_case("VARIADIC bar => 4" => Ok((
        NamedValue::new(Some("bar".into()), IntegerConst(4)),
        true
    )))]
    fn test_variadic_arg(source: &str) -> scan::Result<(NamedValue, bool)> {
        test_parser!(source, variadic_arg)
            .map(|(Located(variadic_arg, _), loc)|
                (variadic_arg, loc.is_some())
            )
    }
}

use crate::combinators::func_arg_expr;
use crate::combinators::func_arg_list;
use crate::combinators::func_name;
use crate::combinators::sort_clause;
use pg_ast::FuncArgsKind;
use pg_ast::FuncArgsKind::All;
use pg_ast::FuncArgsKind::Distinct;
use pg_ast::FuncArgsKind::Empty;
use pg_ast::FuncArgsKind::Variadic;
use pg_ast::FuncArgsKind::Wildcard;
use pg_ast::FuncArgsOrder;
use pg_ast::FuncCall;
use pg_ast::NamedValue;
use pg_basics::Located;
use pg_basics::Location;
use pg_combinators::alt;
use pg_combinators::located;
use pg_combinators::many;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword as Kw;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::Mul;
use pg_parser_core::scan;
use pg_parser_core::syntax;
