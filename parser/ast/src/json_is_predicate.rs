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
