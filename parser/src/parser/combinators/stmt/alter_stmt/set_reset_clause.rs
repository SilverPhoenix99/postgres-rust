/// Alias: `SetResetClause`
pub(super) fn set_reset_clause() -> impl Combinator<Output = SetResetClause> {

    match_first! {
        Set.and_right(set_rest()).map(SetResetClause::Set),
        reset_stmt().map(SetResetClause::Reset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::{SetRest, VariableTarget};
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_set() {
        let source = "set schema 'schema-name'";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = set_reset_clause().parse(&mut stream);

        let expected = SetResetClause::Set(SetRest::Schema("schema-name".into()));

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_reset() {
        let source = "reset time zone";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = set_reset_clause().parse(&mut stream);

        let expected = SetResetClause::Reset(VariableTarget::TimeZone);

        assert_eq!(Ok(expected), actual);
    }
}

use crate::lexer::Keyword::Set;
use crate::parser::ast_node::SetResetClause;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::stmt::reset_stmt::reset_stmt;
use crate::parser::combinators::stmt::set_rest::set_rest;
