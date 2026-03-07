#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Presence {
    /// Fail the command if the existence/absence condition (`IF (NOT)? EXISTS`) was not included in the statement.
    #[default]
    Fail,
    /// Skip the statement if the object already exists (e.g., `create table IF NOT EXISTS`),
    /// or if it doesn't (e.g., `drop table IF EXISTS`).
    Ignore,
}
