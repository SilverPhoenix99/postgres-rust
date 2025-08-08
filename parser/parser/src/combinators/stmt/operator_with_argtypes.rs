pub(super) fn operator_with_argtypes_list(stream: &mut TokenStream) -> scan::Result<Vec<OperatorWithArgs>> {

    /*
        operator_with_argtypes ( ',' operator_with_argtypes )*
    */

    many!(sep = Comma, operator_with_argtypes).parse(stream)
}

pub(super) fn operator_with_argtypes(stream: &mut TokenStream) -> scan::Result<OperatorWithArgs> {

    /*
        any_operator oper_argtypes
    */

    let (name, args) = seq!(any_operator, oper_argtypes).parse(stream)?;

    Ok(OperatorWithArgs::new(name, args))
}

fn oper_argtypes(stream: &mut TokenStream) -> scan::Result<OneOrBoth<Type>> {

    /*
          '(' NONE ',' Typename ')'
        | '(' Typename ',' Typename ')'
        | '(' Typename ',' NONE ')'
        | '(' Typename ')' => Err
    */

    paren!(oper_argtypes_between).parse(stream)
}

fn oper_argtypes_between(stream: &mut TokenStream) -> scan::Result<OneOrBoth<Type>> {
    alt!(
        none_type,
        both_types
    ).parse(stream)
}

fn none_type(stream: &mut TokenStream) -> scan::Result<OneOrBoth<Type>> {

    /*
        '(' NONE ',' Typename ')'
    */

    let (.., typ) = seq!(NoneKw, Comma, typename)
        .parse(stream)?;

    Ok(OneOrBoth::Right(typ))
}

fn both_types(stream: &mut TokenStream) -> scan::Result<OneOrBoth<Type>> {

    /*
        '(' Typename ',' (Typename | NONE) ')'
    */

    let (typ1, typ2) = seq!(typename, right_type)
        .parse(stream)?;

    let pair = match typ2 {
        Some(typ2) => OneOrBoth::Both(typ1, typ2),
        None => OneOrBoth::Left(typ1)
    };

    Ok(pair)
}

/// The `Option` result does not come from an absence of value.
/// It returns `None` when the token is the keyword `NONE`.
fn right_type(stream: &mut TokenStream) -> scan::Result<Option<Type>> {

    /*
          ',' Typename ')'
        | ',' NONE ')'
        | ')' => Err
    */

    alt!(
        close_paren,
        seq!(
            Comma,
            alt!(
                NoneKw.map(|_| None),
                typename.map(Some)
            )
        ).map(|(_, typ)| typ)
    ).parse(stream)
}

/// The `Result<Option>` needs to match the caller's return type.
fn close_paren(stream: &mut TokenStream) -> scan::Result<Option<Type>> {

    let (_, loc) = located!(CloseParenthesis).parse(stream)?;
    let err = MissingOperatorArgumentType.at(loc).into();
    Err(ScanErr(err))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_ast::TypeName::Int4;
    use pg_combinators::test_parser;
    use pg_sink_ast::Operator::Equals;
    use pg_sink_ast::QualifiedOperator;
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

use crate::combinators::typename;
use pg_ast::OneOrBoth;
use pg_ast::OperatorWithArgs;
use pg_ast::Type;
use pg_combinators::alt;
use pg_combinators::located;
use pg_combinators::many;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_elog::parser::Error::MissingOperatorArgumentType;
use pg_lexer::Keyword::NoneKw;
use pg_lexer::OperatorKind::CloseParenthesis;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_parser_core::scan::Error::ScanErr;
use pg_parser_core::stream::TokenStream;
use pg_sink_combinators::any_operator;
