// See [`createdb()`](https://github.com/postgres/postgres/blob/75818b3afbf850d600e0fcd1a3b03199077063f8/src/backend/commands/dbcommands.c#L744-L881)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CreatedbOptionKind {
    AllowConnections,
    BuiltinLocale,
    CollationVersion,
    ConnectionLimit,
    Encoding,
    IcuLocale,
    IcuRules,
    IsTemplate,
    LcCollate,
    LcCtype,
    Locale,
    LocaleProvider,
    /// Undocumented and deprecated.
    ///
    /// [`Tablespace`](crate::parser::ast_node::CreatedbOptionKind::Tablespace) should be used instead.
    Location,
    Owner,
    Tablespace,
    Template,
    Oid,
    Strategy,
    Unknown(Box<str>),
}
