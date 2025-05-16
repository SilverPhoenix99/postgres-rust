/// Alias: `VacuumStmt`
pub(super) fn vacuum_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        VACUUM opt_full opt_freeze opt_verbose opt_analyze opt_vacuum_relation_list
        VACUUM '(' utility_option_list ')' opt_vacuum_relation_list
    */

    Vacuum
        .map(|_| todo!())
}

use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Vacuum;
