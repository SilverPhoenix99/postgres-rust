mod filter_clause;
mod over_clause;
mod within_group_clause;

#[allow(unused_imports)]
pub(super) use {
    filter_clause::*,
    over_clause::*,
    within_group_clause::*,
};

pub(super) fn func_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    // Broken down into smaller combinators, due to large Rust type names.
    or((
        // Must be first, to avoid conflicts with ambiguous prefix_expr.
        ambiguous_prefix_expr,

        func_expr_1,
        func_expr_2,
        func_expr_3,
    )).parse(stream)
}

fn func_expr_1(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    or((
        (Kw::CurrentTime, precision.optional())
            .map(|(_, precision)| CurrentTime { precision }),
        (Kw::CurrentTimestamp, precision.optional())
            .map(|(_, precision)| CurrentTimestamp { precision }),
        (Kw::Localtime, precision.optional())
            .map(|(_, precision)| LocalTime { precision }),
        (Kw::Localtimestamp, precision.optional())
            .map(|(_, precision)| LocalTimestamp { precision }),
    )).parse(stream)
}

fn func_expr_2(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    or((
        Kw::CurrentRole.map(|_| CurrentRole),
        Kw::CurrentUser.map(|_| CurrentUser),
        Kw::SessionUser.map(|_| SessionUser),
        Kw::SystemUser.map(|_| SystemUser),
        Kw::User.map(|_| User),
        Kw::CurrentCatalog.map(|_| CurrentCatalog),
        Kw::CurrentDate.map(|_| CurrentDate),
    )).parse(stream)
}

fn func_expr_3(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    or((
        case_expr.map(From::from),
        cast_expr.map(From::from),
    )).parse(stream)
}

fn ambiguous_prefix_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
          COLLATION FOR '(' a_expr ')'
        | CURRENT_SCHEMA
    */

    match stream.peek2() {

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
    use crate::stream::TokenStream;
    use crate::tests::test_parser;
    use crate::tests::DEFAULT_CONFIG;
    use pg_ast::ExprNode;
    use pg_ast::ExprNode::StringConst;
    use pg_basics::Location;
    use test_case::test_case;

    #[test_case("CURRENT_role", ExprNode::CurrentRole)]
    #[test_case("current_USER", ExprNode::CurrentUser)]
    #[test_case("SESSION_USER", ExprNode::SessionUser)]
    #[test_case("system_user", ExprNode::SystemUser)]
    #[test_case("uSeR", ExprNode::User)]
    #[test_case("current_catalog", ExprNode::CurrentCatalog)]
    #[test_case("current_schema", ExprNode::CurrentSchema)]
    #[test_case("current_date", ExprNode::CurrentDate)]
    #[test_case("current_time", ExprNode::CurrentTime { precision: None })]
    #[test_case("current_time(3)", ExprNode::CurrentTime { precision: Some(3) })]
    #[test_case("current_timestamp", ExprNode::CurrentTimestamp { precision: None })]
    #[test_case("current_timestamp(7)", ExprNode::CurrentTimestamp { precision: Some(7) })]
    #[test_case("localtime", ExprNode::LocalTime { precision: None })]
    #[test_case("localtime(6)", ExprNode::LocalTime { precision: Some(6) })]
    #[test_case("localtimestamp", ExprNode::LocalTimestamp { precision: None })]
    #[test_case("localtimestamp(4)", ExprNode::LocalTimestamp { precision: Some(4) })]
    #[test_case("collation for (5)", CollationFor(Box::new(ExprNode::IntegerConst(5))))]
    fn test_func_expr(source: &str, expected: ExprNode) {
        test_parser!(source, func_expr, expected)
    }

    #[test_case("case when 1 then 2 end")]
    #[test_case("cast ('1' as int)")]
    fn test_func_expr_calls(source: &str) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = func_expr(&mut stream);

        assert_matches!(actual, Ok(_),
            r"expected Ok(Some(_)) for {source:?} but actually got {actual:?}"
        )
    }

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
use crate::combinators::expr::expr_primary::case_expr;
use crate::combinators::expr::expr_primary::cast_expr;
use crate::combinators::foundation::between_paren;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::precision;
use crate::scan;
use crate::scan::Error::NoMatch;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword;
use crate::stream::TokenValue::Operator;
use pg_ast::ExprNode;
use pg_ast::ExprNode::CollationFor;
use pg_ast::ExprNode::CurrentCatalog;
use pg_ast::ExprNode::CurrentDate;
use pg_ast::ExprNode::CurrentRole;
use pg_ast::ExprNode::CurrentTime;
use pg_ast::ExprNode::CurrentTimestamp;
use pg_ast::ExprNode::CurrentUser;
use pg_ast::ExprNode::LocalTime;
use pg_ast::ExprNode::LocalTimestamp;
use pg_ast::ExprNode::SessionUser;
use pg_ast::ExprNode::SystemUser;
use pg_ast::ExprNode::User;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Collation;
use pg_lexer::Keyword::CurrentSchema;
use pg_lexer::Keyword::For;
use pg_lexer::OperatorKind::OpenParenthesis;
