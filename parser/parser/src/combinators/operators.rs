/// Alias: `subquery_Op`
pub(super) fn subquery_op(stream: &mut TokenStream) -> scan::Result<QualifiedOperator> {

    // Intentionally excludes NOT LIKE/NOT ILIKE, due to conflicts.
    // Those will have to be checked separately.

    choice!(parsed stream =>
        qual_all_op,
        like_op.map(From::from)
    )
}

/// Alias: `qual_all_Op`
pub(super) fn qual_all_op(stream: &mut TokenStream) -> scan::Result<QualifiedOperator> {
    choice!(parsed stream =>
        all_op.map(From::from),
        explicit_op
    )
}

/// Alias: `qual_Op`
pub(super) fn qual_op(stream: &mut TokenStream) -> scan::Result<QualifiedOperator> {
    choice!(parsed stream =>
        user_defined_operator
            .map(|op| UserDefined(op).into()),
        explicit_op,
    )
}

pub(super) fn explicit_op(stream: &mut TokenStream) -> scan::Result<QualifiedOperator> {

    /*
        OPERATOR '(' any_operator ')'
    */

    let (_, op) = seq!(=>
        OperatorKw.parse(stream),
        between!(paren : stream => any_operator.parse(stream))
    )?;

    Ok(op)
}

pub(super) fn any_operator(stream: &mut TokenStream) -> scan::Result<QualifiedOperator> {

    /*
        ( col_id '.' )* all_op
    */

    let (qn, op) = seq!(=>
        {
            many!(=>
                seq!(stream => col_id, Dot)
                    .map(|(qn, _)| qn)
            )
            .optional()
            .map(Option::unwrap_or_default)
        },
        {
            all_op(stream)
        }
    )?;

    Ok(QualifiedOperator(qn, op))
}

/// Alias: `all_Op`.
///
/// Inlined: `MathOp`
fn all_op(stream: &mut TokenStream) -> scan::Result<Operator> {
    choice!(parsed stream =>
        additive_op,
        multiplicative_op,
        exponentiation_op,
        boolean_op,
        user_defined_operator.map(UserDefined)
    )
}

fn additive_op(stream: &mut TokenStream) -> scan::Result<Operator> {
    choice!(parsed stream =>
        Plus.map(|_| Addition),
        Minus.map(|_| Subtraction)
    )
}

fn multiplicative_op(stream: &mut TokenStream) -> scan::Result<Operator> {
    choice!(parsed stream =>
        Mul.map(|_| Multiplication),
        Div.map(|_| Division),
        Percent.map(|_| Modulo),
    )
}

fn exponentiation_op(stream: &mut TokenStream) -> scan::Result<Operator> {
    Circumflex
        .parse(stream)
        .map(|_| Exponentiation)
}

fn boolean_op(stream: &mut TokenStream) -> scan::Result<Operator> {
    choice!(parsed stream =>
        Less.map(|_| Operator::Less),
        Equals.map(|_| Operator::Equals),
        Greater.map(|_| Operator::Greater),
        LessEquals.map(|_| Operator::LessEquals),
        GreaterEquals.map(|_| Operator::GreaterEquals),
        NotEquals.map(|_| Operator::NotEquals),
    )
}

fn like_op(stream: &mut TokenStream) -> scan::Result<Operator> {
    choice!(parsed stream =>
        Like.map(|_| Operator::Like),
        Ilike.map(|_| ILike)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_user_defined_op() {

        let source = "operator(|/) <@>";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = QualifiedOperator(vec![], UserDefined("|/".into()));
        assert_eq!(Ok(expected), qual_op(&mut stream));

        let expected = QualifiedOperator(vec![], UserDefined("<@>".into()));
        assert_eq!(Ok(expected), qual_op(&mut stream));
    }

    #[test]
    fn test_qualified_op() {
        let source = "operator(some_qn.*)";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let actual = qual_op(&mut stream);
        let expected = QualifiedOperator(
            vec!["some_qn".into()],
            Multiplication
        );
        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_any_operator() {
        let source = "@@ != q_name.+";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

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
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(UserDefined("~@".into())), all_op(&mut stream));
        assert_eq!(Ok(Operator::NotEquals), all_op(&mut stream));
    }

    #[test]
    fn test_math_op() {

        let source = "+ - * / % ^ < > = <= >= != <>";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

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
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(Operator::Like.into()), subquery_op(&mut stream));
        assert_eq!(Ok(ILike.into()), subquery_op(&mut stream));
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::between;
use crate::combinators::foundation::choice;
use crate::combinators::foundation::many;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::user_defined_operator;
use crate::combinators::foundation::Combinator;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
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
