/// Alias: `subquery_Op`
pub(super) fn subquery_op(stream: &mut TokenStream) -> scan::Result<QualifiedOperator> {

    // Intentionally excludes NOT LIKE/NOT ILIKE, due to conflicts.
    // Those will have to be checked separately.

    alt!(
        qual_all_op,
        like_op.map(From::from)
    ).parse(stream)
}

/// Alias: `qual_all_Op`
pub(super) fn qual_all_op(stream: &mut TokenStream) -> scan::Result<QualifiedOperator> {
    alt!(
        all_op.map(From::from),
        explicit_op
    ).parse(stream)
}

/// Alias: `qual_Op`
pub(super) fn qual_op(stream: &mut TokenStream) -> scan::Result<QualifiedOperator> {
    alt!(
        user_defined_operator
            .map(|op| UserDefined(op).into()),
        explicit_op,
    ).parse(stream)
}

pub(super) fn explicit_op(stream: &mut TokenStream) -> scan::Result<QualifiedOperator> {

    /*
        OPERATOR '(' any_operator ')'
    */

    let (_, op) = seq!(OperatorKw, paren!(any_operator))
        .parse(stream)?;

    Ok(op)
}

pub(super) fn any_operator(stream: &mut TokenStream) -> scan::Result<QualifiedOperator> {

    /*
        ( col_id '.' )* all_op
    */

    let (qn, op) = seq!(
        many!(
            seq!(col_id, Dot)
                .map(|(qn, _)| qn)
        )
            .optional()
            .map(Option::unwrap_or_default),
        all_op
    ).parse(stream)?;

    Ok(QualifiedOperator(qn, op))
}

/// Alias: `all_Op`.
///
/// Inlined: `MathOp`
fn all_op(stream: &mut TokenStream) -> scan::Result<Operator> {
    alt!(
        additive_op,
        multiplicative_op,
        exponentiation_op,
        boolean_op,
        user_defined_operator.map(UserDefined)
    ).parse(stream)
}

fn additive_op(stream: &mut TokenStream) -> scan::Result<Operator> {
    alt!(
        Plus.map(|_| Addition),
        Minus.map(|_| Subtraction)
    ).parse(stream)
}

fn multiplicative_op(stream: &mut TokenStream) -> scan::Result<Operator> {
    alt!(
        Mul.map(|_| Multiplication),
        Div.map(|_| Division),
        Percent.map(|_| Modulo),
    ).parse(stream)
}

fn exponentiation_op(stream: &mut TokenStream) -> scan::Result<Operator> {
    Circumflex.parse(stream)
        .map(|_| Exponentiation)
}

fn boolean_op(stream: &mut TokenStream) -> scan::Result<Operator> {
    alt!(
        Less.map(|_| Operator::Less),
        Equals.map(|_| Operator::Equals),
        Greater.map(|_| Operator::Greater),
        LessEquals.map(|_| Operator::LessEquals),
        GreaterEquals.map(|_| Operator::GreaterEquals),
        NotEquals.map(|_| Operator::NotEquals),
    ).parse(stream)
}

fn like_op(stream: &mut TokenStream) -> scan::Result<Operator> {
    alt!(
        Like.map(|_| Operator::Like),
        Ilike.map(|_| ILike)
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_user_defined_op() {

        let source = "operator(|/) <@>";
        let mut stream = TokenStream::from(source);

        let expected = QualifiedOperator(vec![], UserDefined("|/".into()));
        assert_eq!(Ok(expected), qual_op(&mut stream));

        let expected = QualifiedOperator(vec![], UserDefined("<@>".into()));
        assert_eq!(Ok(expected), qual_op(&mut stream));
    }

    #[test]
    fn test_qualified_op() {
        test_parser!(
            source = "operator(some_qn.*)",
            parser = qual_op,
            expected = QualifiedOperator(
                vec!["some_qn".into()],
                Multiplication
            )
        )
    }

    #[test]
    fn test_any_operator() {
        let source = "@@ != q_name.+";
        let mut stream = TokenStream::from(source);

        let expected = QualifiedOperator(
            vec![],
            UserDefined("@@".into())
        );
        assert_eq!(Ok(expected), any_operator(&mut stream));

        let expected = QualifiedOperator(
            vec![],
            Operator::NotEquals
        );
        assert_eq!(Ok(expected), any_operator(&mut stream));

        let expected = QualifiedOperator(
            vec!["q_name".into()],
            Addition
        );
        assert_eq!(Ok(expected), any_operator(&mut stream));
    }

    #[test]
    fn test_all_op() {
        let source = "~@ <>";
        let mut stream = TokenStream::from(source);

        assert_eq!(Ok(UserDefined("~@".into())), all_op(&mut stream));
        assert_eq!(Ok(Operator::NotEquals), all_op(&mut stream));
    }

    #[test]
    fn test_math_op() {

        let source = "+ - * / % ^ < > = <= >= != <>";
        let mut stream = TokenStream::from(source);

        assert_eq!(Ok(Addition), all_op(&mut stream));
        assert_eq!(Ok(Subtraction), all_op(&mut stream));
        assert_eq!(Ok(Multiplication), all_op(&mut stream));
        assert_eq!(Ok(Division), all_op(&mut stream));
        assert_eq!(Ok(Modulo), all_op(&mut stream));
        assert_eq!(Ok(Exponentiation), all_op(&mut stream));
        assert_eq!(Ok(Operator::Less), all_op(&mut stream));
        assert_eq!(Ok(Operator::Greater), all_op(&mut stream));
        assert_eq!(Ok(Operator::Equals), all_op(&mut stream));
        assert_eq!(Ok(Operator::LessEquals), all_op(&mut stream));
        assert_eq!(Ok(Operator::GreaterEquals), all_op(&mut stream));
        assert_eq!(Ok(Operator::NotEquals), all_op(&mut stream));
        assert_eq!(Ok(Operator::NotEquals), all_op(&mut stream));
    }

    #[test]
    fn test_subquery_op() {
        let source = "like ilike";
        let mut stream = TokenStream::from(source);

        assert_eq!(Ok(Operator::Like.into()), subquery_op(&mut stream));
        assert_eq!(Ok(ILike.into()), subquery_op(&mut stream));
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::many;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::user_defined_operator;
use pg_ast::Operator;
use pg_ast::Operator::Addition;
use pg_ast::Operator::Division;
use pg_ast::Operator::Exponentiation;
use pg_ast::Operator::ILike;
use pg_ast::Operator::Modulo;
use pg_ast::Operator::Multiplication;
use pg_ast::Operator::Subtraction;
use pg_ast::Operator::UserDefined;
use pg_ast::QualifiedOperator;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Ilike;
use pg_lexer::Keyword::Like;
use pg_lexer::Keyword::Operator as OperatorKw;
use pg_lexer::OperatorKind::Circumflex;
use pg_lexer::OperatorKind::Div;
use pg_lexer::OperatorKind::Dot;
use pg_lexer::OperatorKind::Equals;
use pg_lexer::OperatorKind::Greater;
use pg_lexer::OperatorKind::GreaterEquals;
use pg_lexer::OperatorKind::Less;
use pg_lexer::OperatorKind::LessEquals;
use pg_lexer::OperatorKind::Minus;
use pg_lexer::OperatorKind::Mul;
use pg_lexer::OperatorKind::NotEquals;
use pg_lexer::OperatorKind::Percent;
use pg_lexer::OperatorKind::Plus;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
