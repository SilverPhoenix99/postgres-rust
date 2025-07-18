pub(super) fn aggregate_with_argtypes_list(stream: &mut TokenStream) -> scan::Result<Vec<AggregateWithArgs>> {

    /*
        aggr_func ( ',' aggr_func )*
    */

    many_sep(Comma, aggregate_with_argtypes).parse(stream)
}

pub(super) fn aggregate_with_argtypes(stream: &mut TokenStream) -> scan::Result<AggregateWithArgs> {

    /*
        func_name aggr_args
    */

    let (name, (args, order_by)) = (func_name, aggr_args).parse(stream)?;

    Ok(AggregateWithArgs::new(name, args, order_by))
}

/// Either `Vec` can be empty.
/// When both `Vec`s are empty, it means `(*)` was used.
pub(super) fn aggr_args(stream: &mut TokenStream) -> scan::Result<(Vec<FunctionParameter>, Vec<FunctionParameter>)> {

    /*
          '(' '*' ')'
        | '(' ORDER BY aggr_args_list ')'
        | '(' aggr_args_list ( ORDER BY aggr_args_list )? ')'
    */

    between_paren(any_aggr_arg).parse(stream)
}

pub(super) fn any_aggr_arg(stream: &mut TokenStream) -> scan::Result<(Vec<FunctionParameter>, Vec<FunctionParameter>)> {
    or((
        Mul.map(|_| (Vec::new(), Vec::new())),
        order_by_aggr_args.map(|args| (Vec::new(), args)),
        (
            aggr_args_list,
            order_by_aggr_args
                .optional()
                .map(Option::unwrap_or_default)
        )
    )).parse(stream)
}

fn order_by_aggr_args(stream: &mut TokenStream) -> scan::Result<Vec<FunctionParameter>> {

    /*
        ORDER BY aggr_args_list
    */

    let (.., args) = (Order, By, aggr_args_list).parse(stream)?;

    Ok(args)
}

fn aggr_args_list(stream: &mut TokenStream) -> scan::Result<Vec<FunctionParameter>> {

    /*
        aggr_arg ( ',' aggr_arg )*
    */

    many_sep(Comma, aggr_arg).parse(stream)
}

fn aggr_arg(stream: &mut TokenStream) -> scan::Result<FunctionParameter> {

    let (param, loc) = located(func_arg).parse(stream)?;

    if matches!(param.mode(), Mode::Default | Mode::In | Mode::Variadic) {
        return Ok(param)
    }

    let err = AggregateWithOutputParameters.at(loc).into();
    Err(ScanErr(err))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::FuncType;
    use pg_ast::TypeName::Int4;
    use pg_ast::TypeName::Int8;
    use pg_ast::TypeName::Json;
    use test_case::test_case;

    #[test]
    fn test_aggregate_with_argtypes_list() {
        test_parser!(
            source = "agg_name(json), agg_name(*)",
            parser = aggregate_with_argtypes_list,
            expected = vec![
                AggregateWithArgs::new(
                    vec!["agg_name".into()],
                    vec![FuncType::Type(Json.into()).into()],
                    vec![]
                ),
                AggregateWithArgs::new(vec!["agg_name".into()], vec![], vec![])
            ]
        )
    }

    #[test]
    fn test_aggregate_with_argtypes() {
        test_parser!(
            source = "agg_name(json, int order by bigint)",
            parser = aggregate_with_argtypes,
            expected = AggregateWithArgs::new(
                vec!["agg_name".into()],
                vec![
                    FuncType::Type(Json.into()).into(),
                    FuncType::Type(Int4.into()).into(),
                ],
                vec![
                    FuncType::Type(Int8.into()).into(),
                ]
            )
        )
    }

    #[test_case("(*)", vec![], vec![])]
    #[test_case("(int, json)",
        vec![
            FuncType::Type(Int4.into()).into(),
            FuncType::Type(Json.into()).into(),
        ],
        vec![]
    )]
    #[test_case("(order by bigint, int)",
        vec![],
        vec![
            FuncType::Type(Int8.into()).into(),
            FuncType::Type(Int4.into()).into(),
        ]
    )]
    #[test_case("(int, bigint order by json, bigint)",
        vec![
            FuncType::Type(Int4.into()).into(),
            FuncType::Type(Int8.into()).into(),
        ],
        vec![
            FuncType::Type(Json.into()).into(),
            FuncType::Type(Int8.into()).into(),
        ]
    )]
    fn test_aggr_args(source: &str, args: Vec<FunctionParameter>, order_by: Vec<FunctionParameter>) {
        test_parser!(source, aggr_args, (args, order_by))
    }

    #[test]
    fn test_order_by_aggr_args() {
        test_parser!(
            source = "ORDER BY bigint, var2 json",
            parser = order_by_aggr_args,
            expected = vec![
                FuncType::Type(Int8.into()).into(),
                FunctionParameter::new(
                    Some("var2".into()),
                    Mode::Default,
                    FuncType::Type(Json.into())
                )
            ]
        )
    }

    #[test]
    fn test_aggr_arg_list() {
        test_parser!(
            source = "tis json, tis_an int",
            parser = aggr_args_list,
            expected = vec![
                FunctionParameter::new(
                    Some("tis".into()),
                    Mode::Default,
                    FuncType::Type(Json.into())
                ),
                FunctionParameter::new(
                    Some("tis_an".into()),
                    Mode::Default,
                    FuncType::Type(Int4.into())
                )
            ]
        )
    }

    #[test]
    fn test_aggr_arg() {
        test_parser!(
            source = "tis json",
            parser = aggr_arg,
            expected = FunctionParameter::new(
                Some("tis".into()),
                Mode::Default,
                FuncType::Type(Json.into())
            )
        )
    }
}

use crate::combinators::foundation::between_paren;
use crate::combinators::foundation::located;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::func_arg;
use crate::combinators::func_name;
use crate::scan;
use crate::scan::Error::ScanErr;
use crate::stream::TokenStream;
use pg_ast::AggregateWithArgs;
use pg_ast::FunctionParameter;
use pg_ast::FunctionParameterMode as Mode;
use pg_elog::parser::Error::AggregateWithOutputParameters;
use pg_lexer::Keyword::By;
use pg_lexer::Keyword::Order;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::Mul;
