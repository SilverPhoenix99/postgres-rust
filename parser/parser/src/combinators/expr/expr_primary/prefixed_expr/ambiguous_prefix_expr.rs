pub(super) fn ambiguous_prefix_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        Catches ambiguous expressions.
        Called before matching other prefixed primary expressions.

          DOUBLE PRECISION SCONST       => AexprConst
        | COLLATION FOR '(' a_expr ')'  => func_expr
        | CURRENT_SCHEMA                => func_expr

        TODO: Move these matches to `expr_const` or `func_expr` combinators,
              and call them here.
    */

    match stream.peek2() {

        // Unreserved conflicts
        Ok((Keyword(Double), Keyword(Precision))) => {
            // Due to the condition, this will never return `NoMatch`.
            return double_precision(stream)
        },

        // TypeFuncName conflicts
        Ok((Keyword(Collation), Keyword(For))) => {
            return collation_for(stream)
        },
        Ok((Keyword(CurrentSchema), Operator(OpenParenthesis))) => {
            // `current_schema()` is valid syntax, so exclude that case.
            return Err(NoMatch(stream.current_location()))
        },
        Ok((Keyword(CurrentSchema), _)) => {
            stream.next(); // Consume the `current_schema` keyword.
            return Ok(ExprNode::CurrentSchema)
        },

        _ => {}
    }

    // If we reach here, it could be that there are 1 or fewer tokens left in the stream,
    // or there are more tokens, but they didn't match any of the above patterns.

    let _ = CurrentSchema.parse(stream)?;
    Ok(ExprNode::CurrentSchema)
}

fn double_precision(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        DOUBLE PRECISION SCONST
    */

    let (.., value) = (Double, Precision, string).parse(stream)?;
    let expr = TypecastExpr::new(StringConst(value), Float8);
    Ok(expr.into())
}

fn collation_for(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        COLLATION FOR '(' a_expr ')'
    */

    let (.., expr) = (Collation, For, between_paren(a_expr))
        .parse(stream)?;

    let expr = Box::new(expr);
    let expr = CollationFor(expr);
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    use pg_basics::Location;

    #[test_case("double precision '1.23'",
        TypecastExpr::new(
            StringConst("1.23".into()),
            Float8
        ).into()
    )]
    #[test_case("collation for ('foo')",
        CollationFor(
            Box::new(StringConst("foo".into()))
        )
    )]
    #[test_case("current_schema 1", ExprNode::CurrentSchema)]
    #[test_case("current_schema", ExprNode::CurrentSchema)]
    fn test_ambiguous_prefix_expr(source: &str, expected: ExprNode) {
        test_parser!(source, ambiguous_prefix_expr, expected)
    }

    #[test]
    fn test_ambiguous_prefix_expr_no_match() {
        test_parser!(
            source = "current_schema(",
            parser = ambiguous_prefix_expr,
            expected = Err(NoMatch(Location::new(0..14, 1, 1)))
        )
    }

    #[test]
    fn test_double_precision() {
        test_parser!(
            source = "double precision '1.23'",
            parser = double_precision,
            expected = ExprNode::from(
                TypecastExpr::new(
                    StringConst("1.23".into()),
                    Float8
                )
            )
        )
    }

    #[test]
    fn test_collation_for() {
        test_parser!(
            source = "collation for ('foo')",
            parser = collation_for,
            expected = CollationFor(
                Box::new(StringConst("foo".into()))
            )
        )
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::between_paren;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword;
use crate::stream::TokenValue::Operator;
use pg_ast::ExprNode;
use pg_ast::ExprNode::CollationFor;
use pg_ast::ExprNode::StringConst;
use pg_ast::TypeName::Float8;
use pg_ast::TypecastExpr;
use pg_lexer::Keyword::Collation;
use pg_lexer::Keyword::CurrentSchema;
use pg_lexer::Keyword::Double;
use pg_lexer::Keyword::For;
use pg_lexer::Keyword::Precision;
use pg_lexer::OperatorKind::OpenParenthesis;
use crate::scan::Error::NoMatch;
