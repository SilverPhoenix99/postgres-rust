#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OverClause {
    WindowName(Str),
    WindowDefinition(WindowDefinition),
}

use pg_basics::Str;
use crate::WindowDefinition;
