#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ValueOrDefault<T> {
    Default,
    Value(T)
}
