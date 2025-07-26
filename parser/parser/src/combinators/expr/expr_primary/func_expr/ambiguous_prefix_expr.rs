/// Alias: `func_expr_common_subexpr`
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
        | NORMALIZE '(' a_expr ( ',' unicode_normal_form )? ')'
        | NULLIF '(' a_expr ',' a_expr ')'
        | POSITION '(' b_expr IN b_expr ')'
        | TREAT '(' a_expr AS Typename ')'
        | TRIM '(' trim_args ')'
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

        Ok((K(Normalize), Op(OpenParenthesis))) => return normalize_func(stream),

        Ok((K(Nullif), Op(OpenParenthesis))) => return nullif(stream),

        Ok((K(Position), Op(OpenParenthesis))) => return position(stream),

        Ok((K(Kw::Treat), Op(OpenParenthesis))) => return treat(stream),

        Ok((K(Trim), Op(OpenParenthesis))) => return trim(stream),

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

fn normalize_func(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        NORMALIZE '(' a_expr ( ',' unicode_normal_form )? ')'
    */

    let (expr, normal_form) = skip_prefix(1,
        between_paren((
            a_expr,
            (Comma, unicode_normal_form)
                .map(|(_, normal_form)| normal_form)
                .optional()
        ))
    ).parse(stream)?;

    let expr = NormalizeFunc::new(expr, normal_form);
    Ok(expr.into())
}

fn nullif(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        NULLIF '(' a_expr ',' a_expr ')'
    */

    let (left, _, right) = skip_prefix(1,
        between_paren((a_expr, Comma, a_expr))
    ).parse(stream)?;

    let operands = Box::new((left, right));
    Ok(NullIf(operands))
}

/// Inlined: `position_list`
fn position(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        POSITION '(' b_expr IN b_expr ')'

        A "plain syntax" option is deliberately not offered
        for position(), because the reversal of the arguments
        creates too much risk of confusion.
    */

    let (needle, _, haystack) = skip_prefix(1,
        between_paren((b_expr, In, b_expr))
    ).parse(stream)?;

    let expr = PositionFunc::new(needle, haystack);
    Ok(expr.into())
}

fn treat(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        TREAT '(' a_expr AS Typename ')'

        Converts the expression of a particular type to a target type,
        which is defined to be a subtype of the original expression.
        In SQL99, this is intended for use with structured UDTs,
        but let's make this a generally useful form allowing stronger
        coercions than are handled by implicit casting.
    */

    let (expr, _, typename) = skip_prefix(1,
        between_paren((a_expr, As, typename))
    ).parse(stream)?;

    let cast = TypecastExpr::new(expr, typename);
    let expr = Treat(Box::new(cast));
    Ok(expr)
}

fn trim(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        TRIM '(' trim_args ')'
    */

    let expr = skip_prefix(1, between_paren(trim_args))
        .parse(stream)?;

    Ok(expr.into())
}

fn trim_args(stream: &mut TokenStream) -> scan::Result<TrimFunc> {
    use TrimSide::*;

    /*
          LEADING   trim_list
        | TRAILING  trim_list
        | ( BOTH )? trim_list
    */

    let (trim_side, args) = or((

        (Kw::Leading.map(|_| Leading), trim_list),

        (Kw::Trailing.map(|_| Trailing), trim_list),

        (
            Kw::Both.map(|_| Both)
                .optional()
                .map(Option::unwrap_or_default),
            trim_list
        )

    )).parse(stream)?;

    let expr = TrimFunc::new(trim_side, args);
    Ok(expr)
}

