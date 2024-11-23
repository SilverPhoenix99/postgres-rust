mod abort_stmt;
mod alter_collation_stmt;
mod alter_conversion_stmt;
mod alter_default_privileges_stmt;
mod alter_event_trigger_stmt;
mod alter_group_stmt;
mod alter_language_stmt;
mod alter_large_object_stmt;
mod alter_stmt;
mod analyze_stmt;
mod begin_stmt;
mod call_stmt;
mod close_stmt;
mod cluster_stmt;
mod comment_stmt;
mod commit_stmt;
mod copy_stmt;
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

pub(super) fn stmt() -> impl Combinator<Output = RawStmt> {

    match_first! {
        abort_stmt().map(From::from),
        alter_stmt(),
        analyze_stmt(),
        call_stmt(),
        cluster_stmt(),
        keyword(Checkpoint).map(|_| RawStmt::CheckPoint),
        close_stmt().map(ClosePortalStmt),
        comment_stmt(),
        commit_stmt().map(From::from),
        copy_stmt(),
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
        revoke_stmt(),
        rollback_stmt().map(From::from),
        savepoint_stmt().map(From::from),
        security_stmt(),
        set_stmt(),
        show_stmt().map(From::from),
        start_transaction_stmt().map(From::from),
        truncate_stmt(),
        unlisten_stmt().map(UnlistenStmt),
        vacuum_stmt(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test_case("abort transaction")]
    #[test_case("group some_group add user public")]
    #[test_case("close all")]
    #[test_case("commit and no chain")]
    #[test_case("deallocate all")]
    #[test_case("discard all")]
    #[test_case("listen ident")]
    #[test_case("load 'test string'")]
    #[test_case("notify test_ident, 'test-payload'")]
    #[test_case("prepare transaction 'tx id'")]
    #[test_case("reassign owned by public, test_role to target_role")]
    #[test_case("release savepoint test_ident")]
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

pub(in crate::parser) use self::begin_stmt::begin_stmt;
pub(in crate::parser) use self::end_stmt::end_stmt;

use self::abort_stmt::abort_stmt;
use self::alter_stmt::alter_stmt;
use self::analyze_stmt::analyze_stmt;
use self::call_stmt::call_stmt;
use self::close_stmt::close_stmt;
use self::cluster_stmt::cluster_stmt;
use self::comment_stmt::comment_stmt;
use self::commit_stmt::commit_stmt;
use self::copy_stmt::copy_stmt;
use self::deallocate_stmt::deallocate_stmt;
use self::discard_stmt::discard_stmt;
use self::do_stmt::do_stmt;
use self::drop_stmt::drop_stmt;
use self::explain_stmt::explain_stmt;
use self::fetch_stmt::fetch_stmt;
use self::import_stmt::import_stmt;
use self::listen_stmt::listen_stmt;
use self::load_stmt::load_stmt;
use self::lock_stmt::lock_stmt;
use self::move_stmt::move_stmt;
use self::notify_stmt::notify_stmt;
use self::prepare_stmt::prepare_stmt;
use self::reassign_owner_stmt::reassign_owned_stmt;
use self::reindex_stmt::reindex_stmt;
use self::release_savepoint_stmt::release_savepoint_stmt;
use self::revoke_stmt::revoke_stmt;
use self::rollback_stmt::rollback_stmt;
use self::savepoint_stmt::savepoint_stmt;
use self::security_stmt::security_stmt;
use self::set_stmt::set_stmt;
use self::show_stmt::show_stmt;
use self::start_transaction_stmt::start_transaction_stmt;
use self::truncate_stmt::truncate_stmt;
use self::unlisten_stmt::unlisten_stmt;
use self::vacuum_stmt::vacuum_stmt;
use crate::lexer::Keyword::Checkpoint;
use crate::parser::ast_node::RawStmt;
use crate::parser::ast_node::RawStmt::ClosePortalStmt;
use crate::parser::ast_node::RawStmt::DeallocateStmt;
use crate::parser::ast_node::RawStmt::ListenStmt;
use crate::parser::ast_node::RawStmt::LoadStmt;
use crate::parser::ast_node::RawStmt::UnlistenStmt;
use crate::parser::combinators::keyword;
use crate::parser::combinators::match_first;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
