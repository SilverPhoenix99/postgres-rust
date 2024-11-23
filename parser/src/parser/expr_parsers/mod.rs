mod indirection;
mod associativity;
mod a_expr_prec;
mod b_expr_prec;
mod case_expr;
mod expr_const;
mod expr_primary;

pub(super) use indirection::indirection;

impl Parser<'_> {

    #[inline(always)]
    pub(in crate::parser) fn a_expr(&mut self) -> ScanResult<ExprNode> {
        self.a_expr_prec(0)
    }

    #[inline(always)]
    pub(in crate::parser) fn b_expr(&mut self) -> ScanResult<ExprNode> {
        self.b_expr_prec(0)
    }
}

fn a_expr() -> impl Combinator<Output = ExprNode> {
    // TODO
    expr_primary()
}

use crate::parser::ast_node::ExprNode;
use crate::parser::combinators::Combinator;
use crate::parser::result::ScanResult;
use crate::parser::Parser;
use expr_primary::expr_primary;
