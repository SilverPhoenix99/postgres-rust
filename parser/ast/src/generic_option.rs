#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GenericOption {
    name: Str,
    arg: Box<str>,
}

impl GenericOption {
    pub fn new<N, A>(name: N, arg: A) -> Self
    where
        N: Into<Str>,
        A: Into<Box<str>>
    {
        Self {
            name: name.into(),
            arg: arg.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn arg(&self) -> &str {
        &self.arg
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GenericOptionKind {
    Unspecified(GenericOption),
    Set(GenericOption),
    Add(GenericOption),
    Drop(Str)
}

use pg_basics::Str;
