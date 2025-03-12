/// Alias: `VacuumStmt`
pub(super) fn vacuum_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        VACUUM opt_full opt_freeze opt_verbose opt_analyze opt_vacuum_relation_list
        VACUUM '(' utility_option_list ')' opt_vacuum_relation_list
    */

    Vacuum
        .map(|_| todo!())
}

use crate::lexer::Keyword::Vacuum;
use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
