pub(super) fn param_expr() -> impl Combinator<Output = ExprNode> {

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

use crate::parser::ast_node::ExprNode;
use crate::parser::ast_node::ExprNode::ParamRef;
use crate::parser::ast_node::IndirectionExpr;
use crate::parser::combinators::expr::check_indirection;
use crate::parser::combinators::expr::indirection;
use crate::parser::combinators::foundation::located;
use crate::parser::combinators::foundation::param;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;

