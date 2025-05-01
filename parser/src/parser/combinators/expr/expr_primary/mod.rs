mod case_expr;

/// Alias: `c_expr`
pub(super) fn expr_primary() -> impl Combinator<Output = ExprNode> {
    match_first! {
        param_expr().map(From::from),
        case_expr().map(From::from),
        expr_const(),
        CurrentRole.map(|_| ExprNode::CurrentRole),
        CurrentUser.map(|_| ExprNode::CurrentUser),
        SessionUser.map(|_| ExprNode::SessionUser),
        SystemUser.map(|_| ExprNode::SystemUser),
        User.map(|_| ExprNode::User),
        CurrentCatalog.map(|_| ExprNode::CurrentCatalog),
        CurrentDate.map(|_| ExprNode::CurrentDate),
        CurrentTime
            .and_right(opt_precision())
            .map(|precision| ExprNode::CurrentTime { precision }),
        CurrentTimestamp
            .and_right(opt_precision())
            .map(|precision| ExprNode::CurrentTimestamp { precision }),
        Localtime
            .and_right(opt_precision())
            .map(|precision| ExprNode::LocalTime { precision }),
        Localtimestamp
            .and_right(opt_precision())
            .map(|precision| ExprNode::LocalTimestamp { precision }),
    }
}

fn param_expr() -> impl Combinator<Output = ExprNode> {

    /*
        PARAM ( indirection )?

        E.g: $1.foo[0].*
    */

    sequence!(
        param(),
        located(indirection()).optional()
    )
        .map_result(|res| {
            let (index, indirection) = res?;
            let param = ParamRef { index };
            let expr = match indirection {
                None => param,
                Some(indirection) => {
                    let indirection = check_indirection(indirection)?;
                    IndirectionExpr::new(param, indirection).into()
                },
            };
            Ok(expr)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::Indirection::FullSlice;
    use crate::parser::tests::test_parser;
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
    fn test_expr_primary(source: &str, expected: ExprNode) {
        test_parser!(source, expr_primary(), expected)
    }

    #[test]
    fn test_param_expr() {
        test_parser!(
            source = "$5[:]",
            parser = param_expr(),
            expected = IndirectionExpr::new(
                ParamRef { index: 5 },
                vec![FullSlice]
            ).into()
        )
    }
}

use self::case_expr::case_expr;
use crate::lexer::Keyword::CurrentCatalog;
use crate::lexer::Keyword::CurrentDate;
use crate::lexer::Keyword::CurrentRole;
use crate::lexer::Keyword::CurrentTime;
use crate::lexer::Keyword::CurrentTimestamp;
use crate::lexer::Keyword::CurrentUser;
use crate::lexer::Keyword::Localtime;
use crate::lexer::Keyword::Localtimestamp;
use crate::lexer::Keyword::SessionUser;
use crate::lexer::Keyword::SystemUser;
use crate::lexer::Keyword::User;
use crate::parser::ast_node::ExprNode;
use crate::parser::ast_node::ExprNode::ParamRef;
use crate::parser::ast_node::IndirectionExpr;
use crate::parser::combinators::expr::check_indirection;
use crate::parser::combinators::expr::expr_const;
use crate::parser::combinators::expr::indirection;
use crate::parser::combinators::foundation::located;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::param;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::opt_precision;
