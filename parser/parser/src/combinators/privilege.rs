pub(super) fn privileges(stream: &mut TokenStream) -> scan::Result<AccessPrivilege> {

    /*
          ALL ( PRIVILEGES )? opt_column_list
        | privilege_list
    */

    choice!(stream =>
        seq!(stream =>
            AllKw.and(Privileges.optional()),
            paren_name_list.optional()
        )
            .map(|(_, columns)| All { columns }),
        privilege_list
            .parse(stream)
            .map(Specific)
    )
}

pub(super) fn privilege_list(stream: &mut TokenStream) -> scan::Result<Vec<SpecificAccessPrivilege>> {

    /*
        privilege ( ',' privilege )*
    */

    many!(stream => sep = Comma, privilege)
}

fn privilege(stream: &mut TokenStream) -> scan::Result<SpecificAccessPrivilege> {

    /*
          ALTER SYSTEM
        | SELECT opt_column_list
        | REFERENCES opt_column_list
        | CREATE opt_column_list
        | col_id opt_column_list
    */

    choice!(stream =>
        seq!(stream => Alter, SystemKw)
            .map(|_| AlterSystem),
        seq!(stream => CreateKw, paren_name_list.optional())
            .map(|(_, columns)| Create { columns }),
        seq!(stream => ReferencesKw, paren_name_list.optional())
            .map(|(_, columns)| References { columns }),
        seq!(stream => SelectKw, paren_name_list.optional())
            .map(|(_, columns)| Select { columns }),
        seq!(stream => col_id, paren_name_list.optional())
            .map(|(privilege, columns)| Named { privilege, columns })
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("all", All { columns: None })]
    #[test_case("all privileges", All { columns: None })]
    #[test_case("all (column_name)", All { columns: Some(vec!["column_name".into()]) })]
    #[test_case("select, references", Specific(vec![Select { columns: None }, References { columns: None }]))]
    fn test_privileges(source: &str, expected: AccessPrivilege) {
        test_parser!(source, privileges, expected)
    }

    #[test]
    fn test_privilege_list() {
        test_parser!(
            source = "alter system, select, create, some_privilege",
            parser = privilege_list,
            expected = vec![
                AlterSystem,
                Select { columns: None },
                Create { columns: None },
                Named{ privilege: "some_privilege".into(), columns: None },
            ]
        )
    }

    #[test_case("alter system", AlterSystem)]
    #[test_case("select", Select { columns: None })]
    #[test_case("select(column_name)", Select { columns: Some(vec!["column_name".into()]) })]
    #[test_case("references", References { columns: None })]
    #[test_case("references(column_name)", References { columns: Some(vec!["column_name".into()]) })]
    #[test_case("create", Create { columns: None })]
    #[test_case("create(column_name)", Create { columns: Some(vec!["column_name".into()]) })]
    #[test_case("some_name", Named { privilege: "some_name".into(), columns: None })]
    #[test_case("another_name(column_name)",
        Named {
            privilege: "another_name".into(),
            columns: Some(vec!["column_name".into()])
        }
    )]
    fn test_privilege(source: &str, expected: SpecificAccessPrivilege) {
        test_parser!(source, privilege, expected)
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::choice;
use crate::combinators::foundation::many;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::paren_name_list;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::AccessPrivilege;
use pg_ast::AccessPrivilege::All;
use pg_ast::AccessPrivilege::Specific;
use pg_ast::SpecificAccessPrivilege;
use pg_ast::SpecificAccessPrivilege::AlterSystem;
use pg_ast::SpecificAccessPrivilege::Create;
use pg_ast::SpecificAccessPrivilege::Named;
use pg_ast::SpecificAccessPrivilege::References;
use pg_ast::SpecificAccessPrivilege::Select;
use pg_lexer::Keyword::All as AllKw;
use pg_lexer::Keyword::Alter;
use pg_lexer::Keyword::Create as CreateKw;
use pg_lexer::Keyword::Privileges;
use pg_lexer::Keyword::References as ReferencesKw;
use pg_lexer::Keyword::Select as SelectKw;
use pg_lexer::Keyword::SystemKw;
use pg_lexer::OperatorKind::Comma;
