pub(super) fn privileges() -> impl Combinator<Output = AccessPrivilege> {

    /*
          ALL ( PRIVILEGES )? opt_column_list
        | privilege_list
    */

    match_first!(
        sequence!(
            AllKw.and(Privileges.optional()).skip(),
            paren_name_list().optional()
        )
            .map(|(_, columns)| All(columns)),
        privilege_list().map(Specific)
    )
}

/// Post-condition: Vec is **Not** empty
pub(super) fn privilege_list() -> impl Combinator<Output = Vec<SpecificAccessPrivilege>> {

    /*
        privilege ( ',' privilege )*
    */

    many_sep(Comma, privilege())
}

fn privilege() -> impl Combinator<Output = SpecificAccessPrivilege> {

    /*
          ALTER SYSTEM
        | SELECT opt_column_list
        | REFERENCES opt_column_list
        | CREATE opt_column_list
        | col_id opt_column_list
    */

    match_first! {
        Alter.and(SystemKw).map(|_| AlterSystem),
        CreateKw
            .and_then(paren_name_list().optional(), |_, columns| Create(columns)),
        ReferencesKw
            .and_then(paren_name_list().optional(), |_, columns| References(columns)),
        SelectKw
            .and_then(paren_name_list().optional(), |_, columns| Select(columns)),
        col_id()
            .and_then(paren_name_list().optional(), Named)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("all", All(None))]
    #[test_case("all privileges", All(None))]
    #[test_case("all (column_name)", All(Some(vec!["column_name".into()])))]
    #[test_case("select, references", Specific(vec![Select(None), References(None)]))]
    fn test_privileges(source: &str, expected: AccessPrivilege) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(expected), privileges().parse(&mut stream));
    }

    #[test]
    fn test_privilege_list() {
        let source = "alter system, select, create, some_privilege";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = vec![
            AlterSystem,
            Select(None),
            Create(None),
            Named("some_privilege".into(), None),
        ];

        assert_eq!(Ok(expected), privilege_list().parse(&mut stream));
    }

    #[test_case("alter system", AlterSystem)]
    #[test_case("select", Select(None))]
    #[test_case("select(column_name)", Select(Some(vec!["column_name".into()])))]
    #[test_case("references", References(None))]
    #[test_case("references(column_name)", References(Some(vec!["column_name".into()])))]
    #[test_case("create", Create(None))]
    #[test_case("create(column_name)", Create(Some(vec!["column_name".into()])))]
    #[test_case("some_name", Named("some_name".into(), None))]
    #[test_case("another_name(column_name)", Named("another_name".into(), Some(vec!["column_name".into()])))]
    fn test_privilege(source: &str, expected: SpecificAccessPrivilege) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(expected), privilege().parse(&mut stream));
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::sequence;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::paren_name_list;
use postgres_parser_ast::AccessPrivilege;
use postgres_parser_ast::AccessPrivilege::All;
use postgres_parser_ast::AccessPrivilege::Specific;
use postgres_parser_ast::SpecificAccessPrivilege;
use postgres_parser_ast::SpecificAccessPrivilege::AlterSystem;
use postgres_parser_ast::SpecificAccessPrivilege::Create;
use postgres_parser_ast::SpecificAccessPrivilege::Named;
use postgres_parser_ast::SpecificAccessPrivilege::References;
use postgres_parser_ast::SpecificAccessPrivilege::Select;
use postgres_parser_lexer::Keyword::All as AllKw;
use postgres_parser_lexer::Keyword::Alter;
use postgres_parser_lexer::Keyword::Create as CreateKw;
use postgres_parser_lexer::Keyword::Privileges;
use postgres_parser_lexer::Keyword::References as ReferencesKw;
use postgres_parser_lexer::Keyword::Select as SelectKw;
use postgres_parser_lexer::Keyword::SystemKw;
use postgres_parser_lexer::OperatorKind::Comma;
