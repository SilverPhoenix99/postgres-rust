#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterExtensionStmt {
    name: Str,
    options: Option<Vec<Str>>,
}

impl AlterExtensionStmt {
    pub fn new<T: Into<Str>>(name: T) -> Self {
        Self {
            name: name.into(),
            options: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_options(&mut self, options: Option<Vec<Str>>) -> &mut Self {

        self.options = options.and_then(|options|
            if options.is_empty() { None }
            else { Some(options) }
        );

        self
    }

    pub fn with_options(mut self, options: Vec<Str>) -> Self {
        self.options = if options.is_empty() { None } else { Some(options) };
        self
    }

    pub fn options(&self) -> Option<&[Str]> {
        self.options.as_deref()
    }
}

use pg_basics::Str;
