impl Parser<'_> {
    /// Alias: `SecLabelStmt`
    pub(in crate::parser) fn security_stmt(&mut self) -> OptResult<AstNode> {

        /*
            SECURITY LABEL opt_provider ON AGGREGATE aggregate_with_argtypes IS security_label
            SECURITY LABEL opt_provider ON COLUMN any_name IS security_label
            SECURITY LABEL opt_provider ON DOMAIN_P Typename IS security_label
            SECURITY LABEL opt_provider ON FUNCTION function_with_argtypes IS security_label
            SECURITY LABEL opt_provider ON LARGE_P OBJECT_P NumericOnly IS security_label
            SECURITY LABEL opt_provider ON object_type_any_name any_name IS security_label
            SECURITY LABEL opt_provider ON object_type_name ColId IS security_label
            SECURITY LABEL opt_provider ON PROCEDURE function_with_argtypes IS security_label
            SECURITY LABEL opt_provider ON ROUTINE function_with_argtypes IS security_label
            SECURITY LABEL opt_provider ON TYPE_P Typename IS security_label
        */
        
        if self.buffer.consume_kw_eq(Security)?.is_none() {
            return Ok(None)
        }

        self.buffer.consume_kw_eq(Label).required()?;

        todo!()
    }
}

use crate::lexer::Keyword::{Label, Security};
use crate::parser::result::OptionalResult;
use crate::parser::AstNode;
use crate::parser::OptResult;
use crate::parser::Parser;
