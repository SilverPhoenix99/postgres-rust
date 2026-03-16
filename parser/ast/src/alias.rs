#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alias {
    alias: Str,
    columns: Option<Vec<Str>>
}

impl Alias {
    pub fn new<T>(alias: T) -> Self
    where
        T: Into<Str>,
    {
        Self {
            alias: alias.into(),
            columns: None,
        }
    }

    pub fn alias(&self) -> &Str {
        &self.alias
    }

    pub fn set_columns(&mut self, columns: Option<Vec<Str>>) -> &mut Self {

        self.columns = columns.and_then(|cols|
            if cols.is_empty() { None }
            else { Some(cols) }
        );

        self
    }

    pub fn with_columns(mut self, columns: Vec<Str>) -> Self {

        self.columns = if columns.is_empty() {
            None
        } else {
            Some(columns)
        };

        self
    }

    pub fn columns(&self) -> Option<&[Str]> {
        self.columns.as_deref()
    }
}

impl From<Str> for Alias {
    fn from(alias: Str) -> Self {
        Self::new(alias)
    }
}

impl From<&'static str> for Alias {
    fn from(alias: &'static str) -> Self {
        Self::new(alias)
    }
}

use pg_basics::Str;
