pub(super) fn aggregate_with_argtypes() -> impl Combinator<Output = AggregateWithArgs> {

    /*
        func_name aggr_args
    */

    func_name().and_then(aggr_args(), |name, (args, order_by)|
        AggregateWithArgs::new(name, args, order_by)
    )
}

/// Either `Vec` can be empty.
/// When both `Vec`s are empty, it means `(*)` was used.
pub(super) fn aggr_args() -> impl Combinator<Output = (Vec<FunctionParameter>, Vec<FunctionParameter>)> {

    /*
          '(' '*' ')'
        | '(' ORDER BY aggr_args_list ')'
        | '(' aggr_args_list ( ORDER BY aggr_args_list )? ')'
    */

    between(
        OpenParenthesis,
        match_first! {
            Mul
                .map(|_| (Vec::new(), Vec::new())),
            order_by_aggr_args().map(|args| (Vec::new(), args)),
            sequence!(
                aggr_args_list(),
                order_by_aggr_args()
                    .optional()
                    .map(Option::unwrap_or_default)
            )
        },
        CloseParenthesis
    )
}

fn order_by_aggr_args() -> impl Combinator<Output = Vec<FunctionParameter>> {

    /*
        ORDER BY aggr_args_list
    */

    Order.and(By)
        .and_right(aggr_args_list())
}

fn aggr_args_list() -> impl Combinator<Output = Vec<FunctionParameter>> {

    /*
        aggr_arg ( ',' aggr_arg )*
    */

    many_sep(Comma, aggr_arg())
}

fn aggr_arg() -> impl Combinator<Output = FunctionParameter> {

    located(func_arg()).map_result(|res| {
        let (param, loc) = res?;
        if matches!(param.mode(), Mode::Default | Mode::In | Mode::Variadic) {
           return Ok(param)
        }

        let err = ParserError::new(AggregateWithOutputParameters, loc);
        Err(ScanErr(err))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::FuncType;
    use crate::parser::ast_node::TypeName::Int4;
    use crate::parser::ast_node::TypeName::Int8;
    use crate::parser::ast_node::TypeName::Json;
    use crate::parser::tests::test_parser;
    use test_case::test_case;

    #[test]
    fn test_aggregate_with_argtypes() {
        test_parser!(
            source = "agg_name(json, int order by bigint)",
            parser = aggregate_with_argtypes(),
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
        test_parser!(source, aggr_args(), (args, order_by))
    }

    #[test]
    fn test_order_by_aggr_args() {
        test_parser!(
            source = "ORDER BY bigint, var2 json",
            parser = order_by_aggr_args(),
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
            parser = aggr_args_list(),
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
            parser = aggr_arg(),
            expected = FunctionParameter::new(
                Some("tis".into()),
                Mode::Default,
                FuncType::Type(Json.into())
            )
        )
    }
}

use crate::lexer::Keyword::By;
use crate::lexer::Keyword::Order;
use crate::lexer::OperatorKind::CloseParenthesis;
use crate::lexer::OperatorKind::Comma;
use crate::lexer::OperatorKind::Mul;
use crate::lexer::OperatorKind::OpenParenthesis;
use crate::parser::ast_node::AggregateWithArgs;
use crate::parser::ast_node::FunctionParameter;
use crate::parser::ast_node::FunctionParameterMode as Mode;
use crate::parser::combinators::foundation::between;
use crate::parser::combinators::foundation::located;
use crate::parser::combinators::foundation::many_sep;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::func_arg::func_arg;
use crate::parser::combinators::func_name::func_name;
use crate::parser::result::ScanErrorKind::ScanErr;
use crate::parser::ParserError;
use crate::parser::ParserErrorKind::AggregateWithOutputParameters;
