#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OneOrAll<T> {
    All,
    One(T),
}
