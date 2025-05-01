#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum SortDirection {
    #[default]
    Default,
    Ascending,
    Descending,
}
