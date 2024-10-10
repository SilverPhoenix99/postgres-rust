impl Parser<'_> {
    pub(super) fn a_expr_prec(&mut self, prec: i16) -> ScanResult<AstNode> {
        todo!()
    }

    fn a_expr_primary(&mut self) -> ScanResult<AstNode> {
        todo!()
    }
}

use crate::parser::ast_node::AstNode;
use crate::parser::result::ScanResult;
use crate::parser::Parser;
