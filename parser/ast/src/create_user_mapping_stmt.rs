#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CreateUserMappingStmt {
    user: RoleSpec,
    server: Str,
    options: Option<Vec<GenericOption>>,
    existence: Presence
}

impl CreateUserMappingStmt {
    pub fn new<T: Into<Str>>(user: RoleSpec, server: T) -> Self {
        Self {
            user,
            server: server.into(),
            options: None,
            existence: Default::default()
        }
    }

    pub fn user(&self) -> &RoleSpec {
        &self.user
    }

    pub fn server(&self) -> &str {
        &self.server
    }
    
    pub fn set_options(&mut self, options: Option<Vec<GenericOption>>) -> &mut Self {

        self.options = options.and_then(|options|
            if options.is_empty() { None }
            else { Some(options) }
        );
        
        self
    }
    
    pub fn with_options(mut self, options: Vec<GenericOption>) -> Self {
        self.options = if options.is_empty() { None } else { Some(options) };
        self
    }

    pub fn options(&self) -> Option<&[GenericOption]> {
        self.options.as_deref()
    }

    pub fn set_existence(&mut self, existence: Presence) -> &mut Self {
        self.existence = existence;
        self
    }
    
    pub fn with_existence(mut self, existence: Presence) -> Self {
        self.existence = existence;
        self
    }
    
    pub fn existence(&self) -> Presence {
        self.existence
    }
}

use crate::GenericOption;
use crate::Presence;
use crate::RoleSpec;
use pg_basics::Str;
