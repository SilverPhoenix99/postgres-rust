#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlTableColumn {
    name: Str,
    kind: XmlTableColumnKind
}

impl XmlTableColumn {
    pub fn new<S, T>(name: S, kind: T) -> Self
    where
        S: Into<Str>,
        T: Into<XmlTableColumnKind>
    {
        Self {
            name: name.into(),
            kind: kind.into()
        }
    }

    pub fn name(&self) -> &Str {
        &self.name
    }

    pub fn kind(&self) -> &XmlTableColumnKind {
        &self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XmlTableColumnKind {
    ForOrdinality,
    ColumnDefinition(XmlTableColumnDefinition),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlTableColumnDefinition {
    type_name: Type,
    is_not_null: bool,
    default_value: Option<ExprNode>,
    path_spec: Option<ExprNode>,
}

impl XmlTableColumnDefinition {
    pub fn new<T>(type_name: T) -> Self
    where
        T: Into<Type>
    {
        Self {
            type_name: type_name.into(),
            is_not_null: Default::default(),
            default_value: Default::default(),
            path_spec: Default::default(),
        }
    }

    pub fn type_name(&self) -> &Type {
        &self.type_name
    }

    pub fn set_not_null(&mut self, is_not_null: bool) -> &mut Self {
        self.is_not_null = is_not_null;
        self
    }

    pub fn with_not_null(mut self, is_not_null: bool) -> Self {
        self.is_not_null = is_not_null;
        self
    }

    pub fn is_not_null(&self) -> bool {
        self.is_not_null
    }

    pub fn set_default_value(&mut self, value: Option<ExprNode>) -> &mut Self {
        self.default_value = value;
        self
    }

    pub fn with_default_value(mut self, value: ExprNode) -> Self {
        self.default_value = Some(value);
        self
    }

    pub fn default_value(&self) -> Option<&ExprNode> {
        self.default_value.as_ref()
    }

    pub fn set_path_spec(&mut self, path: Option<ExprNode>) -> &mut Self {
        self.path_spec = path;
        self
    }

    pub fn with_path_spec(mut self, path: ExprNode) -> Self {
        self.path_spec = Some(path);
        self
    }

    pub fn path_spec(&self) -> Option<&ExprNode> {
        self.path_spec.as_ref()
    }
}

impl<T> From<T> for XmlTableColumnDefinition
where
    T: Into<Type>,
{
    fn from(value: T) -> Self {
        XmlTableColumnDefinition::new(value)
    }
}

impl From<XmlTableColumnDefinition> for XmlTableColumnKind {
    fn from(value: XmlTableColumnDefinition) -> Self {
        Self::ColumnDefinition(value)
    }
}

use pg_basics::Str;
use crate::ExprNode;
use crate::Type;
