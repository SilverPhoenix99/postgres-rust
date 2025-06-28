mod filter_clause;
mod over_clause;
mod within_group_clause;

pub(super) fn func_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    choice!(stream =>
        Kw::CurrentRole.parse(stream).map(|_| CurrentRole),
        Kw::CurrentUser.parse(stream).map(|_| CurrentUser),
        Kw::SessionUser.parse(stream).map(|_| SessionUser),
        Kw::SystemUser.parse(stream).map(|_| SystemUser),
        Kw::User.parse(stream).map(|_| User),
        Kw::CurrentCatalog.parse(stream).map(|_| CurrentCatalog),
        Kw::CurrentDate.parse(stream).map(|_| CurrentDate),
        seq!(stream => Kw::CurrentTime, opt_precision)
            .map(|(_, precision)| CurrentTime { precision }),
        seq!(stream => Kw::CurrentTimestamp, opt_precision)
            .map(|(_, precision)| CurrentTimestamp { precision }),
        seq!(stream => Kw::Localtime, opt_precision)
            .map(|(_, precision)| LocalTime { precision }),
        seq!(stream => Kw::Localtimestamp, opt_precision)
            .map(|(_, precision)| LocalTimestamp { precision }),
        case_expr(stream).map(From::from),
        cast_expr(stream).map(From::from),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::test_parser;
    use crate::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("CURRENT_role", ExprNode::CurrentRole)]
    #[test_case("current_USER", ExprNode::CurrentUser)]
    #[test_case("SESSION_USER", ExprNode::SessionUser)]
    #[test_case("system_user", ExprNode::SystemUser)]
    #[test_case("uSeR", ExprNode::User)]
    #[test_case("current_catalog", ExprNode::CurrentCatalog)]
    #[test_case("current_date", ExprNode::CurrentDate)]
    #[test_case("current_time", ExprNode::CurrentTime { precision: None })]
    #[test_case("current_time(3)", ExprNode::CurrentTime { precision: Some(3) })]
    #[test_case("current_timestamp", ExprNode::CurrentTimestamp { precision: None })]
    #[test_case("current_timestamp(7)", ExprNode::CurrentTimestamp { precision: Some(7) })]
    #[test_case("localtime", ExprNode::LocalTime { precision: None })]
    #[test_case("localtime(6)", ExprNode::LocalTime { precision: Some(6) })]
    #[test_case("localtimestamp", ExprNode::LocalTimestamp { precision: None })]
    #[test_case("localtimestamp(4)", ExprNode::LocalTimestamp { precision: Some(4) })]
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
}

use crate::combinators::expr::expr_primary::case_expr;
use crate::combinators::expr::expr_primary::cast_expr;
use crate::combinators::foundation::choice;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::opt_precision;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
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
