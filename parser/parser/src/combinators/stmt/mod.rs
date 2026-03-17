mod object_type_name;
mod privilege_target;

pub(super) fn stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    alt!(
        alter_stmt,
        analyze_stmt,
        call_stmt,
        check_point_stmt.map(CheckPointStmt),
        close_stmt.map(ClosePortalStmt),
        comment_stmt.map(From::from),
        copy_stmt,
        create_stmt,
        deallocate_stmt.map(DeallocateStmt),
        discard_stmt.map(From::from),
        do_stmt,
        drop_stmt,
        explain_stmt,
        fetch_stmt,
        import_stmt,
        listen_stmt.map(ListenStmt),
        load_stmt.map(LoadStmt),
        lock_stmt,
        move_stmt,
        notify_stmt.map(From::from),
        prepare_stmt,
        reassign_owned_stmt.map(From::from),
        reindex_stmt,
        repack_stmt,
        reset_stmt.map(VariableResetStmt),
        revoke_stmt,
        security_label_stmt.map(From::from),
        set_stmt,
        show_stmt.map(VariableShowStmt),
        transaction_stmt.map(From::from),
        truncate_stmt,
        unlisten_stmt.map(UnlistenStmt),
        vacuum_stmt,
        wait_stmt,
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_matrix;

    // This only quickly tests that statement types aren't missing.
    // More in-depth testing is within each statement's module.
    #[test_matrix(
        [
            "alter group some_group add user public",
            "checkpoint",
            "close all",
            "comment on type int is 'comment'",
            "create database the_db with allow connections false",
            "deallocate all",
            "discard all",
            "listen ident",
            "load 'test string'",
            "notify test_ident, 'test-payload'",
            "prepare transaction 'tx id'",
            "reassign owned by public, test_role to target_role",
            "reset time zone",
            "security label for 'foo' on type int is 'bar'",
            "set schema 'abc123'",
            "show all",
            "start transaction read only, read write deferrable",
            "unlisten *",
        ]
        => matches Ok(_)
    )]
    fn test_stmt(source: &str) -> scan::Result<RawStmt> {
        test_parser!(source, stmt)
    }
}

pg_basics::reexport! {
    access_method_stmt,
    aggregate_stmt,
    aggregate_with_argtypes,
    alter_function_option,
    alter_stmt,
    analyze_stmt,
    call_stmt,
    cast_stmt,
    check_point_stmt,
    close_stmt,
    collation_stmt,
    comment_stmt,
    conversion_stmt,
    copy_stmt,
    create_stmt,
    database_stmt,
    deallocate_stmt,
    default_privileges_stmt,
    discard_stmt,
    do_stmt,
    drop_stmt,
    event_trigger_stmt,
    explain_stmt,
    extension_stmt,
    fetch_stmt,
    function_stmt,
    generic_options,
    import_stmt,
    language_stmt,
    large_object_stmt,
    listen_stmt,
    load_stmt,
    lock_stmt,
    move_stmt,
    notify_stmt,
    operator_with_argtypes,
    prepare_stmt,
    reassign_owner_stmt,
    reindex_stmt,
    repack_stmt,
    reset_stmt,
    revoke_stmt,
    role_stmt,
    security_label_stmt,
    set_reset_clause,
    set_rest,
    set_stmt,
    show_stmt,
    system_stmt,
    truncate_stmt,
    unlisten_stmt,
    utility_options,
    vacuum_stmt,
    wait_stmt,
}

mod transaction_stmt;
pub(in crate::combinators) use transaction_stmt::transaction_stmt_legacy;
use transaction_stmt::{transaction_mode_list, transaction_stmt};

mod select_stmt;
pub(in crate::combinators) use select_stmt::select_stmt;

use object_type_name::{
    access_method::*,
    aggregate::*,
    collation::*,
    column::*,
    conversion::*,
    database::*,
    domain::*,
    event_trigger::*,
    extension::*,
    foreign::*,
    function::*,
    index::*,
    language::*,
    large_object::*,
    materialized_view::*,
    operator::*,
    procedure::*,
    publication::*,
    role::*,
    routine::*,
    schema::*,
    sequence::*,
    server::*,
    statistics::*,
    subscription::*,
    table::*,
    tablespace::*,
    text_search::*,
    transform::*,
    type_name::*,
    typecast::*,
    view::*,
};

use crate::alt;
use crate::combinators::core::Combinator;
use crate::ParserContext;
use pg_ast::RawStmt;
use pg_ast::RawStmt::CheckPointStmt;
use pg_ast::RawStmt::ClosePortalStmt;
use pg_ast::RawStmt::DeallocateStmt;
use pg_ast::RawStmt::ListenStmt;
use pg_ast::RawStmt::LoadStmt;
use pg_ast::RawStmt::UnlistenStmt;
use pg_ast::RawStmt::VariableResetStmt;
use pg_ast::RawStmt::VariableShowStmt;
use pg_parser_core::scan;
