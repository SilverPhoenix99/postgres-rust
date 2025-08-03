#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonTablePathSpec {
    path_spec: Box<str>,
    name: Option<Str>,
}

impl JsonTablePathSpec {
    pub fn new<T>(path_spec: T) -> Self
    where
        T: Into<Box<str>>
    {
        Self {
            path_spec: path_spec.into(),
            name: None
        }
    }

    pub fn path_spec(&self) -> &str {
        &self.path_spec
    }

    pub fn set_name(&mut self, name: Option<Str>) -> &mut Self {
        self.name = name;
        self
    }

    pub fn with_name<T>(mut self, name: T) -> Self
    where
        T: Into<Str>
    {
        self.name = Some(name.into());
        self
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

use pg_basics::Str;
