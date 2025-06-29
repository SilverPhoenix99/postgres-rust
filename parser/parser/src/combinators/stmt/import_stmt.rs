/// Alias: `ImportForeignSchemaStmt`
pub(super) fn import_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        IMPORT_P FOREIGN SCHEMA ColId import_qualification FROM SERVER ColId INTO ColId create_generic_options
    */

    (Import, Foreign, Schema, col_id)
        .map(|_| todo!())
        .parse(stream)
}

use crate::combinators::col_id;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Foreign;
use pg_lexer::Keyword::Import;
use pg_lexer::Keyword::Schema;
