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
        .map(|(name, args)|
            OperatorWithArgs::new(name, args)
        )
}

fn oper_argtypes() -> impl Combinator<Output = OneOrBoth<Type>> {

    /*
          '(' NONE ',' Typename ')'
        | '(' Typename ',' Typename ')'
        | '(' Typename ',' NONE ')'
        | '(' Typename ')' => Err
    */

    between_paren(match_first! {
        NoneKw.and(Comma)
            .and_right(typename())
            .map(OneOrBoth::Right),
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
                |typ1, typ2| match typ2 {
                    Some(typ2) => OneOrBoth::Both(typ1, typ2),
                    None => OneOrBoth::Left(typ1)
                }
            )
    })
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
    use crate::parser::tests::test_parser;
    use postgres_parser_ast::Operator::Equals;
    use postgres_parser_ast::QualifiedOperator;
    use postgres_parser_ast::TypeName::Int4;
    use test_case::test_case;

    #[test]
    fn test_operator_with_argtypes_list() {
        test_parser!(
            source = "=(int, int), =(none, int), =(int, none)",
            parser = operator_with_argtypes_list(),
            expected = vec![
                OperatorWithArgs::new(QualifiedOperator(vec![], Equals), OneOrBoth::Both(Int4.into(), Int4.into())),
                OperatorWithArgs::new(QualifiedOperator(vec![], Equals), OneOrBoth::Right(Int4.into())),
                OperatorWithArgs::new(QualifiedOperator(vec![], Equals), OneOrBoth::Left(Int4.into()))
            ]
        )
    }

    #[test_case("= (int,  int)", OneOrBoth::Both(Int4.into(), Int4.into()))]
    #[test_case("= (none, int)", OneOrBoth::Right(Int4.into()))]
    #[test_case("= (int,  none)", OneOrBoth::Left(Int4.into()))]
    fn test_operator_with_argtypes(source: &str, expected: OneOrBoth<Type>) {
        test_parser!(
            source = source,
            parser = operator_with_argtypes(),
            expected = OperatorWithArgs::new(QualifiedOperator(vec![], Equals), expected)
        )
    }

    #[test_case("(int, int)", OneOrBoth::Both(Int4.into(), Int4.into()))]
    #[test_case("(none, int)", OneOrBoth::Right(Int4.into()))]
    #[test_case("(int, none)", OneOrBoth::Left(Int4.into()))]
    fn test_oper_argtypes(source: &str, expected: OneOrBoth<Type>) {
        test_parser!(source, oper_argtypes(), expected);
    }
}

use crate::parser::combinators::between_paren;
use crate::parser::combinators::foundation::located;
use crate::parser::combinators::foundation::many_sep;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::operators::any_operator;
use crate::parser::combinators::typename;
use crate::parser::result::ScanErrorKind::ScanErr;
use elog::parser::ParserError;
use elog::parser::ParserErrorKind::MissingOperatorArgumentType;
use postgres_parser_ast::OneOrBoth;
use postgres_parser_ast::OperatorWithArgs;
use postgres_parser_ast::Type;
use postgres_parser_lexer::Keyword::NoneKw;
use postgres_parser_lexer::OperatorKind::CloseParenthesis;
use postgres_parser_lexer::OperatorKind::Comma;
