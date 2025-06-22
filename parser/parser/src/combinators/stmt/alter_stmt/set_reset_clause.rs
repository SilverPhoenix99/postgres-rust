/// Alias: `SetResetClause`
pub(super) fn set_reset_clause() -> impl Combinator<Output = SetResetClause> {

    match_first! {
        Set.and_right(set_rest).map(SetResetClause::Set),
        reset_stmt.map(SetResetClause::Reset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use pg_ast::SetRest;
    use pg_ast::VariableTarget;

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

use crate::combinators::foundation::match_first;
use crate::combinators::foundation::Combinator;
use crate::combinators::stmt::reset_stmt::reset_stmt;
use crate::combinators::stmt::set_rest;
use pg_ast::SetResetClause;
use pg_lexer::Keyword::Set;
