pub(super) fn stmt() -> impl Combinator<Output = RawStmt> {
    use crate::lexer::Keyword::*;

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

use crate::parser::ast_node::RawStmt;
use crate::parser::ast_node::RawStmt::{ClosePortalStmt, DeallocateStmt, ListenStmt, LoadStmt, UnlistenStmt};
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::combinators::{keyword, match_first};
use crate::parser::stmt_parsers::abort_stmt;
use crate::parser::stmt_parsers::alter_stmt;
use crate::parser::stmt_parsers::analyze_stmt;
use crate::parser::stmt_parsers::call_stmt;
use crate::parser::stmt_parsers::close_stmt;
use crate::parser::stmt_parsers::cluster_stmt;
use crate::parser::stmt_parsers::comment_stmt;
use crate::parser::stmt_parsers::commit_stmt;
use crate::parser::stmt_parsers::copy_stmt;
use crate::parser::stmt_parsers::deallocate_stmt;
use crate::parser::stmt_parsers::discard_stmt;
use crate::parser::stmt_parsers::do_stmt;
use crate::parser::stmt_parsers::drop_stmt;
use crate::parser::stmt_parsers::explain_stmt;
use crate::parser::stmt_parsers::fetch_stmt;
use crate::parser::stmt_parsers::import_stmt;
use crate::parser::stmt_parsers::listen_stmt;
use crate::parser::stmt_parsers::load_stmt;
use crate::parser::stmt_parsers::lock_stmt;
use crate::parser::stmt_parsers::move_stmt;
use crate::parser::stmt_parsers::notify_stmt;
use crate::parser::stmt_parsers::prepare_stmt;
use crate::parser::stmt_parsers::reassign_owned_stmt;
use crate::parser::stmt_parsers::reindex_stmt;
use crate::parser::stmt_parsers::release_savepoint_stmt;
use crate::parser::stmt_parsers::revoke_stmt;
use crate::parser::stmt_parsers::rollback_stmt;
use crate::parser::stmt_parsers::savepoint_stmt;
use crate::parser::stmt_parsers::security_stmt;
use crate::parser::stmt_parsers::set_stmt;
use crate::parser::stmt_parsers::show_stmt;
use crate::parser::stmt_parsers::start_transaction_stmt;
use crate::parser::stmt_parsers::truncate_stmt;
use crate::parser::stmt_parsers::unlisten_stmt;
use crate::parser::stmt_parsers::vacuum_stmt;
