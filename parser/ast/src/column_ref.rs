#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColumnRef {

    /// Example: `foo`
    SingleName(Str),

    /// Example: `foo.bar`
    Name(Vec<Str>),

    /// Example: `foo.*`
    WildcardName(Vec<Str>),

    /// Example: `foo.bar[1:].*.baz`
    Indirection {
        name: Vec<Str>,
        indirection: Vec<Indirection>
    }
}

impl TryFrom<ColumnRef> for QualifiedName {
    type Error = ColumnRef;

    /// Returns the input parameter in `Err`,
    /// if it's not possible to convert the `ColumnRef` into a `QualifiedName`.
    fn try_from(value: ColumnRef) -> Result<Self, Self::Error> {

        match value {
            ColumnRef::SingleName(name) => Ok(vec![name]),
            ColumnRef::Name(name) => Ok(name),
            _ => Err(value)
        }
    }
}

use crate::Indirection;
use pg_basics::QualifiedName;
use pg_basics::Str;
