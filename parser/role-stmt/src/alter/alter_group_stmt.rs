enum Change {
    Rename(Str),
    Role {
        action: AddDrop,
        members: Vec<RoleSpec>,
    }
}

/// Alias: `AlterGroupStmt`
pub fn alter_group_stmt(ctx: &mut ParserContext) -> scan::Result<RoleStmt> {

    /*
        ALTER GROUP role_id RENAME TO role_id
        ALTER GROUP role_spec (ADD | DROP) USER role_list
    */

    let (_, Located(group, loc), stmt) = seq!(
        Group,
        located!(role_spec),
        alt!(rename, change_role)
    ).parse(ctx)?;

    let stmt = match stmt {

        Change::Rename(new_name) => {

            let role_name = group.into_role_id()
                .map_err(|err| err.at_location(loc))?;

            RoleStmt::Rename { role_name, new_name }
        },

        Change::Role { action, members } => {

            let options = Some(vec![RoleMembers { action, members }]);

            AlterRoleStmt::new(group, options).into()
        }
    };

    Ok(stmt)
}

fn rename(ctx: &mut ParserContext) -> scan::Result<Change> {

    let (.., new_name) = seq!(Rename, To, role_id).parse(ctx)?;
    Ok(Change::Rename(new_name))
}

fn change_role(ctx: &mut ParserContext) -> scan::Result<Change> {

    let (action, _, members) = seq!(
        add_drop,
        User,
        role_list
    ).parse(ctx)?;

    Ok(Change::Role { action, members })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("group some_group rename to new_group_name" => Ok(
        RoleStmt::Rename {
            role_name: "some_group".into(),
            new_name: "new_group_name".into()
        }
    ))]
    #[test_case("group some_group add user current_role, new_user" => Ok(
        AlterRoleStmt::new(
            RoleSpec::Name("some_group".into()),
            Some(vec![RoleMembers {
                action: AddDrop::Add,
                members: vec![
                    RoleSpec::CurrentRole,
                    RoleSpec::Name("new_user".into())
                ]
            }])
        ).into()
    ))]
    #[test_case("group some_group drop user session_user, public" => Ok(
        AlterRoleStmt::new(
            RoleSpec::Name("some_group".into()),
            Some(vec![RoleMembers {
                action: AddDrop::Drop,
                members: vec![
                    RoleSpec::SessionUser,
                    RoleSpec::Public
                ]
            }])
        ).into()
    ))]
    fn test_alter_group_stmt(source: &str) -> scan::Result<RoleStmt> {
        test_parser!(source, alter_group_stmt)
    }
}

use pg_basics::IntoLocated;
use pg_basics::Located;
use pg_basics::Str;
use pg_combinators::alt;
use pg_combinators::located;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Group;
use pg_lexer::Keyword::Rename;
use pg_lexer::Keyword::To;
use pg_lexer::Keyword::User;
use pg_parser_core::scan;
use pg_role_ast::AlterRoleOption::RoleMembers;
use pg_role_ast::AlterRoleStmt;
use pg_role_ast::RoleStmt;
use pg_sink_ast::AddDrop;
use pg_sink_ast::RoleSpec;
use pg_sink_combinators::add_drop;
use pg_sink_combinators::role_id;
use pg_sink_combinators::role_list;
use pg_sink_combinators::role_spec;
use pg_sink_combinators::IntoRoleId;
