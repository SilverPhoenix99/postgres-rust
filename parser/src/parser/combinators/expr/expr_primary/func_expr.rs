pub(super) fn func_expr() -> impl Combinator<Output = ExprNode> {

    match_first! {
        Kw::CurrentRole.map(|_| CurrentRole),
        Kw::CurrentUser.map(|_| CurrentUser),
        Kw::SessionUser.map(|_| SessionUser),
        Kw::SystemUser.map(|_| SystemUser),
        Kw::User.map(|_| User),
        Kw::CurrentCatalog.map(|_| CurrentCatalog),
        Kw::CurrentDate.map(|_| CurrentDate),
        Kw::CurrentTime
            .and_right(opt_precision())
            .map(|precision| CurrentTime { precision }),
        Kw::CurrentTimestamp
            .and_right(opt_precision())
            .map(|precision| CurrentTimestamp { precision }),
        Kw::Localtime
            .and_right(opt_precision())
            .map(|precision| LocalTime { precision }),
        Kw::Localtimestamp
            .and_right(opt_precision())
            .map(|precision| LocalTimestamp { precision }),
        case_expr().map(From::from),
        cast_expr().map(From::from),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
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
        test_parser!(source, func_expr(), expected)
    }

    #[test_case("case when 1 then 2 end")]
    #[test_case("cast ('1' as int)")]
    fn test_func_expr_calls(source: &str) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = func_expr().parse(&mut stream);

        assert_matches!(actual, Ok(_),
            r"expected Ok(Some(_)) for {source:?} but actually got {actual:?}"
        )
    }
}

use crate::lexer::Keyword as Kw;
use crate::parser::ast_node::ExprNode;
use crate::parser::ast_node::ExprNode::CurrentCatalog;
use crate::parser::ast_node::ExprNode::CurrentDate;
use crate::parser::ast_node::ExprNode::CurrentRole;
use crate::parser::ast_node::ExprNode::CurrentTime;
use crate::parser::ast_node::ExprNode::CurrentTimestamp;
use crate::parser::ast_node::ExprNode::CurrentUser;
use crate::parser::ast_node::ExprNode::LocalTime;
use crate::parser::ast_node::ExprNode::LocalTimestamp;
use crate::parser::ast_node::ExprNode::SessionUser;
use crate::parser::ast_node::ExprNode::SystemUser;
use crate::parser::ast_node::ExprNode::User;
use crate::parser::combinators::expr::expr_primary::case_expr;
use crate::parser::combinators::expr::expr_primary::cast_expr;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::opt_precision;
