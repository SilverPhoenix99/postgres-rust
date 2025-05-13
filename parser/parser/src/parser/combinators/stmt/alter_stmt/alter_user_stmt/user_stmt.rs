pub(super) fn user_stmt() -> impl Combinator<Output = RawStmt> {

    /*
          ALL ( in_database )? SetResetClause   => AlterRoleSetStmt
        | RoleId RENAME TO RoleId               => RenameStmt
        | RoleSpec in_database SetResetClause   => AlterRoleSetStmt
        | RoleSpec WITH AlterOptRoleList        => AlterRoleStmt
        | RoleSpec SetResetClause               => AlterRoleSetStmt
        | RoleSpec AlterOptRoleList             => AlterRoleStmt
    */

    match_first! {
        sequence!(All.skip(), in_database().optional(), set_reset_clause())
            .map(|(_, dbname, set_stmt)|
                AlterRoleSetStmt::new(OneOrAll::All, dbname, set_stmt).into()
            ),
        located(role_spec()).chain(match_first_with_state!(|(role, loc), stream| {
            {
                Rename.and(To)
                    .and_right(role_id())
            } => (new_name) {
                let role_id = role.into_role_id()
                    .map_err(|err|
                        ScanErr(ParserError::new(err, loc))
                    )?;
                RenameStmt::new(Role(role_id), new_name).into()
            },
            {
                sequence!(in_database(), set_reset_clause())
            } => ((dbname, set_stmt)) {
                AlterRoleSetStmt::new(OneOrAll::One(role), Some(dbname), set_stmt).into()
            },
            {
                With.and_right(alter_role_options())
            } => (options) {
                AlterRoleStmt::new(role, Add, options).into()
            },
            {
                set_reset_clause()
            } => (set_stmt) {
                AlterRoleSetStmt::new(OneOrAll::One(role), None, set_stmt).into()
            },
            {
                alter_role_options()
            } => (options) {
                AlterRoleStmt::new(role, Add, options).into()
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use postgres_parser_ast::AlterRoleOption;
    use postgres_parser_ast::RoleSpec;
    use postgres_parser_ast::SetResetClause::Reset;
    use postgres_parser_ast::SetResetClause::Set;
    use postgres_parser_ast::SetRest::LocalTransactionCharacteristics;
    use postgres_parser_ast::SetRest::TransactionSnapshot;
    use postgres_parser_ast::TransactionMode::Deferrable;
    use postgres_parser_ast::VariableTarget::SessionAuthorization;
    use postgres_parser_ast::VariableTarget::TimeZone;

    #[test]
    fn test_all_in_db_set() {
        let source = "all in database foo set transaction snapshot 'bar'";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = user_stmt().parse(&mut stream);

        let expected = AlterRoleSetStmt::new(
            OneOrAll::All,
            Some("foo".into()),
            Set(TransactionSnapshot("bar".into()))
        );

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_all_set() {
        let source = "all set transaction deferrable";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = user_stmt().parse(&mut stream);

        let expected = AlterRoleSetStmt::new(
            OneOrAll::All,
            None,
            Set(LocalTransactionCharacteristics(vec![Deferrable]))
        );

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_rename() {
        let source = "this_user rename to that_role";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = user_stmt().parse(&mut stream);

        let expected = RenameStmt::new(
            Role("this_user".into()),
            "that_role"
        );

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_role_in_db() {
        let source = "current_user in database test_db reset session authorization";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = user_stmt().parse(&mut stream);

        let expected = AlterRoleSetStmt::new(
            OneOrAll::One(RoleSpec::CurrentUser),
            Some("test_db".into()),
            Reset(SessionAuthorization)
        );

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_role_set_reset_clause() {
        let source = "public reset time zone";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = user_stmt().parse(&mut stream);

        let expected = AlterRoleSetStmt::new(
            OneOrAll::One(RoleSpec::Public),
            None,
            Reset(TimeZone)
        );

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_role_with_alter_op() {
        let source = "public encrypted password 'abc123'";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = user_stmt().parse(&mut stream);

        let expected = AlterRoleStmt::new(
            RoleSpec::Public,
            Add,
            vec![AlterRoleOption::Password(Some("abc123".into()))],
        );

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_role_alter_op() {
        let source = "public with noinherit";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = user_stmt().parse(&mut stream);

        let expected = AlterRoleStmt::new(
            RoleSpec::Public,
            Add,
            vec![AlterRoleOption::Inherit(false)],
        );

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_role_no_options() {
        let source = "public";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = user_stmt().parse(&mut stream);

        let expected = AlterRoleStmt::new(
            RoleSpec::Public,
            Add,
            vec![]
        );

        assert_eq!(Ok(expected.into()), actual);
    }
}

use super::in_database::in_database;
use crate::parser::combinators::foundation::located;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::match_first_with_state;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::role_id;
use crate::parser::combinators::role_spec;
use crate::parser::combinators::stmt::alter_role_options;
use crate::parser::combinators::stmt::alter_stmt::set_reset_clause;
use crate::parser::ParserError;
use postgres_parser_ast::AddDrop::Add;
use postgres_parser_ast::AlterRoleSetStmt;
use postgres_parser_ast::AlterRoleStmt;
use postgres_parser_ast::OneOrAll;
use postgres_parser_ast::RawStmt;
use postgres_parser_ast::RenameStmt;
use postgres_parser_ast::RenameTarget::Role;
use postgres_parser_lexer::Keyword::All;
use postgres_parser_lexer::Keyword::Rename;
use postgres_parser_lexer::Keyword::To;
use postgres_parser_lexer::Keyword::With;
