impl Parser<'_> {

    pub(super) fn stmt(&mut self, allow_tx_legacy_stmts: bool) -> ParseResult<RawStmt> {
        use TokenKind::Keyword as Kw;
        use Keyword::*;
        const FN_NAME: &str = "postgres_parser::parser::Parser::stmt";

        consume! {self
            Ok {
                Kw(Begin) if allow_tx_legacy_stmts => self.begin_stmt().map(From::from),
                Kw(End) if allow_tx_legacy_stmts => self.end_stmt().map(From::from),
                Kw(Checkpoint) => Ok(RawStmt::CheckPoint),
                Kw(Abort) => self.abort_stmt().map(From::from),
                Kw(Alter) => self.alter_stmt(),
                Kw(Analyse | Analyze) => self.analyze_stmt(),
                Kw(Call) => self.call_stmt(),
                Kw(Close) => self.close_stmt().map(ClosePortalStmt),
                Kw(Cluster) => self.cluster_stmt(),
                Kw(Comment) => self.comment_stmt(),
                Kw(Commit) => self.commit_stmt().map(From::from),
                Kw(CopyKw) => self.copy_stmt(),
                Kw(Deallocate) => self.deallocate_stmt().map(DeallocateStmt),
                Kw(Discard) => self.discard_stmt().map(From::from),
                Kw(Do) => self.do_stmt(),
                Kw(DropKw) => self.drop_stmt(),
                Kw(Explain) => self.explain_stmt(),
                Kw(Fetch) => self.fetch_stmt(),
                Kw(Import) => self.import_stmt(),
                Kw(Listen) => self.listen_stmt().map(ListenStmt),
                Kw(Load) => self.load_stmt().map(LoadStmt),
                Kw(Lock) => self.lock_stmt(),
                Kw(Move) => self.move_stmt(),
                Kw(Notify) => self.notify_stmt().map(From::from),
                Kw(Prepare) => self.prepare_stmt(),
                Kw(Reassign) => self.reassign_owned_stmt().map(From::from),
                Kw(Reindex) => self.reindex_stmt(),
                Kw(Release) => self.release_savepoint_stmt().map(From::from),
                Kw(Revoke) => self.revoke_stmt(),
                Kw(Rollback) => self.rollback_stmt().map(From::from),
                Kw(Savepoint) => self.savepoint_stmt().map(From::from),
                Kw(Security) => self.security_stmt(),
                Kw(Set) => self.set_stmt(),
                Kw(Show) => self.show_stmt().map(From::from),
                Kw(Start) => self.start_transaction_stmt().map(From::from),
                Kw(Truncate) => self.truncate_stmt(),
                Kw(Unlisten) => self.unlisten_stmt().map(UnlistenStmt),
                Kw(Vacuum) => self.vacuum_stmt(),
            }
            Err {
                Ok(_) | Err(EofErrorKind::Eof) => syntax_err!(FN_NAME),
                Err(NotEof(err)) => err,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("abort transaction")]
    #[test_case("alter group some_group add user public")]
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

        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        let actual = parser.stmt(true);

        // This only quickly tests that statement types aren't missing.
        // More in-depth testing is within each statement's module.
        assert_matches!(actual, Ok(_),
            r"expected Ok(Some(_)) for {source:?} but actually got {actual:?}"
        )
    }
}

use crate::{
    lexer::{Keyword, TokenKind},
    parser::{
        ast_node::RawStmt::{self, ClosePortalStmt, DeallocateStmt, ListenStmt, LoadStmt, UnlistenStmt},
        consume_macro::consume,
        error::syntax_err,
        result::EofErrorKind::{self, NotEof},
        ParseResult,
        Parser
    }
};
use postgres_basics::fn_info;
