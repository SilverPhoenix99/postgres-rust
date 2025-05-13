#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Typecast {
    from_type: Type,
    to_type: Type
}

impl Typecast {
    pub fn new<F: Into<Type>, T: Into<Type>>(from_type: F, to_type: T) -> Self {
        Self {
            from_type: from_type.into(),
            to_type: to_type.into()
        }
    }

    pub fn from_type(&self) -> &Type {
        &self.from_type
    }

    pub fn to_type(&self) -> &Type {
        &self.to_type
    }
}

use crate::Type;
