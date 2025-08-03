#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonTableColumnDefinition {
    ForOrdinality { column_name: Str },
    Regular(JsonTableRegularColumn),
    Exists(JsonTableExistsColumn),
    Nested(JsonTableNestedColumn),
}

impl_from!(JsonTableRegularColumn for JsonTableColumnDefinition::Regular);
impl_from!(JsonTableExistsColumn for JsonTableColumnDefinition::Exists);
impl_from!(JsonTableNestedColumn for JsonTableColumnDefinition::Nested);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonTableRegularColumn {
    column_name: Str,
    type_name: Type,
    format: Option<JsonFormat>,
    path_spec: JsonTablePathSpec,
    wrapper: JsonWrapperBehavior,
    quotes: Option<JsonQuotes>,
    behavior: Option<JsonBehaviorClause>
}

impl JsonTableRegularColumn {
    pub fn new<S, T>(
        column_name: S,
        type_name: T,
        path_spec: JsonTablePathSpec,
        wrapper: JsonWrapperBehavior
    ) -> Self
    where
        S: Into<Str>,
        T: Into<Type>
    {
        Self {
            column_name: column_name.into(),
            type_name: type_name.into(),
            format: None,
            path_spec,
            wrapper,
            quotes: None,
            behavior: None
        }
    }

    pub fn set_format(mut self, format: Option<JsonFormat>) -> Self {
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

    pub fn set_quotes(mut self, quotes: Option<JsonQuotes>) -> Self {
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

    pub fn set_behavior(mut self, behavior: Option<JsonBehaviorClause>) -> Self {
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

    pub fn column_name(&self) -> &str {
        &self.column_name
    }

    pub fn type_name(&self) -> &Type {
        &self.type_name
    }

    pub fn path_spec(&self) -> &JsonTablePathSpec {
        &self.path_spec
    }

    pub fn wrapper(&self) -> &JsonWrapperBehavior {
        &self.wrapper
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

    pub fn set_path_spec(mut self, path_spec: Option<JsonTablePathSpec>) -> Self {
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

    pub fn set_on_error(mut self, on_error: Option<JsonBehavior>) -> Self {
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

    pub fn column_name(&self) -> &str {
        &self.column_name
    }

    pub fn type_name(&self) -> &Type {
        &self.type_name
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonTableNestedColumn {
    column_name: Option<Str>,
    path_spec: JsonTablePathSpec,
    columns: Vec<JsonTableColumnDefinition>
}

impl JsonTableNestedColumn {
    pub fn new(path_spec: JsonTablePathSpec, columns: Vec<JsonTableColumnDefinition>) -> Self {
        Self {
            column_name: None,
            path_spec,
            columns
        }
    }

    pub fn set_column_name<T>(mut self, column_name: Option<Str>) -> Self {
        self.column_name = column_name;
        self
    }

    pub fn with_column_name<T>(mut self, column_name: T) -> Self
    where
        T: Into<Str>
    {
        self.column_name = Some(column_name.into());
        self
    }

    pub fn column_name(&self) -> Option<&str> {
        self.column_name.as_deref()
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
use pg_basics::impl_from;
use pg_basics::Str;
