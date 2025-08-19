#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GenericOptionKind {
    Unspecified(GenericOption),
    Set(GenericOption),
    Add(GenericOption),
    Drop(Str)
}

use crate::generic_option::GenericOption;
use pg_basics::Str;
