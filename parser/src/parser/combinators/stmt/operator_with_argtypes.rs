pub(super) fn operator_with_argtypes_list() -> impl Combinator<Output = Vec<OperatorWithArgs>> {

    /*
        operator_with_argtypes ( ',' operator_with_argtypes )*
    */

    many_sep(Comma, operator_with_argtypes())
}

pub(super) fn operator_with_argtypes() -> impl Combinator<Output = OperatorWithArgs> {

    /*
        any_operator oper_argtypes
    */

    sequence!(any_operator(), oper_argtypes())
        .map(|(name, (left_arg, right_arg))|
            OperatorWithArgs::new(name, left_arg, right_arg)
        )
}

fn oper_argtypes() -> impl Combinator<Output = (Option<Type>, Option<Type>)> {

    /*
          '(' NONE ',' Typename ')'
        | '(' Typename ',' Typename ')'
        | '(' Typename ',' NONE ')'
        | '(' Typename ')' => Err
    */

    between(
        OpenParenthesis,
        match_first! {
            NoneKw.and(Comma)
                .and_right(typename())
                .map(|typ| (None, Some(typ))),
            typename()
                .and_left(or(
                    close_paren(),
                    Comma.skip()
                ))
                .and_then(
                    or(
                        NoneKw.map(|_| None),
                        typename().map(Some)
                    ),
                    |typ1, typ2| (Some(typ1), typ2)
                )
        },
        CloseParenthesis
    )
}

fn close_paren() -> impl Combinator<Output = ()> {

    located(CloseParenthesis).map_result(|res| match res {
        Ok((_, loc)) => {
            let err = ParserError::new(MissingOperatorArgumentType, loc);
            Err(ScanErr(err))
        }
        Err(err) => Err(err)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::Operator::Equals;
    use crate::parser::ast_node::QualifiedOperator;
    use crate::parser::ast_node::Type;
    use crate::parser::ast_node::TypeName::Int4;
    use crate::parser::tests::test_parser;
    use test_case::test_case;

    #[test]
    fn test_operator_with_argtypes_list() {
        test_parser!(
            source = "=(int, int), =(none, int), =(int, none)",
            parser = operator_with_argtypes_list(),
            expected = vec![
                OperatorWithArgs::new(QualifiedOperator(vec![], Equals), Some(Int4.into()), Some(Int4.into())),
                OperatorWithArgs::new(QualifiedOperator(vec![], Equals), None, Some(Int4.into())),
                OperatorWithArgs::new(QualifiedOperator(vec![], Equals), Some(Int4.into()), None)
            ]
        )
    }

    #[test_case("= (int,  int)", Some(Int4.into()), Some(Int4.into()))]
    #[test_case("= (none, int)", None, Some(Int4.into()))]
    #[test_case("= (int,  none)", Some(Int4.into()), None)]
    fn test_operator_with_argtypes(source: &str, left: Option<Type>, right: Option<Type>) {
        test_parser!(
            source = source,
            parser = operator_with_argtypes(),
            expected = OperatorWithArgs::new(QualifiedOperator(vec![], Equals), left, right)
        )
    }

    #[test_case("(int, int)", (Some(Int4.into()), Some(Int4.into())))]
    #[test_case("(none, int)", (None, Some(Int4.into())))]
    #[test_case("(int, none)", (Some(Int4.into()), None))]
    fn test_oper_argtypes(source: &str, expected: (Option<Type>, Option<Type>)) {
        test_parser!(source, oper_argtypes(), expected);
    }
}

use crate::lexer::Keyword::NoneKw;
use crate::lexer::OperatorKind::CloseParenthesis;
use crate::lexer::OperatorKind::Comma;
use crate::lexer::OperatorKind::OpenParenthesis;
use crate::parser::ast_node::OperatorWithArgs;
use crate::parser::ast_node::Type;
use crate::parser::combinators::foundation::located;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::foundation::{between, many_sep};
use crate::parser::combinators::operators::any_operator;
use crate::parser::combinators::typename;
use crate::parser::result::ScanErrorKind::ScanErr;
use crate::parser::ParserError;
use crate::parser::ParserErrorKind::MissingOperatorArgumentType;
