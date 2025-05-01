#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SortNulls {
    #[default]
    Default,
    First,
    Last,
}
