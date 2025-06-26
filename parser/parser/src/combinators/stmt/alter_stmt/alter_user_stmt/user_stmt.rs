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
        (All.skip(), in_database().optional(), set_reset_clause())
            .map(|(_, dbname, set_stmt)|
                AlterRoleSetStmt::new(OneOrAll::All, dbname, set_stmt).into()
            ),
        located!(role_spec)
            .chain(match_first_with_state!(|(role, loc), stream| {
                {
                    Rename.and(To)
                        .and_right(role_id)
                } => (new_name) {
                    let role_id = role.into_role_id()
                        .map_err(|err| err.at(loc))?;
                    RenameStmt::new(Role(role_id), new_name).into()
                },
                {
                    (in_database(), set_reset_clause())
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
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use pg_ast::AlterRoleOption;
    use pg_ast::RoleSpec;
    use pg_ast::SetResetClause::Reset;
    use pg_ast::SetResetClause::Set;
    use pg_ast::SetRest::LocalTransactionCharacteristics;
    use pg_ast::SetRest::TransactionSnapshot;
    use pg_ast::TransactionMode::Deferrable;
    use pg_ast::VariableTarget::SessionAuthorization;
    use pg_ast::VariableTarget::TimeZone;

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
            Some(vec![AlterRoleOption::Password(Some("abc123".into()))]),
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
            Some(vec![AlterRoleOption::Inherit(false)]),
        );

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_role_no_options() {
        let source = "public";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = user_stmt().parse(&mut stream);

        let expected = AlterRoleStmt::new(RoleSpec::Public, Add, None);

        assert_eq!(Ok(expected.into()), actual);
    }
}

use super::in_database::in_database;
use crate::combinators::foundation::located;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::match_first_with_state;
use crate::combinators::foundation::Combinator;
use crate::combinators::role_id;
use crate::combinators::role_spec;
use crate::combinators::stmt::alter_role_options;
use crate::combinators::stmt::alter_stmt::set_reset_clause;
use pg_ast::AddDrop::Add;
use pg_ast::AlterRoleSetStmt;
use pg_ast::AlterRoleStmt;
use pg_ast::OneOrAll;
use pg_ast::RawStmt;
use pg_ast::RenameStmt;
use pg_ast::RenameTarget::Role;
use pg_lexer::Keyword::All;
use pg_lexer::Keyword::Rename;
use pg_lexer::Keyword::To;
use pg_lexer::Keyword::With;
