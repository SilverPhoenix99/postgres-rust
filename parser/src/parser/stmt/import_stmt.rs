/// Alias: `ImportForeignSchemaStmt`
pub(in crate::parser) fn import_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        IMPORT_P FOREIGN SCHEMA ColId import_qualification FROM SERVER ColId INTO ColId create_generic_options
    */

    keyword(Import)
        .and(keyword(Foreign))
        .and(keyword(Schema))
        .and_right(col_id())
        .map(|_| todo!())
}

use crate::lexer::Keyword::Schema;
use crate::lexer::Keyword::{Foreign, Import};
use crate::parser::ast_node::RawStmt;
use crate::parser::col_id;
use crate::parser::combinators::{keyword, Combinator, CombinatorHelpers};
