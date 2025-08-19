#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterRoleStmt {
    role: RoleSpec,
    action: AddDrop,
    options: Option<Vec<AlterRoleOption>>,
}

impl AlterRoleStmt {

    pub fn new(role: RoleSpec, action: AddDrop, options: Option<Vec<AlterRoleOption>>) -> Self {
        Self { role, action, options }
    }

    pub fn role(&self) -> &RoleSpec {
        &self.role
    }

    pub fn action(&self) -> AddDrop {
        self.action
    }

    pub fn options(&self) -> Option<&[AlterRoleOption]> {
        self.options.as_deref()
    }

    pub fn add(&self) -> bool {
        self.action == AddDrop::Add
    }

    pub fn remove(&self) -> bool {
        self.action == AddDrop::Drop
    }
}

use pg_role_ast::AlterRoleOption;
use pg_sink_ast::AddDrop;
use pg_sink_ast::RoleSpec;
