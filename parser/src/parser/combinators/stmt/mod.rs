mod abort_stmt;
mod alter_stmt;
mod analyze_stmt;
mod begin_stmt;
mod call_stmt;
mod close_stmt;
mod cluster_stmt;
mod comment_stmt;
mod commit_stmt;
mod copy_stmt;
mod create_stmt;
mod deallocate_stmt;
mod discard_stmt;
mod do_stmt;
mod drop_stmt;
mod end_stmt;
mod explain_stmt;
mod fetch_stmt;
mod import_stmt;
mod listen_stmt;
mod load_stmt;
mod lock_stmt;
mod move_stmt;
mod notify_stmt;
mod prepare_stmt;
mod reassign_owner_stmt;
mod reindex_stmt;
mod release_savepoint_stmt;
mod reset_stmt;
mod revoke_stmt;
mod rollback_stmt;
mod savepoint_stmt;
mod security_stmt;
mod set_stmt;
mod show_stmt;
mod start_transaction_stmt;
mod truncate_stmt;
mod unlisten_stmt;
mod vacuum_stmt;
mod variable_target;

pub(in crate::parser::combinators) use self::begin_stmt::begin_stmt;
pub(in crate::parser::combinators) use self::end_stmt::end_stmt;

pub(super) fn stmt() -> impl Combinator<Output = RawStmt> {

    match_first! {
        abort_stmt().map(From::from),
        alter_stmt(),
        analyze_stmt(),
        call_stmt(),
        cluster_stmt(),
        Checkpoint.map(|_| CheckPoint),
        close_stmt().map(ClosePortalStmt),
        comment_stmt(),
        commit_stmt().map(From::from),
        copy_stmt(),
        create_stmt(),
        deallocate_stmt().map(DeallocateStmt),
        discard_stmt().map(From::from),
        do_stmt(),
        drop_stmt(),
        explain_stmt(),
        fetch_stmt(),
        import_stmt(),
        listen_stmt().map(ListenStmt),
        load_stmt().map(LoadStmt),
        lock_stmt(),
        move_stmt(),
        notify_stmt().map(From::from),
        prepare_stmt(),
        reassign_owned_stmt().map(From::from),
        reindex_stmt(),
        release_savepoint_stmt().map(From::from),
        reset_stmt().map(VariableResetStmt),
        revoke_stmt(),
        rollback_stmt().map(From::from),
        savepoint_stmt().map(From::from),
        security_stmt(),
        set_stmt(),
        show_stmt().map(VariableShowStmt),
        start_transaction_stmt().map(From::from),
        truncate_stmt(),
        unlisten_stmt().map(UnlistenStmt),
        vacuum_stmt(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test_case("abort transaction")]
    #[test_case("alter group some_group add user public")]
    #[test_case("close all")]
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
    #[test_case("show all")]
    #[test_case("start transaction read only, read write deferrable")]
    #[test_case("unlisten *")]
    fn test_stmt(source: &str) {

        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = stmt().parse(&mut stream);

        // This only quickly tests that statement types aren't missing.
        // More in-depth testing is within each statement's module.
        assert_matches!(actual, Ok(_),
            r"expected Ok(Some(_)) for {source:?} but actually got {actual:?}"
        )
    }
}

use self::{
    abort_stmt::abort_stmt,
    alter_stmt::alter_stmt,
    analyze_stmt::analyze_stmt,
    call_stmt::call_stmt,
    close_stmt::close_stmt,
    cluster_stmt::cluster_stmt,
    comment_stmt::comment_stmt,
    commit_stmt::commit_stmt,
    copy_stmt::copy_stmt,
    create_stmt::create_stmt,
    create_stmt::createdb_opt_value,
    deallocate_stmt::deallocate_stmt,
    discard_stmt::discard_stmt,
    do_stmt::do_stmt,
    drop_stmt::drop_stmt,
    explain_stmt::explain_stmt,
    fetch_stmt::fetch_stmt,
    import_stmt::import_stmt,
    listen_stmt::listen_stmt,
    load_stmt::load_stmt,
    lock_stmt::lock_stmt,
    move_stmt::move_stmt,
    notify_stmt::notify_stmt,
    prepare_stmt::prepare_stmt,
    reassign_owner_stmt::reassign_owned_stmt,
    reindex_stmt::reindex_stmt,
    release_savepoint_stmt::release_savepoint_stmt,
    reset_stmt::reset_stmt,
    revoke_stmt::revoke_stmt,
    rollback_stmt::rollback_stmt,
    savepoint_stmt::savepoint_stmt,
    security_stmt::security_stmt,
    set_stmt::set_stmt,
    show_stmt::show_stmt,
    start_transaction_stmt::start_transaction_stmt,
    truncate_stmt::truncate_stmt,
    unlisten_stmt::unlisten_stmt,
    vacuum_stmt::vacuum_stmt,
    variable_target::variable_target
};
use crate::lexer::Keyword::Checkpoint;
use crate::parser::ast_node::RawStmt;
use crate::parser::ast_node::RawStmt::CheckPoint;
use crate::parser::ast_node::RawStmt::ClosePortalStmt;
use crate::parser::ast_node::RawStmt::DeallocateStmt;
use crate::parser::ast_node::RawStmt::ListenStmt;
use crate::parser::ast_node::RawStmt::LoadStmt;
use crate::parser::ast_node::RawStmt::UnlistenStmt;
use crate::parser::ast_node::RawStmt::VariableResetStmt;
use crate::parser::ast_node::RawStmt::VariableShowStmt;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
