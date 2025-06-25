/// Alias: `NotifyStmt`
pub(super) fn notify_stmt() -> impl Combinator<Output = NotifyStmt> {

    /*
        NOTIFY ColId ( ',' SCONST )?
    */

    (
        Notify.skip(),
        col_id,
        Comma
            .and_right(string)
            .optional()
    ).map(|(_, condition_name, payload)| {
        if let Some(payload) = payload {
            NotifyStmt::with_payload(condition_name, payload)
        }
        else {
            NotifyStmt::new(condition_name)
        }
    })

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_notify() {
        let mut stream = TokenStream::new("notify test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(NotifyStmt::new("test_ident")), notify_stmt().parse(&mut stream));
    }

    #[test]
    fn test_notify_with_payload() {
        let mut stream = TokenStream::new("notify test_ident, 'test-payload'", DEFAULT_CONFIG);
        let expected = NotifyStmt::with_payload("test_ident", "test-payload");
        assert_eq!(Ok(expected), notify_stmt().parse(&mut stream));
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use pg_ast::NotifyStmt;
use pg_lexer::Keyword::Notify;
use pg_lexer::OperatorKind::Comma;
