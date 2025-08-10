/// Alias: `SetResetClause`
pub(super) fn set_reset_clause(ctx: &mut ParserContext) -> scan::Result<SetResetClause> {

    alt!(
        seq!(Set, set_rest)
            .map(|(_, option)| SetResetClause::Set(option)),
        reset_stmt.map(SetResetClause::Reset)
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_ast::SetRest;
    use pg_ast::VariableTarget;
    use pg_combinators::test_parser;

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

use crate::combinators::stmt::reset_stmt::reset_stmt;
use crate::combinators::stmt::set_rest;
use pg_ast::SetResetClause;
use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Set;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
