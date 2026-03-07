#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CreateRoleStmt {
    name: Str,
    kind: RoleKind,
    options: Option<Vec<CreateRoleOption>>
}

impl CreateRoleStmt {
    pub fn new<T: Into<Str>>(name: T, kind: RoleKind) -> Self {
        Self {
            name: name.into(),
            kind,
            options: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> RoleKind {
        self.kind
    }

    pub fn set_options(&mut self, options: Option<Vec<CreateRoleOption>>) -> &mut Self {

        let options = options.and_then(|options|
            if options.is_empty() { None }
            else { Some(options) }
        );

        self.options = options;
        self
    }

    pub fn with_options(mut self, options: Vec<CreateRoleOption>) -> Self {
        self.options = if options.is_empty() { None } else { Some(options) };
        self
    }

    pub fn options(&self) -> Option<&[CreateRoleOption]> {
        self.options.as_deref()
    }
}

use crate::CreateRoleOption;
use crate::RoleKind;
use pg_basics::Str;
