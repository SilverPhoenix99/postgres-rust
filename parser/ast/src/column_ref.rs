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

use crate::Indirection;
use pg_basics::Str;
