/// Alias: `NotifyStmt`
pub(in crate::parser) fn notify_stmt() -> impl Combinator<Output = NotifyStmt> {

    /*
        NOTIFY ColId ( ',' SCONST )?
    */

    sequence!(
        Notify.skip(),
        col_id(),
        Comma
            .and_right(string())
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
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_notify() {
        let mut stream = TokenStream::new("notify test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(NotifyStmt::new("test_ident".into())), notify_stmt().parse(&mut stream));
    }

    #[test]
    fn test_notify_with_payload() {
        let mut stream = TokenStream::new("notify test_ident, 'test-payload'", DEFAULT_CONFIG);
        let expected = NotifyStmt::with_payload("test_ident".into(), "test-payload".into());
        assert_eq!(Ok(expected), notify_stmt().parse(&mut stream));
    }
}

use crate::lexer::Keyword::Notify;
use crate::lexer::OperatorKind::Comma;
use crate::parser::ast_node::NotifyStmt;
use crate::parser::col_id;
use crate::parser::combinators::sequence;
use crate::parser::combinators::string;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
