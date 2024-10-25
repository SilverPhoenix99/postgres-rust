impl Parser<'_> {

    pub(super) fn stmt(&mut self, allow_tx_legacy_stmts: bool) -> ParseResult<RawStmt> {
        use TokenKind::Keyword as Kw;
        use Keyword::*;

        consume! {self
            ok {
                Ok(Kw(Begin)) if allow_tx_legacy_stmts => self.begin_stmt().map(From::from),
                Ok(Kw(End)) if allow_tx_legacy_stmts => self.end_stmt().map(From::from),
                Ok(Kw(Checkpoint)) => Ok(RawStmt::CheckPoint),
                Ok(Kw(Abort)) => self.abort_stmt().map(From::from),
                Ok(Kw(Alter)) => self.alter_stmt(),
                Ok(Kw(Analyse | Analyze)) => self.analyze_stmt(),
                Ok(Kw(Call)) => self.call_stmt(),
                Ok(Kw(Close)) => self.close_stmt().map(ClosePortalStmt),
                Ok(Kw(Cluster)) => self.cluster_stmt(),
                Ok(Kw(Comment)) => self.comment_stmt(),
                Ok(Kw(Commit)) => self.commit_stmt().map(From::from),
                Ok(Kw(CopyKw)) => self.copy_stmt(),
                Ok(Kw(Deallocate)) => self.deallocate_stmt().map(DeallocateStmt),
                Ok(Kw(Discard)) => self.discard_stmt().map(From::from),
                Ok(Kw(Do)) => self.do_stmt(),
                Ok(Kw(DropKw)) => self.drop_stmt(),
                Ok(Kw(Explain)) => self.explain_stmt(),
                Ok(Kw(Fetch)) => self.fetch_stmt(),
                Ok(Kw(Import)) => self.import_stmt(),
                Ok(Kw(Listen)) => self.listen_stmt().map(ListenStmt),
                Ok(Kw(Load)) => self.load_stmt().map(LoadStmt),
                Ok(Kw(Lock)) => self.lock_stmt(),
                Ok(Kw(Move)) => self.move_stmt(),
                Ok(Kw(Notify)) => self.notify_stmt().map(From::from),
                Ok(Kw(Prepare)) => self.prepare_stmt(),
                Ok(Kw(Reassign)) => self.reassign_owned_stmt().map(From::from),
                Ok(Kw(Reindex)) => self.reindex_stmt(),
                Ok(Kw(Release)) => self.release_savepoint_stmt().map(From::from),
                Ok(Kw(Revoke)) => self.revoke_stmt(),
                Ok(Kw(Rollback)) => self.rollback_stmt().map(From::from),
                Ok(Kw(Savepoint)) => self.savepoint_stmt().map(From::from),
                Ok(Kw(Security)) => self.security_stmt(),
                Ok(Kw(Set)) => self.set_stmt(),
                Ok(Kw(Show)) => self.show_stmt().map(From::from),
                Ok(Kw(Start)) => self.start_transaction_stmt().map(From::from),
                Ok(Kw(Truncate)) => self.truncate_stmt(),
                Ok(Kw(Unlisten)) => self.unlisten_stmt().map(UnlistenStmt),
                Ok(Kw(Vacuum)) => self.vacuum_stmt(),
            }
            err {
                Ok(_) | Err(EofErrorKind::Eof) => Err(Default::default()),
                Err(NotEof(err)) => Err(err),
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
        result::EofErrorKind::{self, NotEof},
        ParseResult,
        Parser
    }
};
