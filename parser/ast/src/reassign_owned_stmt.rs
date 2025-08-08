#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReassignOwnedStmt {
    roles: Vec<RoleSpec>,
    new_role: RoleSpec,
}

impl ReassignOwnedStmt {

    pub fn new(roles: Vec<RoleSpec>, new_role: RoleSpec) -> Self {
        Self { roles, new_role }
    }

    pub fn roles(&self) -> &[RoleSpec] {
        &self.roles
    }

    pub fn new_role(&self) -> &RoleSpec {
        &self.new_role
    }
}

use pg_sink_ast::RoleSpec;
