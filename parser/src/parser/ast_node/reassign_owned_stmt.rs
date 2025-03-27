#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReassignOwnedStmt {
    roles: Vec<RoleSpec>,
    new_role: RoleSpec,
}

impl ReassignOwnedStmt {
    #[inline(always)]
    pub fn new(roles: Vec<RoleSpec>, new_role: RoleSpec) -> Self {
        Self { roles, new_role }
    }

    #[inline(always)]
    pub fn roles(&self) -> &[RoleSpec] {
        &self.roles
    }

    #[inline(always)]
    pub fn new_role(&self) -> &RoleSpec {
        &self.new_role
    }
}

use super::RoleSpec;
