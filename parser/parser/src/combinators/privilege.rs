pub(super) fn privileges(stream: &mut TokenStream) -> scan::Result<AccessPrivilege> {

    /*
          ALL ( PRIVILEGES )? ( '(' name_list ')' )?
        | privilege_list
    */

    alt!(
        seq!(
            AllKw,
            Privileges.optional(),
            paren!(name_list).optional()
        )
            .map(|(.., columns)| All { columns }),
        privilege_list
            .map(Specific)
    ).parse(stream)
}

pub(super) fn privilege_list(stream: &mut TokenStream) -> scan::Result<Vec<SpecificAccessPrivilege>> {

    /*
        privilege ( ',' privilege )*
    */

    many!(sep = Comma, privilege).parse(stream)
}

fn privilege(stream: &mut TokenStream) -> scan::Result<SpecificAccessPrivilege> {

    /*
          ALTER SYSTEM
        | SELECT ( '(' name_list ')' )?
        | REFERENCES ( '(' name_list ')' )?
        | CREATE ( '(' name_list ')' )?
        | col_id ( '(' name_list ')' )?
    */

    alt!(
        alter_system,
        create,
        references,
        select,
        named
    ).parse(stream)
}

fn alter_system(stream: &mut TokenStream) -> scan::Result<SpecificAccessPrivilege> {
    let _ = seq!(Alter, SystemKw).parse(stream)?;
    Ok(AlterSystem)
}

fn create(stream: &mut TokenStream) -> scan::Result<SpecificAccessPrivilege> {

    let (_, columns) = seq!(
        CreateKw,
        paren!(name_list).optional()
    ).parse(stream)?;

    Ok(Create { columns })
}

fn references(stream: &mut TokenStream) -> scan::Result<SpecificAccessPrivilege> {

    let (_, columns) = seq!(
        ReferencesKw,
        paren!(name_list).optional()
    ).parse(stream)?;

    Ok(References { columns })
}

fn select(stream: &mut TokenStream) -> scan::Result<SpecificAccessPrivilege> {

    let (_, columns) = seq!(
        SelectKw,
        paren!(name_list).optional()
    ).parse(stream)?;

    Ok(Select { columns })
}

fn named(stream: &mut TokenStream) -> scan::Result<SpecificAccessPrivilege> {

    let (privilege, columns) = seq!(
        col_id,
        paren!(name_list).optional()
    ).parse(stream)?;

    Ok(Named { privilege, columns })
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
use crate::combinators::foundation::alt;
use crate::combinators::foundation::many;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::name_list;
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
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
