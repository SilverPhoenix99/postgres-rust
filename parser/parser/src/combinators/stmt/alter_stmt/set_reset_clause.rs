/// Alias: `SetResetClause`
pub(super) fn set_reset_clause(stream: &mut TokenStream) -> scan::Result<SetResetClause> {

    alt!(
        seq!(Set, set_rest)
            .map(|(_, option)| SetResetClause::Set(option)),
        reset_stmt.map(SetResetClause::Reset)
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::SetRest;
    use pg_ast::VariableTarget;

    #[test]
    fn test_set() {
        test_parser!(
            source = "set schema 'schema-name'",
            parser = set_reset_clause,
            expected = SetResetClause::Set(SetRest::Schema("schema-name".into()))
        )
    }

    #[test]
    fn test_reset() {
        test_parser!(
            source = "reset time zone",
            parser = set_reset_clause,
            expected = SetResetClause::Reset(VariableTarget::TimeZone)
        )
    }
}

use crate::combinators::foundation::alt;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::stmt::reset_stmt::reset_stmt;
use crate::combinators::stmt::set_rest;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::SetResetClause;
use pg_lexer::Keyword::Set;
