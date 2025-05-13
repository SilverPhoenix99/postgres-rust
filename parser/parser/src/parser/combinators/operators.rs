/// Alias: `subquery_Op`
pub(super) fn subquery_op() -> impl Combinator<Output = QualifiedOperator> {

    // Intentionally excludes NOT LIKE/NOT ILIKE, due to conflicts.
    // Those will have to be checked separately.

    match_first!(
        qual_all_op(),
        like_op().map(From::from)
    )
}

/// Alias: `qual_all_Op`
pub(super) fn qual_all_op() -> impl Combinator<Output = QualifiedOperator> {
    match_first!(
        all_op().map(From::from),
        explicit_op()
    )
}

/// Alias: `qual_Op`
pub(super) fn qual_op() -> impl Combinator<Output = QualifiedOperator> {
    match_first!(
        user_defined_operator()
            .map(|op| UserDefined(op).into()),
        explicit_op(),
    )
}

pub(super) fn explicit_op() -> impl Combinator<Output = QualifiedOperator> {

    /*
        OPERATOR '(' any_operator ')'
    */

    sequence!(
        OperatorKw.skip(),
        OpenParenthesis.skip(),
        any_operator(),
        CloseParenthesis.skip()
    )
        .map(|(_, _, op, _)| op)
}

pub(super) fn any_operator() -> impl Combinator<Output = QualifiedOperator> {

    /*
        ( col_id '.' )* all_op
    */

    many(enclosure! { col_id().and_left(Dot) })
        .optional()
        .map(Option::unwrap_or_default)
        .and_then(all_op(), QualifiedOperator)
}

/// Alias: `all_Op`.
///
/// Inlined: `MathOp`
fn all_op() -> impl Combinator<Output = Operator> {
    match_first!(
        additive_op(),
        multiplicative_op(),
        exponentiation_op(),
        boolean_op(),
        user_defined_operator().map(UserDefined)
    )
}

fn additive_op() -> impl Combinator<Output = Operator> {
    or(
        Plus.map(|_| Addition),
        Minus.map(|_| Subtraction)
    )
}

fn multiplicative_op() -> impl Combinator<Output = Operator> {
    match_first!(
        Mul.map(|_| Multiplication),
        Div.map(|_| Division),
        Percent.map(|_| Modulo),
    )
}

fn exponentiation_op() -> impl Combinator<Output = Operator> {
    Circumflex.map(|_| Exponentiation)
}

fn boolean_op() -> impl Combinator<Output = Operator> {
    match_first!(
        Less.map(|_| Operator::Less),
        Equals.map(|_| Operator::Equals),
        Greater.map(|_| Operator::Greater),
        LessEquals.map(|_| Operator::LessEquals),
        GreaterEquals.map(|_| Operator::GreaterEquals),
        NotEquals.map(|_| Operator::NotEquals),
    )
}

fn like_op() -> impl Combinator<Output = Operator> {
    or(
        Like.map(|_| Operator::Like),
        Ilike.map(|_| ILike)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_user_defined_op() {

        let source = "operator(|/) <@>";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = QualifiedOperator(vec![], UserDefined("|/".into()));
        assert_eq!(Ok(expected), qual_op().parse(&mut stream));

        let expected = QualifiedOperator(vec![], UserDefined("<@>".into()));
        assert_eq!(Ok(expected), qual_op().parse(&mut stream));
    }

    #[test]
    fn test_qualified_op() {
        let source = "operator(some_qn.*)";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let actual = qual_op().parse(&mut stream);
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
        assert_eq!(Ok(expected), any_operator().parse(&mut stream));

        let expected = QualifiedOperator(
            vec![],
            Operator::NotEquals
        );
        assert_eq!(Ok(expected), any_operator().parse(&mut stream));

        let expected = QualifiedOperator(
            vec!["q_name".into()],
            Addition
        );
        assert_eq!(Ok(expected), any_operator().parse(&mut stream));
    }

    #[test]
    fn test_all_op() {
        let source = "~@ <>";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(UserDefined("~@".into())), all_op().parse(&mut stream));
        assert_eq!(Ok(Operator::NotEquals), all_op().parse(&mut stream));
    }

    #[test]
    fn test_math_op() {

        let source = "+ - * / % ^ < > = <= >= != <>";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(Addition), all_op().parse(&mut stream));
        assert_eq!(Ok(Subtraction), all_op().parse(&mut stream));
        assert_eq!(Ok(Multiplication), all_op().parse(&mut stream));
        assert_eq!(Ok(Division), all_op().parse(&mut stream));
        assert_eq!(Ok(Modulo), all_op().parse(&mut stream));
        assert_eq!(Ok(Exponentiation), all_op().parse(&mut stream));
        assert_eq!(Ok(Operator::Less), all_op().parse(&mut stream));
        assert_eq!(Ok(Operator::Greater), all_op().parse(&mut stream));
        assert_eq!(Ok(Operator::Equals), all_op().parse(&mut stream));
        assert_eq!(Ok(Operator::LessEquals), all_op().parse(&mut stream));
        assert_eq!(Ok(Operator::GreaterEquals), all_op().parse(&mut stream));
        assert_eq!(Ok(Operator::NotEquals), all_op().parse(&mut stream));
        assert_eq!(Ok(Operator::NotEquals), all_op().parse(&mut stream));
    }

    #[test]
    fn test_subquery_op() {
        let source = "like ilike";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(Operator::Like.into()), subquery_op().parse(&mut stream));
        assert_eq!(Ok(ILike.into()), subquery_op().parse(&mut stream));
    }
}

use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::enclosure;
use crate::parser::combinators::foundation::many;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::user_defined_operator;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_parser_ast::Operator;
use postgres_parser_ast::Operator::Addition;
use postgres_parser_ast::Operator::Division;
use postgres_parser_ast::Operator::Exponentiation;
use postgres_parser_ast::Operator::ILike;
use postgres_parser_ast::Operator::Modulo;
use postgres_parser_ast::Operator::Multiplication;
use postgres_parser_ast::Operator::Subtraction;
use postgres_parser_ast::Operator::UserDefined;
use postgres_parser_ast::QualifiedOperator;
use postgres_parser_lexer::Keyword::Ilike;
use postgres_parser_lexer::Keyword::Like;
use postgres_parser_lexer::Keyword::Operator as OperatorKw;
use postgres_parser_lexer::OperatorKind::Circumflex;
use postgres_parser_lexer::OperatorKind::CloseParenthesis;
use postgres_parser_lexer::OperatorKind::Div;
use postgres_parser_lexer::OperatorKind::Dot;
use postgres_parser_lexer::OperatorKind::Equals;
use postgres_parser_lexer::OperatorKind::Greater;
use postgres_parser_lexer::OperatorKind::GreaterEquals;
use postgres_parser_lexer::OperatorKind::Less;
use postgres_parser_lexer::OperatorKind::LessEquals;
use postgres_parser_lexer::OperatorKind::Minus;
use postgres_parser_lexer::OperatorKind::Mul;
use postgres_parser_lexer::OperatorKind::NotEquals;
use postgres_parser_lexer::OperatorKind::OpenParenthesis;
use postgres_parser_lexer::OperatorKind::Percent;
use postgres_parser_lexer::OperatorKind::Plus;
