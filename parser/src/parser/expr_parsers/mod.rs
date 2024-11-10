pub(super) mod indirection;
mod associativity;
mod a_expr_prec;
mod b_expr_prec;
mod expr_const_parser;
mod case_expr;

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

use crate::parser::{
    ast_node::ExprNode,
    result::ScanResult,
    Parser
};
