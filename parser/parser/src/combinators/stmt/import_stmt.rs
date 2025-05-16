/// Alias: `ImportForeignSchemaStmt`
pub(super) fn import_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        IMPORT_P FOREIGN SCHEMA ColId import_qualification FROM SERVER ColId INTO ColId create_generic_options
    */

    Import.and(Foreign).and(Schema)
        .and_right(col_id())
        .map(|_| todo!())
}

use crate::combinators::col_id;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Foreign;
use pg_lexer::Keyword::Import;
use pg_lexer::Keyword::Schema;
