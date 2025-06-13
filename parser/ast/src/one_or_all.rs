#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OneOrAll<T> {
    All,
    One(T),
}

impl<T> OneOrAll<T> {
    pub fn as_ref(&self) -> OneOrAll<&T> {
        match *self {
            OneOrAll::All => OneOrAll::All,
            OneOrAll::One(ref value) => OneOrAll::One(value),
        }
    }
}

impl<T> OneOrAll<T>
where
    T: Deref
{
    pub fn as_deref(&self) -> OneOrAll<&T::Target> {
        match *self {
            OneOrAll::All => OneOrAll::All,
            OneOrAll::One(ref value) => OneOrAll::One(value.deref())
        }
    }
}

use core::ops::Deref;
