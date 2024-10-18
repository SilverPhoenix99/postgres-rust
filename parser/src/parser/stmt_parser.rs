impl Parser<'_> {

    pub(super) fn stmt(&mut self, allow_tx_legacy_stmts: bool) -> ParseResult<RawStmt> {
        use TokenKind::Keyword as Kw;
        use Keyword::*;

        consume!{self default,
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_stmt() {
        let sources = [
            // TODO: analyze, call, cluster, comment, copy, do, drop, explain, fetch, import, lock, move,
            //       reindex, revoke, security, set, truncate, vacuum
            "abort transaction",
            "alter group some_group add user public",
            "close all",
            "commit and no chain",
            "deallocate all",
            "discard all",
            "listen ident",
            "load 'test string'",
            "notify test_ident, 'test-payload'",
            "prepare transaction 'tx id'",
            "reassign owned by public, test_role to target_role",
            "release savepoint test_ident",
            "rollback to test_ident",
            "savepoint test_ident",
            "show all",
            "start transaction read only, read write deferrable",
            "unlisten *",
        ];

        for source in sources {
            let mut parser = Parser::new(source, DEFAULT_CONFIG);
            let actual = parser.stmt(true);

            // This only quickly tests that statement types aren't missing.
            // More in-depth testing is within each statement's module.
            assert_matches!(actual, Ok(_),
                r"expected Ok(Some(_)) for {source:?} but actually got {actual:?}"
            )
        }
    }
}

use crate::lexer::{Keyword, TokenKind};
use crate::parser::ast_node::RawStmt;
use crate::parser::ast_node::RawStmt::{ClosePortalStmt, DeallocateStmt, ListenStmt, LoadStmt, UnlistenStmt};
use crate::parser::consume_macro::consume;
use crate::parser::{ParseResult, Parser};
