pub(super) fn operator_with_argtypes_list(stream: &mut TokenStream) -> scan::Result<Vec<OperatorWithArgs>> {

    /*
        operator_with_argtypes ( ',' operator_with_argtypes )*
    */

    many_sep(Comma, operator_with_argtypes).parse(stream)
}

pub(super) fn operator_with_argtypes(stream: &mut TokenStream) -> scan::Result<OperatorWithArgs> {

    /*
        any_operator oper_argtypes
    */

    let (name, args) = (any_operator, oper_argtypes).parse(stream)?;

    Ok(OperatorWithArgs::new(name, args))
}

fn oper_argtypes(stream: &mut TokenStream) -> scan::Result<OneOrBoth<Type>> {

    /*
          '(' NONE ',' Typename ')'
        | '(' Typename ',' Typename ')'
        | '(' Typename ',' NONE ')'
        | '(' Typename ')' => Err
    */

    between_paren(oper_argtypes_between).parse(stream)
}

fn oper_argtypes_between(stream: &mut TokenStream) -> scan::Result<OneOrBoth<Type>> {
    or((
        (NoneKw, Comma, typename)
            .map(|(.., typ)| OneOrBoth::Right(typ)),
        (
            typename,
            or((
                close_paren,
                (
                    Comma,
                    or((
                        NoneKw.map(|_| None),
                        typename.map(Some)
                    ))
                ).map(|(_, typ)| typ)
            ))
        )
            .map(|(typ1, typ2)| match typ2 {
                Some(typ2) => OneOrBoth::Both(typ1, typ2),
                None => OneOrBoth::Left(typ1)
            })
    )).parse(stream)
}

fn close_paren(stream: &mut TokenStream) -> scan::Result<Option<Type>> {

    let (_, loc) = located(CloseParenthesis).parse(stream)?;
    let err = MissingOperatorArgumentType.at(loc).into();
    Err(ScanErr(err))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::Operator::Equals;
    use pg_ast::QualifiedOperator;
    use pg_ast::TypeName::Int4;
    use test_case::test_case;

    #[test]
    fn test_operator_with_argtypes_list() {
        test_parser!(
            source = "=(int, int), =(none, int), =(int, none)",
            parser = operator_with_argtypes_list,
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
            parser = operator_with_argtypes,
            expected = OperatorWithArgs::new(QualifiedOperator(vec![], Equals), expected)
        )
    }

    #[test_case("(int, int)", OneOrBoth::Both(Int4.into(), Int4.into()))]
    #[test_case("(none, int)", OneOrBoth::Right(Int4.into()))]
    #[test_case("(int, none)", OneOrBoth::Left(Int4.into()))]
    fn test_oper_argtypes(source: &str, expected: OneOrBoth<Type>) {
        test_parser!(source, oper_argtypes, expected);
    }
}

use crate::combinators::foundation::between_paren;
use crate::combinators::foundation::located;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::operators::any_operator;
use crate::combinators::typename;
use crate::scan;
use crate::scan::Error::ScanErr;
use crate::stream::TokenStream;
use pg_ast::OneOrBoth;
use pg_ast::OperatorWithArgs;
use pg_ast::Type;
use pg_elog::parser::Error::MissingOperatorArgumentType;
use pg_lexer::Keyword::NoneKw;
use pg_lexer::OperatorKind::CloseParenthesis;
use pg_lexer::OperatorKind::Comma;
