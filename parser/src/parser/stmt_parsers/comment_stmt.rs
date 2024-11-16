impl Parser<'_> {
    /// Alias: `CommentStmt`
    pub(in crate::parser) fn comment_stmt(&mut self) -> ParseResult<RawStmt> {

        /*
            COMMENT ON AGGREGATE aggregate_with_argtypes IS comment_text
            COMMENT ON CAST '(' Typename AS Typename ')' IS comment_text
            COMMENT ON COLUMN any_name IS comment_text
            COMMENT ON CONSTRAINT ColId ON any_name IS comment_text
            COMMENT ON CONSTRAINT ColId ON DOMAIN_P any_name IS comment_text
            COMMENT ON DOMAIN_P Typename IS comment_text
            COMMENT ON FUNCTION function_with_argtypes IS comment_text
            COMMENT ON LARGE_P OBJECT_P NumericOnly IS comment_text
            COMMENT ON object_type_any_name any_name IS comment_text
            COMMENT ON object_type_name ColId IS comment_text
            COMMENT ON object_type_name_on_any_name ColId ON any_name IS comment_text
            COMMENT ON OPERATOR CLASS any_name USING ColId IS comment_text
            COMMENT ON OPERATOR FAMILY any_name USING ColId IS comment_text
            COMMENT ON OPERATOR operator_with_argtypes IS comment_text
            COMMENT ON PROCEDURE function_with_argtypes IS comment_text
            COMMENT ON ROUTINE function_with_argtypes IS comment_text
            COMMENT ON TRANSFORM FOR Typename LANGUAGE ColId IS comment_text
            COMMENT ON TYPE_P Typename IS comment_text
        */

        self.buffer.consume_kw_eq(On).required(fn_info!())?;

        todo!()
    }
}

use crate::{
    lexer::Keyword::On,
    parser::{
        ast_node::RawStmt,
        result::Required,
        ParseResult,
        Parser
    }
};
use postgres_basics::fn_info;
