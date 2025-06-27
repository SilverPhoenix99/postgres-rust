mod abort_stmt;
mod access_method;
mod aggregate;
mod aggregate_with_argtypes;
mod alter_function_option;
mod alter_role_option;
mod alter_stmt;
mod analyze_stmt;
mod auth_ident;
mod begin_stmt;
mod call_stmt;
mod close_stmt;
mod cluster_stmt;
mod collation;
mod column;
mod comment_stmt;
mod commit_stmt;
mod conversion;
mod copy_stmt;
mod create_generic_options;
mod create_stmt;
mod database;
mod deallocate_stmt;
mod discard_stmt;
mod do_stmt;
mod domain;
mod drop_stmt;
mod end_stmt;
mod event_trigger;
mod explain_stmt;
mod extension;
mod fetch_stmt;
mod foreign;
mod function;
mod if_not_exists;
mod import_stmt;
mod index;
mod language;
mod large_object;
mod listen_stmt;
mod load_stmt;
mod lock_stmt;
mod materialized_view;
mod move_stmt;
mod notify_stmt;
mod operator;
mod operator_with_argtypes;
mod prepare_stmt;
mod privilege_target;
mod procedure;
mod publication;
mod reassign_owner_stmt;
mod reindex_stmt;
mod release_savepoint_stmt;
mod reset_stmt;
mod revoke_stmt;
mod role;
mod rollback_stmt;
mod routine;
mod savepoint_stmt;
mod schema;
mod security_label_stmt;
mod sequence;
mod server;
mod set_rest;
mod set_stmt;
mod show_stmt;
mod start_transaction_stmt;
mod statistics;
mod subscription;
mod table;
mod tablespace;
mod text_search;
mod transform;
mod truncate_stmt;
mod type_name;
mod typecast;
mod unlisten_stmt;
mod vacuum_stmt;
mod variable_target;
mod view;

pub(in crate::combinators) use self::begin_stmt::begin_stmt;
pub(in crate::combinators) use self::end_stmt::end_stmt;

pub(super) fn stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    choice!(parsed stream =>
        abort_stmt.map(From::from),
        alter_stmt,
        analyze_stmt,
        call_stmt,
        cluster_stmt,
        Checkpoint.map(|_| CheckPoint),
        close_stmt.map(ClosePortalStmt),
        comment_stmt.map(From::from),
        commit_stmt().map(From::from),
        copy_stmt(),
        create_stmt(),
        deallocate_stmt().map(DeallocateStmt),
        discard_stmt().map(From::from),
        do_stmt(),
        drop_stmt(),
        explain_stmt(),
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
    )
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

#[allow(unused_imports)] // TODO: eventually remove
use self::{
    abort_stmt::abort_stmt,
    access_method::access_method,
    aggregate::aggregate,
    aggregate_with_argtypes::{aggr_args, aggregate_with_argtypes, aggregate_with_argtypes_list},
    alter_function_option::alter_function_option,
    alter_role_option::{alter_role_option, alter_role_options},
    alter_stmt::alter_stmt,
    analyze_stmt::analyze_stmt,
    auth_ident::auth_ident,
    call_stmt::call_stmt,
    close_stmt::close_stmt,
    cluster_stmt::cluster_stmt,
    collation::collation,
    column::column,
    comment_stmt::comment_stmt,
    commit_stmt::commit_stmt,
    conversion::conversion,
    copy_stmt::copy_stmt,
    create_generic_options::create_generic_options,
    create_stmt::create_stmt,
    create_stmt::createdb_opt_value,
    database::database,
    deallocate_stmt::deallocate_stmt,
    discard_stmt::discard_stmt,
    do_stmt::do_stmt,
    domain::domain,
    drop_stmt::drop_stmt,
    event_trigger::event_trigger,
    explain_stmt::explain_stmt,
    extension::extension,
    fetch_stmt::fetch_stmt,
    foreign::{foreign, Foreign},
    function::function,
    if_not_exists::if_not_exists,
    import_stmt::import_stmt,
    index::index,
    language::language,
    large_object::large_object,
    listen_stmt::listen_stmt,
    load_stmt::load_stmt,
    lock_stmt::lock_stmt,
    materialized_view::materialized_view,
    move_stmt::move_stmt,
    notify_stmt::notify_stmt,
    operator::{operator, Operator},
    operator_with_argtypes::{operator_with_argtypes, operator_with_argtypes_list},
    prepare_stmt::prepare_stmt,
    procedure::procedure,
    publication::publication,
    reassign_owner_stmt::reassign_owned_stmt,
    reindex_stmt::reindex_stmt,
    release_savepoint_stmt::release_savepoint_stmt,
    reset_stmt::reset_stmt,
    revoke_stmt::revoke_stmt,
    role::role,
    rollback_stmt::rollback_stmt,
    routine::routine,
    savepoint_stmt::savepoint_stmt,
    schema::schema,
    security_label_stmt::security_label_stmt,
    sequence::sequence,
    server::server,
    set_rest::{set_rest, set_rest_more},
    set_stmt::set_stmt,
    show_stmt::show_stmt,
    start_transaction_stmt::start_transaction_stmt,
    statistics::statistics,
    subscription::subscription,
    table::table,
    tablespace::tablespace,
    text_search::{text_search, TextSearch},
    transform::transform,
    truncate_stmt::truncate_stmt,
    type_name::type_name,
    typecast::typecast,
    unlisten_stmt::unlisten_stmt,
    vacuum_stmt::vacuum_stmt,
    variable_target::variable_target,
    view::view,
};
use crate::combinators::foundation::choice;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_ast::RawStmt::CheckPoint;
use pg_ast::RawStmt::ClosePortalStmt;
use pg_ast::RawStmt::DeallocateStmt;
use pg_ast::RawStmt::ListenStmt;
use pg_ast::RawStmt::LoadStmt;
use pg_ast::RawStmt::UnlistenStmt;
use pg_ast::RawStmt::VariableResetStmt;
use pg_ast::RawStmt::VariableShowStmt;
use pg_lexer::Keyword::Checkpoint;
