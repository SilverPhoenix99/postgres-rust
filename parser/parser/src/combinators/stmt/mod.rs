mod begin_stmt;
mod end_stmt;
mod if_exists;
mod privilege_target;

pub(in crate::combinators) use self::begin_stmt::begin_stmt;
pub(in crate::combinators) use self::end_stmt::end_stmt;

pub(super) fn stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    alt!(
        abort_stmt.map(From::from),
        alter_stmt,
        analyze_stmt,
        call_stmt,
        cluster_stmt,
        check_point_stmt,
        close_stmt.map(ClosePortalStmt),
        comment_stmt.map(From::from),
        commit_stmt.map(From::from),
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
        release_savepoint_stmt.map(From::from),
        reset_stmt.map(VariableResetStmt),
        revoke_stmt,
        rollback_stmt.map(From::from),
        savepoint_stmt.map(From::from),
        security_label_stmt.map(From::from),
        set_stmt,
        show_stmt.map(VariableShowStmt),
        start_transaction_stmt.map(From::from),
        truncate_stmt,
        unlisten_stmt.map(UnlistenStmt),
        vacuum_stmt,
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_matrix;

    // This only quickly tests that statement types aren't missing.
    // More in-depth testing is within each statement's module.
    #[test_matrix(
        [
            "abort transaction",
            "alter group some_group add user public",
            "checkpoint",
            "close all",
            "comment on type int is 'comment'",
            "commit and no chain",
            "create database the_db with allow connections false",
            "deallocate all",
            "discard all",
            "listen ident",
            "load 'test string'",
            "notify test_ident, 'test-payload'",
            "prepare transaction 'tx id'",
            "reassign owned by public, test_role to target_role",
            "release savepoint test_ident",
            "reset time zone",
            "rollback to test_ident",
            "savepoint test_ident",
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
    abort_stmt,
    access_method,
    aggregate,
    aggregate_with_argtypes,
    alter_function_option,
    alter_role_option,
    alter_stmt,
    analyze_keyword,
    analyze_stmt,
    auth_ident,
    call_stmt,
    check_point_stmt,
    close_stmt,
    cluster_stmt,
    collation,
    column,
    comment_stmt,
    commit_stmt,
    conversion,
    copy_stmt,
    create_generic_options,
    create_stmt,
    database,
    deallocate_stmt,
    discard_stmt,
    do_stmt,
    domain,
    drop_stmt,
    event_trigger,
    explain_stmt,
    extension,
    fetch_stmt,
    foreign,
    function,
    if_not_exists,
    import_stmt,
    index,
    language,
    large_object,
    listen_stmt,
    load_stmt,
    lock_stmt,
    materialized_view,
    move_stmt,
    notify_stmt,
    operator,
    operator_with_argtypes,
    prepare_stmt,
    procedure,
    publication,
    reassign_owner_stmt,
    reindex_stmt,
    release_savepoint_stmt,
    reset_stmt,
    revoke_stmt,
    role,
    rollback_stmt,
    routine,
    savepoint_stmt,
    schema,
    security_label_stmt,
    sequence,
    server,
    set_rest,
    set_stmt,
    show_stmt,
    start_transaction_stmt,
    statistics,
    subscription,
    table,
    tablespace,
    text_search,
    transform,
    truncate_stmt,
    type_name,
    typecast,
    unlisten_stmt,
    utility_option,
    vacuum_stmt,
    variable_target,
    view,
}

use crate::combinators::foundation::alt;
use pg_ast::RawStmt;
use pg_ast::RawStmt::ClosePortalStmt;
use pg_ast::RawStmt::DeallocateStmt;
use pg_ast::RawStmt::ListenStmt;
use pg_ast::RawStmt::LoadStmt;
use pg_ast::RawStmt::UnlistenStmt;
use pg_ast::RawStmt::VariableResetStmt;
use pg_ast::RawStmt::VariableShowStmt;
use pg_combinators::Combinator;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
