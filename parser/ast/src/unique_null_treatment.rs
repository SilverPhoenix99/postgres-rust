#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum UniqueNullTreatment {
    NullsNotDistinct,
    #[default]
    NullsDistinct,
}

impl From<bool> for UniqueNullTreatment {
    fn from(value: bool) -> Self {
        if value { NullsDistinct } else { NullsNotDistinct }
    }
}

impl From<UniqueNullTreatment> for bool {
    fn from(value: UniqueNullTreatment) -> Self {
        value == NullsDistinct
    }
}

use UniqueNullTreatment::{NullsDistinct, NullsNotDistinct};
