#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OverClause {
    WindowName(Str),
    WindowDefinition(WindowDefinition),
}

use crate::WindowDefinition;
use pg_basics::Str;
