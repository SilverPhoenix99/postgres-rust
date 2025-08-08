enum Change {
    Owner(RoleSpec),
    Name(Str),
}

pub(super) fn alter_language_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        ALTER (PROCEDURAL)? LANGUAGE ColId OWNER TO RoleSpec # AlterOwnerStmt
        ALTER (PROCEDURAL)? LANGUAGE ColId RENAME TO ColId # RenameStmt
    */

    let (_, language, stmt) = seq!(
        alt!(
            seq!(Procedural, Language).skip(),
            Language.skip()
        ),
        col_id,
        alt!(
            seq!(Owner, To, role_spec)
                .map(|(.., new_owner)| Change::Owner(new_owner)),
            seq!(Rename, To, col_id)
                .map(|(.., new_name)| Change::Name(new_name))
        )
    ).parse(stream)?;

    let stmt = match stmt {
        Change::Owner(new_owner) => {
            AlterOwnerStmt::new(
                AlterOwnerTarget::Language(language),
                new_owner
            ).into()
        },
        Change::Name(new_name) => {
            RenameStmt::new(
                RenameTarget::Language(language),
                new_name
            ).into()
        },
    };

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    #[allow(unused_imports)]
    use pg_sink_ast::RoleSpec::Public;
    use test_case::test_case;

    #[test_case(
        "procedural language some_language owner to public",
        AlterOwnerStmt::new(
            AlterOwnerTarget::Language("some_language".into()),
            Public
        ).into()
    )]
    #[test_case(
        "language some_language rename to new_lang",
        RenameStmt::new(
            RenameTarget::Language("some_language".into()),
            "new_lang"
        ).into()
    )]
    fn test_alter_language_stmt(source: &str, expected: RawStmt) {
        test_parser!(source, alter_language_stmt, expected);
    }
}

use pg_ast::AlterOwnerStmt;
use pg_ast::AlterOwnerTarget;
use pg_ast::RawStmt;
use pg_ast::RenameStmt;
use pg_ast::RenameTarget;
use pg_basics::Str;
use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Language;
use pg_lexer::Keyword::Owner;
use pg_lexer::Keyword::Procedural;
use pg_lexer::Keyword::Rename;
use pg_lexer::Keyword::To;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
use pg_sink_ast::RoleSpec;
use pg_sink_combinators::col_id;
use pg_sink_combinators::role_spec;
