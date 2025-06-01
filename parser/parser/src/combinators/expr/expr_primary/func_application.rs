pub(super) fn func_application_args() -> impl Combinator<Output = FuncArgsKind> {

    /*
        '(' (
              '*'
            | ALL      func_arg_list ( sort_clause )?
            | DISTINCT func_arg_list ( sort_clause )?
            | variadic_func_arg_list ( sort_clause )?
        )? ')'
    */

    between_paren(
        match_first! {
            star_args(),
            all_args(),
            distinct_args(),
            simple_args(),
        }
            .optional()
            .map(|args| {
                args.unwrap_or(Empty { order_within_group: None })
            })
    )
}

fn star_args() -> impl Combinator<Output = FuncArgsKind> {
    Mul.map(|_| Star { order_within_group: None })
}

fn all_args() -> impl Combinator<Output = FuncArgsKind> {

    Kw::All
        .and_right(and(
            func_arg_list(),
            sort_clause().optional()
        ))
        .map(|(args, order)|
            All {
                args,
                order: order.map(FuncArgsOrder::OrderBy)
            }
        )
}

fn distinct_args() -> impl Combinator<Output = FuncArgsKind> {

    Kw::Distinct
        .and_right(and(
            func_arg_list(),
            sort_clause().optional()
        ))
        .map(|(args, order)|
            Distinct { args, order }
        )
}

fn simple_args() -> impl Combinator<Output = FuncArgsKind> {

    and(
        variadic_func_args(),
        sort_clause().optional()
    )
        .map(|((args, variadic), order)| {
            if variadic {
                Variadic { args, order }
            }
            else {
                All {
                    args,
                    order: order.map(FuncArgsOrder::OrderBy)
                }
            }
        })
}

fn variadic_func_args() -> impl Combinator<Output = (Vec<FuncArgExpr>, bool)> {

    /*
          func_arg_list
        | VARIADIC func_arg_expr
        | func_arg_list ',' VARIADIC func_arg_expr

        In this case, it's easier to allow VARIADIC for all arguments,
        and then check if none or only the last argument is VARIADIC.
    */

    variadic_args()
        .map_result(|res| {
            sanitize_variadic_args(res?)
        })
}

fn sanitize_variadic_args(
    args: Vec<(FuncArgExpr, Option<Location>)>
)
    -> ScanResult<(Vec<FuncArgExpr>, bool)>
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

    if variadics.len() > 1 {
        return Err(syntax(loc.clone()))
    }

    if *index != args.len() - 1 {
        // Variadic argument can only be the last one
        return Err(syntax(loc.clone()))
    }

    Ok((args, true))
}

fn variadic_args() -> impl Combinator<Output = Vec<(FuncArgExpr, Option<Location>)>> {

    /*
        ( VARIADIC )? func_arg_expr ( ',' ( VARIADIC )? func_arg_expr )*
    */

    many_sep(Comma, variadic_arg())
}

fn variadic_arg() -> impl Combinator<Output = (FuncArgExpr, Option<Location>)> {

    /*
        ( VARIADIC )? func_arg_expr
    */

    or(
        located(Kw::Variadic)
            .and_then(func_arg_expr(), |(_, loc), arg|
                (arg, Some(loc))
            ),
        func_arg_expr()
            .map(|arg| (arg, None)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::ExprNode::IntegerConst;
    use pg_ast::FuncArgExpr::NamedValue;
    use pg_ast::FuncArgExpr::Unnamed;
    use test_case::test_case;

    #[test_case("(*)", Star { order_within_group: None })]
    #[test_case("(all 1, 2)", All {
        args: vec![Unnamed(IntegerConst(1)), Unnamed(IntegerConst(2))],
        order: None
    })]
    #[test_case("(distinct 1, 2)", Distinct {
        args: vec![Unnamed(IntegerConst(1)), Unnamed(IntegerConst(2))],
        order: None
    })]
    #[test_case("(1, 2, 3)", All {
        args: vec![Unnamed(IntegerConst(1)), Unnamed(IntegerConst(2)), Unnamed(IntegerConst(3))],
        order: None
    })]
    #[test_case("(variadic 1)", Variadic {
        args: vec![Unnamed(IntegerConst(1))],
        order: None
    })]
    #[test_case("()", Empty { order_within_group: None })]
    fn test_func_application_args(source: &str, expected: FuncArgsKind) {
        test_parser!(source, func_application_args(), expected);
    }

    #[test_case("1, 2, variadic 3", (
        vec![
            Unnamed(IntegerConst(1)),
            Unnamed(IntegerConst(2)),
            Unnamed(IntegerConst(3)),
        ],
        true
    ))]
    #[test_case("1, 2, 3", (
        vec![
            Unnamed(IntegerConst(1)),
            Unnamed(IntegerConst(2)),
            Unnamed(IntegerConst(3)),
        ],
        false
    ))]
    fn test_variadic_func_args(source: &str, expected: (Vec<FuncArgExpr>, bool)) {
        test_parser!(source, variadic_func_args(), expected);
    }

    #[test]
    fn test_variadic_args() {
        test_parser!(
            source = "1, variadic 2, 3, variadic foo := 4, bar => 5",
            parser = variadic_args(),
            expected = vec![
                (Unnamed(IntegerConst(1)), None),
                (Unnamed(IntegerConst(2)), Some(Location::new(3..11, 1, 4))),
                (Unnamed(IntegerConst(3)), None),
                (NamedValue { name: "foo".into(), value: IntegerConst(4) }, Some(Location::new(18..26, 1, 19))),
                (NamedValue { name: "bar".into(), value: IntegerConst(5) }, None),
            ]
        )
    }

    #[test_case("1",
        Unnamed(IntegerConst(1)),
        None
    )]
    #[test_case("VARIADIC 2",
        Unnamed(IntegerConst(2)),
        Some(Location::new(0..8, 1, 1))
    )]
    #[test_case("foo := 3",
        NamedValue {
            name: "foo".into(),
            value: IntegerConst(3)
        },
        None
    )]
    #[test_case("VARIADIC bar => 4",
        NamedValue { name: "bar".into(), value: IntegerConst(4) },
        Some(Location::new(0..8, 1, 1))
    )]
    fn test_variadic_arg(source: &str, expected: FuncArgExpr, loc: Option<Location>) {
        test_parser!(source, variadic_arg(), (expected, loc));
    }
}

use crate::combinators::between_paren;
use crate::combinators::foundation::and;
use crate::combinators::foundation::located;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::func_arg_expr;
use crate::combinators::func_arg_list;
use crate::combinators::sort_clause;
use crate::result::ScanResult;
use pg_ast::FuncArgExpr;
use pg_ast::FuncArgsKind;
use pg_ast::FuncArgsKind::All;
use pg_ast::FuncArgsKind::Distinct;
use pg_ast::FuncArgsKind::Empty;
use pg_ast::FuncArgsKind::Star;
use pg_ast::FuncArgsKind::Variadic;
use pg_ast::FuncArgsOrder;
use pg_basics::Location;
use pg_elog::syntax;
use pg_lexer::Keyword as Kw;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::Mul;
