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
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("abort transaction")]
    #[test_case("alter group some_group add user public")]
    #[test_case("checkpoint")]
    #[test_case("close all")]
    #[test_case("comment on type int is 'comment'")]
    #[test_case("commit and no chain")]
    #[test_case("create database the_db with allow connections false")]
    #[test_case("deallocate all")]
    #[test_case("discard all")]
    #[test_case("listen ident")]
    #[test_case("load 'test string'")]
    #[test_case("notify test_ident, 'test-payload'")]
    #[test_case("prepare transaction 'tx id'")]
    #[test_case("reassign owned by public, test_role to target_role")]
    #[test_case("release savepoint test_ident")]
    #[test_case("reset time zone")]
    #[test_case("rollback to test_ident")]
    #[test_case("savepoint test_ident")]
    #[test_case("security label for 'foo' on type int is 'bar'")]
    #[test_case("set schema 'abc123'")]
    #[test_case("show all")]
    #[test_case("start transaction read only, read write deferrable")]
    #[test_case("unlisten *")]
    fn test_stmt(source: &str) {

        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = stmt(&mut stream);

        // This only quickly tests that statement types aren't missing.
        // More in-depth testing is within each statement's module.
        assert_matches!(actual, Ok(_),
            r"expected Ok(Some(_)) for {source:?} but actually got {actual:?}"
        )
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
use crate::combinators::foundation::Combinator;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_ast::RawStmt::ClosePortalStmt;
use pg_ast::RawStmt::DeallocateStmt;
use pg_ast::RawStmt::ListenStmt;
use pg_ast::RawStmt::LoadStmt;
use pg_ast::RawStmt::UnlistenStmt;
use pg_ast::RawStmt::VariableResetStmt;
use pg_ast::RawStmt::VariableShowStmt;
use pg_parser_core::scan;
