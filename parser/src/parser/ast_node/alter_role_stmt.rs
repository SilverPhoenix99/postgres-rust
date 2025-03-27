#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AlterRoleAction {
    Add,
    Remove,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AlterRoleOption {
    RoleMembers(Vec<RoleSpec>),
    Password(Option<Box<str>>),
    Inherit(bool),
    ConnectionLimit(i32),
    ValidUntil(Box<str>),
    SuperUser(bool),
    CreateRole(bool),
    Replication(bool),
    CreateDatabase(bool),
    CanLogin(bool),
    BypassRls(bool),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterRoleStmt {
    role: RoleSpec,
    action: AlterRoleAction,
    options: Vec<AlterRoleOption>,
}

impl AlterRoleStmt {
    #[inline(always)]
    pub fn new(role: RoleSpec, action: AlterRoleAction, options: Vec<AlterRoleOption>) -> Self {
        Self { role, action, options }
    }

    #[inline(always)]
    pub fn role(&self) -> &RoleSpec {
        &self.role
    }

    #[inline(always)]
    pub fn action(&self) -> AlterRoleAction {
        self.action
    }

    #[inline(always)]
    pub fn options(&self) -> &[AlterRoleOption] {
        &self.options
    }

    #[inline(always)]
    pub fn add(&self) -> bool {
        self.action == AlterRoleAction::Add
    }

    #[inline(always)]
    pub fn remove(&self) -> bool {
        self.action == AlterRoleAction::Remove
    }
}

use crate::parser::ast_node::RoleSpec;
