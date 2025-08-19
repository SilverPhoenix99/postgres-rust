/// Alias: `AlterOptRoleElem`
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AlterRoleOption {
    RoleMembers(Vec<RoleSpec>),
    Password(Option<Box<str>>),
    Inherit(bool),
    ConnectionLimit(i32),
    ValidUntil(Box<str>),
    SuperUser(bool),
    CreateRole(bool),
    IsReplication(bool),
    CreateDatabase(bool),
    CanLogin(bool),
    BypassRls(bool),
}

use pg_sink_ast::RoleSpec;
