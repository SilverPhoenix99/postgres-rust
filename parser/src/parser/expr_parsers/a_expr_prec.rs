impl Parser<'_> {
    pub(super) fn a_expr_prec(&mut self, prec: i16) -> Result<AstNode, ScanErrorKind> {
        todo!()
    }

    fn a_expr_primary(&mut self) -> Result<AstNode, ScanErrorKind> {
        todo!()
    }
}

use crate::parser::ast_node::AstNode;
use crate::parser::result::ScanErrorKind;
use crate::parser::Parser;
