/// Alias: `SecLabelStmt`
pub(super) fn security_stmt() -> impl Combinator<Output = RawStmt> {

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

    Security.and(Label)
        .map(|_| todo!())
}

use crate::lexer::Keyword::Label;
use crate::lexer::Keyword::Security;
use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
