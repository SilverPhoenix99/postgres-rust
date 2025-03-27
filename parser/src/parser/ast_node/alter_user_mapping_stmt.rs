#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterUserMappingStmt {
    user: RoleSpec,
    server_name: Str,
    options: Vec<GenericOptionKind>
}

impl AlterUserMappingStmt {
    pub fn new<T>(user: RoleSpec, server_name: T, options: Vec<GenericOptionKind>) -> Self
    where
        Str: From<T>
    {
        Self {
            user,
            server_name: server_name.into(),
            options
        }
    }

    pub fn user(&self) -> &RoleSpec {
        &self.user
    }

    pub fn server_name(&self) -> &str {
        &self.server_name
    }

    pub fn options(&self) -> &[GenericOptionKind] {
        &self.options
    }
}

use crate::parser::ast_node::GenericOptionKind;
use crate::parser::ast_node::RoleSpec;
use postgres_basics::Str;
