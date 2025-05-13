#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum DropBehavior {
    #[default]
    Restrict,
    Cascade,
}
