/// Alias: `ImportForeignSchemaStmt`
pub(super) fn import_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        IMPORT_P FOREIGN SCHEMA ColId import_qualification FROM SERVER ColId INTO ColId create_generic_options
    */

    seq!(Import, Foreign, Schema, col_id)
        .map(|_| todo!())
        .parse(stream)
}

use pg_ast::RawStmt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Foreign;
use pg_lexer::Keyword::Import;
use pg_lexer::Keyword::Schema;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
use pg_sink_combinators::col_id;
