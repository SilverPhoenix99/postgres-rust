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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterRoleStmt {
    role: RoleSpec,
    action: AddDrop,
    options: Option<Vec<AlterRoleOption>>,
}

impl AlterRoleStmt {
    #[inline(always)]
    pub fn new(role: RoleSpec, action: AddDrop, options: Option<Vec<AlterRoleOption>>) -> Self {
        Self { role, action, options }
    }

    #[inline(always)]
    pub fn role(&self) -> &RoleSpec {
        &self.role
    }

    #[inline(always)]
    pub fn action(&self) -> AddDrop {
        self.action
    }

    #[inline(always)]
    pub fn options(&self) -> Option<&[AlterRoleOption]> {
        self.options.as_deref()
    }

    #[inline(always)]
    pub fn add(&self) -> bool {
        self.action == AddDrop::Add
    }

    #[inline(always)]
    pub fn remove(&self) -> bool {
        self.action == AddDrop::Drop
    }
}

use crate::AddDrop;
use crate::RoleSpec;
