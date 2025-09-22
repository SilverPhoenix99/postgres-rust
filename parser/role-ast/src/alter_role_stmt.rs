#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterRoleStmt {
    role: RoleSpec,
    options: Option<Vec<AlterRoleOption>>,
}

impl AlterRoleStmt {

    pub fn new(role: RoleSpec, options: Option<Vec<AlterRoleOption>>) -> Self {
        Self { role, options }
    }

    pub fn role(&self) -> &RoleSpec {
        &self.role
    }

    pub fn set_options(&mut self, options: Option<Vec<AlterRoleOption>>) -> &mut Self {

        self.options = options.and_then(|options|
            if options.is_empty() { None }
            else { Some(options) }
        );

        self
    }

    pub fn with_options(mut self, options: Vec<AlterRoleOption>) -> Self {
        self.options = if options.is_empty() { None } else { Some(options) };
        self
    }

    pub fn options(&self) -> Option<&[AlterRoleOption]> {
        self.options.as_deref()
    }
}

use crate::AlterRoleOption;
use pg_sink_ast::RoleSpec;
