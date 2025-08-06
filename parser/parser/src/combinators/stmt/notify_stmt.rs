/// Alias: `NotifyStmt`
pub(super) fn notify_stmt(stream: &mut TokenStream) -> scan::Result<NotifyStmt> {

    /*
        NOTIFY ColId ( ',' SCONST )?
    */

    let (_, condition_name, payload) = seq!(
        Notify,
        col_id,
        seq!(Comma, string).optional()
    ).parse(stream)?;

    let stmt = if let Some((_, payload)) = payload {
        NotifyStmt::with_payload(condition_name, payload)
    }
    else {
        NotifyStmt::new(condition_name)
    };

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_notify() {
        test_parser!(
            source = "notify test_ident",
            parser = notify_stmt,
            expected = NotifyStmt::new("test_ident")
        )
    }

    #[test]
    fn test_notify_with_payload() {
        test_parser!(
            source = "notify test_ident, 'test-payload'",
            parser = notify_stmt,
            expected = NotifyStmt::with_payload("test_ident", "test-payload")
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use pg_ast::NotifyStmt;
use pg_lexer::Keyword::Notify;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
