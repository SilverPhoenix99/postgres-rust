#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturningClause {
    options: Option<Vec<ReturningOptionKind>>,
    exprs: Vec<OneOrAll<NamedValue>>,
}

impl ReturningClause {

    pub fn new(exprs: Vec<OneOrAll<NamedValue>>) -> Self {
        Self {
            options: None,
            exprs,
        }
    }

    pub fn exprs(&self) -> &[OneOrAll<NamedValue>] {
        &self.exprs
    }

    pub fn set_options(&mut self, options: Option<Vec<ReturningOptionKind>>) -> &mut Self {

        self.options = options.and_then(|options|
            if options.is_empty() { None }
            else { Some(options) }
        );

        self
    }

    pub fn with_options(mut self, options: Vec<ReturningOptionKind>) -> Self {
        self.options = if options.is_empty() { None } else { Some(options) };
        self
    }

    pub fn options(&self) -> Option<&[ReturningOptionKind]> {
        self.options.as_deref()
    }
}

use crate::NamedValue;
use crate::OneOrAll;
use crate::ReturningOptionKind;
