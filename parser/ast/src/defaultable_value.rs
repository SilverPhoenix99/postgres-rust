#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum DefaultableValue<T> {
    #[default]
    Default,
    Null,
    Value(T)
}
