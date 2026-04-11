// See [`AlterDatabase()`](https://github.com/postgres/postgres/blob/75818b3afbf850d600e0fcd1a3b03199077063f8/src/backend/commands/dbcommands.c#L2363-L2396)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AlterdbOptionKind {
    AllowConnections,
    ConnectionLimit,
    IsTemplate,
    Tablespace,
    Unknown(Box<str>),
}
