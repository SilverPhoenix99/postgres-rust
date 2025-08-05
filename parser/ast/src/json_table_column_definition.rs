#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum JsonTableColumnDefinition {
    #[from(ignore)]
    ForOrdinality { column_name: Str },
    Regular(JsonTableRegularColumn),
    Exists(JsonTableExistsColumn),
    Nested(JsonTableNestedColumn),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonTableRegularColumn {
    column_name: Str,
    type_name: Type,
    wrapper: JsonWrapperBehavior,
    format: Option<JsonFormat>,
    path_spec: Option<JsonTablePathSpec>,
    quotes: Option<JsonQuotes>,
    behavior: Option<JsonBehaviorClause>
}

impl JsonTableRegularColumn {
    pub fn new<S, T>(column_name: S, type_name: T, wrapper: JsonWrapperBehavior) -> Self
    where
        S: Into<Str>,
        T: Into<Type>
    {
        Self {
            column_name: column_name.into(),
            type_name: type_name.into(),
            format: None,
            path_spec : None,
            wrapper,
            quotes: None,
            behavior: None
        }
    }

    pub fn column_name(&self) -> &str {
        &self.column_name
    }

    pub fn type_name(&self) -> &Type {
        &self.type_name
    }

    pub fn wrapper(&self) -> &JsonWrapperBehavior {
        &self.wrapper
    }

    pub fn set_format(&mut self, format: Option<JsonFormat>) -> &mut Self {
        self.format = format;
        self
    }

    pub fn with_format(mut self, format: JsonFormat) -> Self {
        self.format = Some(format);
        self
    }

    pub fn format(&self) -> Option<&JsonFormat> {
        self.format.as_ref()
    }

    pub fn set_path_spec(&mut self, path_spec: Option<JsonTablePathSpec>) -> &mut Self {
        self.path_spec = path_spec;
        self
    }

    pub fn with_path_spec(mut self, path_spec: JsonTablePathSpec) -> Self {
        self.path_spec = Some(path_spec);
        self
    }

    pub fn path_spec(&self) -> Option<&JsonTablePathSpec> {
        self.path_spec.as_ref()
    }

    pub fn set_quotes(&mut self, quotes: Option<JsonQuotes>) -> &mut Self {
        self.quotes = quotes;
        self
    }

    pub fn with_quotes(mut self, quotes: JsonQuotes) -> Self {
        self.quotes = Some(quotes);
        self
    }

    pub fn quotes(&self) -> Option<&JsonQuotes> {
        self.quotes.as_ref()
    }

    pub fn set_behavior(&mut self, behavior: Option<JsonBehaviorClause>) -> &mut Self {
        self.behavior = behavior;
        self
    }

    pub fn with_behavior(mut self, behavior: JsonBehaviorClause) -> Self {
        self.behavior = Some(behavior);
        self
    }

    pub fn behavior(&self) -> Option<&JsonBehaviorClause> {
        self.behavior.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonTableExistsColumn {
    column_name: Str,
    type_name: Type,
    path_spec: Option<JsonTablePathSpec>,
    on_error: Option<JsonBehavior>,
}

impl JsonTableExistsColumn {
    pub fn new<S, T>(column_name: S, type_name: T) -> Self
    where
        S: Into<Str>,
        T: Into<Type>
    {
        Self {
            column_name: column_name.into(),
            type_name: type_name.into(),
            path_spec: None,
            on_error: None
        }
    }

    pub fn column_name(&self) -> &str {
        &self.column_name
    }

    pub fn type_name(&self) -> &Type {
        &self.type_name
    }

    pub fn set_path_spec(&mut self, path_spec: Option<JsonTablePathSpec>) -> &mut Self {
        self.path_spec = path_spec;
        self
    }

    pub fn with_path_spec(mut self, path_spec: JsonTablePathSpec) -> Self {
        self.path_spec = Some(path_spec);
        self
    }

    pub fn path_spec(&self) -> Option<&JsonTablePathSpec> {
        self.path_spec.as_ref()
    }

    pub fn set_on_error(&mut self, on_error: Option<JsonBehavior>) -> &mut Self {
        self.on_error = on_error;
        self
    }

    pub fn with_on_error(mut self, on_error: JsonBehavior) -> Self {
        self.on_error = Some(on_error);
        self
    }

    pub fn on_error(&self) -> Option<&JsonBehavior> {
        self.on_error.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonTableNestedColumn {
    path_spec: JsonTablePathSpec,
    columns: Vec<JsonTableColumnDefinition>,
}

impl JsonTableNestedColumn {
    pub fn new(path_spec: JsonTablePathSpec, columns: Vec<JsonTableColumnDefinition>) -> Self {
        Self {
            path_spec,
            columns
        }
    }

    pub fn path_spec(&self) -> &JsonTablePathSpec {
        &self.path_spec
    }

    pub fn columns(&self) -> &[JsonTableColumnDefinition] {
        &self.columns
    }
}

use crate::JsonBehavior;
use crate::JsonBehaviorClause;
use crate::JsonFormat;
use crate::JsonQuotes;
use crate::JsonTablePathSpec;
use crate::JsonWrapperBehavior;
use crate::Type;
use derive_more::From;
use pg_basics::Str;
