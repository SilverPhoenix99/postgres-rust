pub(super) fn privileges(ctx: &mut ParserContext) -> scan::Result<AccessPrivilege> {

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
    ).parse(ctx)
}

pub(super) fn privilege_list(ctx: &mut ParserContext) -> scan::Result<Vec<SpecificAccessPrivilege>> {

    /*
        privilege ( ',' privilege )*
    */

    many!(sep = Comma, privilege).parse(ctx)
}

fn privilege(ctx: &mut ParserContext) -> scan::Result<SpecificAccessPrivilege> {

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
    ).parse(ctx)
}

fn alter_system(ctx: &mut ParserContext) -> scan::Result<SpecificAccessPrivilege> {
    let _ = seq!(Alter, SystemKw).parse(ctx)?;
    Ok(AlterSystem)
}

fn create(ctx: &mut ParserContext) -> scan::Result<SpecificAccessPrivilege> {

    let (_, columns) = seq!(
        CreateKw,
        paren!(name_list).optional()
    ).parse(ctx)?;

    Ok(Create { columns })
}

fn references(ctx: &mut ParserContext) -> scan::Result<SpecificAccessPrivilege> {

    let (_, columns) = seq!(
        ReferencesKw,
        paren!(name_list).optional()
    ).parse(ctx)?;

    Ok(References { columns })
}

fn select(ctx: &mut ParserContext) -> scan::Result<SpecificAccessPrivilege> {

    let (_, columns) = seq!(
        SelectKw,
        paren!(name_list).optional()
    ).parse(ctx)?;

    Ok(Select { columns })
}

fn named(ctx: &mut ParserContext) -> scan::Result<SpecificAccessPrivilege> {

    let (privilege, columns) = seq!(
        col_id,
        paren!(name_list).optional()
    ).parse(ctx)?;

    Ok(Named { privilege, columns })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_matrix;

    #[test_matrix("all" => Ok(All { columns: None }))]
    #[test_matrix("all privileges" => Ok(All { columns: None }))]
    #[test_matrix("all (column_name)" => Ok(All { columns: Some(vec!["column_name".into()]) }))]
    #[test_matrix("select, references" => Ok(Specific(vec![Select { columns: None }, References { columns: None }])))]
    fn test_privileges(source: &str) -> scan::Result<AccessPrivilege> {
        test_parser!(source, privileges)
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

    #[test_matrix("alter system" => Ok(AlterSystem))]
    #[test_matrix("select" => Ok(Select { columns: None }))]
    #[test_matrix("select(column_name)" => Ok(Select { columns: Some(vec!["column_name".into()]) }))]
    #[test_matrix("references" => Ok(References { columns: None }))]
    #[test_matrix("references(column_name)" => Ok(References { columns: Some(vec!["column_name".into()]) }))]
    #[test_matrix("create" => Ok(Create { columns: None }))]
    #[test_matrix("create(column_name)" => Ok(Create { columns: Some(vec!["column_name".into()]) }))]
    #[test_matrix("some_name" => Ok(Named { privilege: "some_name".into(), columns: None }))]
    #[test_matrix("another_name(column_name)" => Ok(
        Named {
            privilege: "another_name".into(),
            columns: Some(vec!["column_name".into()])
        }
    ))]
    fn test_privilege(source: &str) -> scan::Result<SpecificAccessPrivilege> {
        test_parser!(source, privilege)
    }
}

use crate::alt;
use crate::combinators::col_id;
use crate::combinators::core::Combinator;
use crate::combinators::name_list;
use crate::many;
use crate::paren;
use crate::seq;
use crate::ParserContext;
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
