pub(super) fn ambiguous_prefix_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {
    use crate::stream::TokenValue::Keyword as K;
    use crate::stream::TokenValue::Operator as Op;

    /*
          COLLATION FOR '(' a_expr ')'
        | CURRENT_SCHEMA
        | COALESCE '(' expr_list ')'
        | EXTRACT '(' extract_list ')'
        | GREATEST '(' expr_list ')'
        | LEAST '(' expr_list ')'
    */

    match stream.peek2() {

        // TypeFuncName conflicts

        // `current_schema()` is valid syntax, so exclude that case.
        Ok((K(Kw::CurrentSchema), Op(OpenParenthesis))) => return no_match(stream),
        Ok((K(Kw::CurrentSchema), _)) => {
            stream.skip(1); // Consume the `current_schema` keyword.
            return Ok(CurrentSchema)
        },

        Ok((K(Collation), K(For))) => return collation_for(stream),

        // ColumnName conflicts

        Ok((K(Coalesce), Op(OpenParenthesis))) => return coalesce_expr(stream),

        Ok((K(Extract), Op(OpenParenthesis))) => return extract_func(stream),

        Ok((K(Greatest), Op(OpenParenthesis))) => return greatest_func(stream),

        Ok((K(Least), Op(OpenParenthesis))) => return least_func(stream),

        Ok((K(MergeAction), Op(OpenParenthesis))) => {
            skip_prefix(2, CloseParenthesis)
                .parse(stream)?;
            return Ok(MergeSupportFunc)
        },

        _ => {}
    }


    // If we reach here, it could be that there are 1 or fewer tokens left in the stream,
    // or there are more tokens, but they didn't match any of the above patterns.

    let _ = Kw::CurrentSchema.parse(stream)?;
    Ok(CurrentSchema)
}

fn collation_for(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        COLLATION FOR '(' a_expr ')'
    */

    let expr = skip_prefix(2, between_paren(a_expr))
        .parse(stream)?;

    let expr = Box::new(expr);
    Ok(CollationFor(expr))
}

fn coalesce_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        COALESCE '(' expr_list ')'
    */

    let args = skip_prefix(1, expr_list_paren)
        .parse(stream)?;

    Ok(CoalesceExpr(args))
}

fn extract_func(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        EXTRACT '(' extract_list ')'
    */

    let expr = skip_prefix(1, between_paren(extract_args))
        .parse(stream)?;

    Ok(expr.into())
}

fn greatest_func(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        GREATEST '(' expr_list ')'
    */

    let args = skip_prefix(1, expr_list_paren)
        .parse(stream)?;

    Ok(GreatestFunc(args))
}

fn least_func(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        LEAST '(' expr_list ')'
    */

    let args = skip_prefix(1, expr_list_paren)
        .parse(stream)?;

    Ok(LeastFunc(args))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scan::Error::NoMatch;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{
        ExprNode::{IntegerConst, StringConst},
        ExtractArg,
        ExtractFunc,
    };
    use pg_basics::Location;
    use test_case::test_case;

    #[test_case("current_schema 1", CurrentSchema)]
    #[test_case("current_schema", CurrentSchema)]
    #[test_case("collation for ('foo')",
        CollationFor(
            Box::new(StringConst("foo".into()))
        )
    )]
    #[test_case("coalesce('foo', 'bar')",
        CoalesceExpr(vec![
            StringConst("foo".into()),
            StringConst("bar".into())
        ])
    )]
    #[test_case("extract(year from 'foo')",
        ExtractFunc::new(
            ExtractArg::Year,
            StringConst("foo".into())
        ).into()
    )]
    #[test_case("greatest(1, 2)",
        GreatestFunc(vec![
            IntegerConst(1),
            IntegerConst(2)
        ])
    )]
    #[test_case("least(1, 2)",
        LeastFunc(vec![
            IntegerConst(1),
            IntegerConst(2)
        ])
    )]
    #[test_case("merge_action()", MergeSupportFunc)]
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
use crate::combinators::foundation::skip_prefix;
use crate::combinators::foundation::Combinator;
use crate::no_match;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
use pg_ast::ExprNode::CoalesceExpr;
use pg_ast::ExprNode::CollationFor;
use pg_ast::ExprNode::CurrentSchema;
use pg_ast::ExprNode::GreatestFunc;
use pg_ast::ExprNode::LeastFunc;
use pg_ast::ExprNode::MergeSupportFunc;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Coalesce;
use pg_lexer::Keyword::Collation;
use pg_lexer::Keyword::Extract;
use pg_lexer::Keyword::For;
use pg_lexer::Keyword::Greatest;
use pg_lexer::Keyword::Least;
use pg_lexer::Keyword::MergeAction;
use pg_lexer::OperatorKind::CloseParenthesis;
use pg_lexer::OperatorKind::OpenParenthesis;
