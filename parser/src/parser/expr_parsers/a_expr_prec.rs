impl Parser<'_> {
    pub(super) fn a_expr_prec(&mut self, prec: i16) -> ScanResult<ExprNode> {
        todo!()
    }

    fn a_expr_primary(&mut self) -> ScanResult<ExprNode> {
        todo!()
    }
}

use crate::parser::ast_node::ExprNode;
use crate::parser::result::ScanResult;
use crate::parser::Parser;
