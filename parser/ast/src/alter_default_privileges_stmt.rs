#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterDefaultPrivilegesStmt {
    options: Vec<AclOption>,
    action: GrantStmt,
}

impl AlterDefaultPrivilegesStmt {
    pub fn new(options: Vec<AclOption>, action: GrantStmt) -> Self {
        Self { options, action }
    }

    pub fn options(&self) -> &[AclOption] {
        &self.options
    }

    pub fn action(&self) -> &GrantStmt {
        &self.action
    }
}

use crate::AclOption;
use crate::GrantStmt;
