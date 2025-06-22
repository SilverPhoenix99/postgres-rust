/// Alias: `DropStmt`
pub(super) fn drop_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        DROP DOMAIN_P IF_P EXISTS type_name_list opt_drop_behavior
        DROP DOMAIN_P type_name_list opt_drop_behavior
        DROP drop_type_name IF_P EXISTS name_list opt_drop_behavior
        DROP drop_type_name name_list opt_drop_behavior
        DROP INDEX CONCURRENTLY any_name_list opt_drop_behavior
        DROP INDEX CONCURRENTLY IF_P EXISTS any_name_list opt_drop_behavior
        DROP object_type_any_name any_name_list opt_drop_behavior
        DROP object_type_any_name IF_P EXISTS any_name_list opt_drop_behavior
        DROP object_type_name_on_any_name ColId ON any_name opt_drop_behavior
        DROP object_type_name_on_any_name IF_P EXISTS ColId ON any_name opt_drop_behavior
        DROP TYPE_P IF_P EXISTS type_name_list opt_drop_behavior
        DROP TYPE_P type_name_list opt_drop_behavior
    */

    DropKw
        .map(|_| todo!())
}

use crate::combinators::foundation::Combinator;
use pg_ast::RawStmt;
use pg_lexer::Keyword::DropKw;
