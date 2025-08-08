#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CreateRoleStmt {
    name: Str,
    kind: RoleKind,
    options: Vec<CreateRoleOption>
}

impl CreateRoleStmt {
    pub fn new<T: Into<Str>>(name: T, kind: RoleKind, options: Vec<CreateRoleOption>) -> Self {
        Self {
            name: name.into(),
            kind,
            options
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> RoleKind {
        self.kind
    }

    pub fn options(&self) -> &[CreateRoleOption] {
        &self.options
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RoleKind {
    Role,
    User,
    Group,
}

/// Alias: `CreateOptRoleElem`
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CreateRoleOption {
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
    SysId(NonNegative),
    AdminMembers(Vec<RoleSpec>),
    AddRoleTo(Vec<RoleSpec>),
}

impl From<AlterRoleOption> for CreateRoleOption {
    fn from(value: AlterRoleOption) -> Self {
        match value {
            AlterRoleOption::RoleMembers(members) => Self::RoleMembers(members),
            AlterRoleOption::Password(password) => Self::Password(password),
            AlterRoleOption::Inherit(inherit) => Self::Inherit(inherit),
            AlterRoleOption::ConnectionLimit(limit) => Self::ConnectionLimit(limit),
            AlterRoleOption::ValidUntil(ts) => Self::ValidUntil(ts),
            AlterRoleOption::SuperUser(is_super_user) => Self::SuperUser(is_super_user),
            AlterRoleOption::CreateRole(can_create_role) => Self::CreateRole(can_create_role),
            AlterRoleOption::IsReplication(is_replication) => Self::IsReplication(is_replication),
            AlterRoleOption::CreateDatabase(can_create_db) => Self::CreateDatabase(can_create_db),
            AlterRoleOption::CanLogin(can_login) => Self::CanLogin(can_login),
            AlterRoleOption::BypassRls(bypass_rls) => Self::BypassRls(bypass_rls),
        }
    }
}

use crate::AlterRoleOption;
use pg_basics::NonNegative;
use pg_basics::Str;
use pg_sink_ast::RoleSpec;
