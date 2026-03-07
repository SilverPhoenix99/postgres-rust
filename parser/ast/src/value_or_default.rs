#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum ValueOrDefault<T> {
    #[default]
    Default,
    Value(T)
}
