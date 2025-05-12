/// Alias: `ImportForeignSchemaStmt`
pub(super) fn import_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        IMPORT_P FOREIGN SCHEMA ColId import_qualification FROM SERVER ColId INTO ColId create_generic_options
    */

    Import.and(Foreign).and(Schema)
        .and_right(col_id())
        .map(|_| todo!())
}

use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_parser_lexer::Keyword::Foreign;
use postgres_parser_lexer::Keyword::Import;
use postgres_parser_lexer::Keyword::Schema;
