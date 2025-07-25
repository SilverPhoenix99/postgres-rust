pub(super) fn ambiguous_prefix_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {
    use crate::stream::TokenValue::Keyword as K;
    use crate::stream::TokenValue::Operator as Op;

    /*
          COLLATION FOR '(' a_expr ')'
        | CURRENT_SCHEMA
        | COALESCE '(' expr_list ')'
        | EXTRACT '(' extract_list ')'
    */

    match stream.peek2() {

        // TypeFuncName conflicts

        // `current_schema()` is valid syntax, so exclude that case.
        Ok((K(CurrentSchema), Op(OpenParenthesis))) => return no_match(stream),
        Ok((K(CurrentSchema), _)) => {
            stream.next(); // Consume the `current_schema` keyword.
            return Ok(ExprNode::CurrentSchema)
        },

        Ok((K(Collation), K(For))) => return collation_for(stream),

        // ColumnName conflicts

        Ok((K(Coalesce), Op(OpenParenthesis))) => return coalesce_expr(stream),

        Ok((K(Extract), Op(OpenParenthesis))) => return extract_func(stream),

        _ => {}
    }


    // If we reach here, it could be that there are 1 or fewer tokens left in the stream,
    // or there are more tokens, but they didn't match any of the above patterns.

    let _ = CurrentSchema.parse(stream)?;
    Ok(ExprNode::CurrentSchema)
}

fn collation_for(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        COLLATION FOR '(' a_expr ')'
    */

    stream.next(); // "collation" keyword
    stream.next(); // "for" keyword

    let expr = between_paren(a_expr)
        .parse(stream)
        .required()?;

    let expr = Box::new(expr);
    let expr = CollationFor(expr);
    Ok(expr)
}

fn coalesce_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        COALESCE '(' expr_list ')'
    */

    stream.next(); // "coalesce" keyword

    let args = expr_list_paren(stream).required()?;

    Ok(CoalesceExpr(args))
}

fn extract_func(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        EXTRACT '(' extract_list ')'
    */

    stream.next(); // "extract" keyword

    let expr = between_paren(extract_args)
        .parse(stream)
        .required()?;

    Ok(expr.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scan::Error::NoMatch;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{
        ExprNode::StringConst,
        ExtractArg,
        ExtractFunc,
    };
    use pg_basics::Location;
    use test_case::test_case;

    #[test_case("current_schema 1", ExprNode::CurrentSchema)]
    #[test_case("current_schema", ExprNode::CurrentSchema)]
    #[test_case("collation for ('foo')",
        CollationFor(
            Box::new(StringConst("foo".into()))
        )
    )]
    #[test_case("coalesce ('foo', 'bar')",
        CoalesceExpr(vec![
            StringConst("foo".into()),
            StringConst("bar".into())
        ])
    )]
    #[test_case("extract (year from 'foo')",
        ExtractFunc::new(
            ExtractArg::Year,
            StringConst("foo".into())
        ).into()
    )]
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
}

use super::extract_list::extract_args;
use crate::combinators::expr::a_expr;
use crate::combinators::expr_list_paren;
use crate::combinators::foundation::between_paren;
use crate::combinators::foundation::Combinator;
use crate::no_match;
use crate::result::Required;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
use pg_ast::ExprNode::CoalesceExpr;
use pg_ast::ExprNode::CollationFor;
use pg_lexer::Keyword::Coalesce;
use pg_lexer::Keyword::Collation;
use pg_lexer::Keyword::CurrentSchema;
use pg_lexer::Keyword::Extract;
use pg_lexer::Keyword::For;
use pg_lexer::OperatorKind::OpenParenthesis;
