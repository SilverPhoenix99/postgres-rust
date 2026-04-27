use crate::ExprNode;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct JsonIsPredicate {
    expression: ExprNode,
    kind: JsonValueKind,
    unique_keys: bool,
}

impl JsonIsPredicate {

    pub fn new(expression: ExprNode) -> Self {
        Self {
            expression,
            kind: Default::default(),
            unique_keys: false,
        }
    }

    pub fn expression(&self) -> &ExprNode {
        &self.expression
    }

    pub fn set_kind(&mut self, kind: JsonValueKind) -> &mut Self {
        self.kind = kind;
        self
    }

    pub fn with_kind(mut self, kind: JsonValueKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> JsonValueKind {
        self.kind
    }

    pub fn set_unique_keys(&mut self, unique_keys: bool) -> &mut Self {
        self.unique_keys = unique_keys;
        self
    }

    pub fn with_unique_keys(mut self, unique_keys: bool) -> Self {
        self.unique_keys = unique_keys;
        self
    }

    pub fn unique_keys(&self) -> bool {
        self.unique_keys
    }
}

/// Alias: `JsonValueType`
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum JsonValueKind {
    /// Alias: `Value`
    #[default]
    Value,
    Object,
    Array,
    Scalar,
}
