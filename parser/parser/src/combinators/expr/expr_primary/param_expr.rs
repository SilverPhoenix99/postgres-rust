pub(super) fn param_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        PARAM ( indirection )?

        E.g: $1.foo[0].*
    */

    let (index, indirection) = seq!(
        param,
        located!(indirection).optional()
    ).parse(stream)?;

    let param = ParamRef { index };
    let expr = match indirection {
        None => param,
        Some(indirection) => {
            let indirection = check_indirection(indirection)?;
            IndirectionExpr::new(param, indirection).into()
        },
    };

    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_ast::Indirection::Slice;
    use pg_combinators::test_parser;

    #[test]
    fn test_param_expr() {
        test_parser!(
            source = "$5[:]",
            parser = param_expr,
            expected = IndirectionExpr::new(
                ParamRef { index: 5 },
                vec![Slice(None, None)]
            )
        )
    }
}

use crate::combinators::expr::check_indirection;
use crate::combinators::expr::indirection;
use crate::combinators::foundation::located;
use crate::combinators::foundation::param;
use crate::combinators::foundation::seq;
use pg_ast::ExprNode;
use pg_ast::ExprNode::ParamRef;
use pg_ast::IndirectionExpr;
use pg_combinators::Combinator;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