fn trim_list(stream: &mut TokenStream) -> scan::Result<Vec<ExprNode>> {

    /*
          FROM expr_list
        | a_expr ( ( FROM | ',') expr_list )?
    */

    or((
        (FromKw, expr_list).map(|(_, args)| args),
        (a_expr,
            (
                or((
                    Comma.map(|_| true),  // Prepend
                    FromKw.map(|_| false) // Append
                )),
                expr_list
            ).optional()
                .map(|opt|
                    opt.unwrap_or_else(|| (false, Vec::with_capacity(1)))
                )
        )
            .map(|(arg, (prepend, mut args))| {
                if prepend {
                    args.insert(0, arg);
                }
                else {
                    args.push(arg);
                }
                args
            })
    )).parse(stream)
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
        TypeName,
        UnicodeNormalForm::CanonicalComposition,
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
    #[test_case("normalize('foo')",
        NormalizeFunc::new(
            StringConst("foo".into()),
            None
        ).into()
    )]
    #[test_case("normalize('foo', nfc)",
        NormalizeFunc::new(
            StringConst("foo".into()),
            Some(CanonicalComposition)
        ).into()
    )]
    #[test_case("nullif(null, 'foo')",
        NullIf(Box::new((
            ExprNode::NullConst,
            StringConst("foo".into())
        )))
    )]
    #[test_case("position('f' in 'foo')",
        PositionFunc::new(
            StringConst("f".into()),
            StringConst("foo".into())
        ).into()
    )]
    #[test_case("treat(123 as int)",
        Treat(Box::new(TypecastExpr::new(
            IntegerConst(123),
            TypeName::Int4
        )))
    )]
    #[test_case("trim('foo' from 'bar')",
        TrimFunc::new(
            TrimSide::Both,
            vec![StringConst("bar".into()), StringConst("foo".into())]
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

    #[test_case("leading from 'foo'", TrimSide::Leading, vec![StringConst("foo".into())])]
    #[test_case("trailing 'foo' from 'bar'", TrimSide::Trailing,
        vec![StringConst("bar".into()), StringConst("foo".into())]
    )]
    #[test_case("both 'foo'", TrimSide::Both, vec![StringConst("foo".into())])]
    #[test_case("'foo', 'bar'", TrimSide::Both,
        vec![StringConst("foo".into()), StringConst("bar".into())]
    )]
    fn test_trim_args(source: &str, trim_side: TrimSide, args: Vec<ExprNode>) {
        let expected = TrimFunc::new(trim_side, args);
        test_parser!(source, trim_args, expected)
    }

    #[test_case("from 'foo'", vec![StringConst("foo".into())])]
    #[test_case("from 'foo', 'bar'", vec![StringConst("foo".into()), StringConst("bar".into())])]
    #[test_case("'foo'", vec![StringConst("foo".into())])]
    #[test_case("'foo' from 'bar'", vec![StringConst("bar".into()), StringConst("foo".into())])]
    #[test_case("'foo' from 'bar', 'baz'", vec![
        StringConst("bar".into()),
        StringConst("baz".into()),
        StringConst("foo".into())
    ])]
    #[test_case("'foo', 'bar'", vec![StringConst("foo".into()), StringConst("bar".into())])]
    #[test_case("'foo', 'bar', 'baz'", vec![
        StringConst("foo".into()),
        StringConst("bar".into()),
        StringConst("baz".into()),
    ])]
    fn test_trim_list(source: &str, expected: Vec<ExprNode>) {
        test_parser!(source, trim_list, expected)
    }
}

use super::extract_list::extract_args;
use crate::combinators::expr::a_expr;
use crate::combinators::expr::b_expr;
use crate::combinators::expr::unicode_normal_form;
use crate::combinators::expr_list::expr_list;
use crate::combinators::expr_list_paren;
use crate::combinators::foundation::between_paren;
use crate::combinators::foundation::or;
use crate::combinators::foundation::skip_prefix;
use crate::combinators::foundation::Combinator;
use crate::combinators::typename;
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
use pg_ast::ExprNode::NullIf;
use pg_ast::ExprNode::Treat;
use pg_ast::NormalizeFunc;
use pg_ast::PositionFunc;
use pg_ast::TrimFunc;
use pg_ast::TrimSide;
use pg_ast::TypecastExpr;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::As;
use pg_lexer::Keyword::Coalesce;
use pg_lexer::Keyword::Collation;
use pg_lexer::Keyword::Extract;
use pg_lexer::Keyword::For;
use pg_lexer::Keyword::FromKw;
use pg_lexer::Keyword::Greatest;
use pg_lexer::Keyword::In;
use pg_lexer::Keyword::Least;
use pg_lexer::Keyword::MergeAction;
use pg_lexer::Keyword::Normalize;
use pg_lexer::Keyword::Nullif;
use pg_lexer::Keyword::Position;
use pg_lexer::Keyword::Trim;
use pg_lexer::OperatorKind::CloseParenthesis;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::OpenParenthesis;
