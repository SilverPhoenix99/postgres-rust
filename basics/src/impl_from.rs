/// Generates `From` impls, where the input is wrapped in an output enum variant.
#[macro_export]
macro_rules! impl_from {

    ($variant:ident for $for_:ident) => {
        impl_from!($variant for $for_ :: $variant);
    };

    ($from:ident for $for_:ident :: $variant:ident) => {
        impl From<$from> for $for_ {
            fn from(value: $from) -> Self {
                Self::$variant(value.into())
            }
        }
    };

    (box $variant:ident for $for_:ident) => {
        impl_from!(box $variant for $for_ :: $variant);
    };

    (box $from:ident for $for_:ident :: $variant:ident) => {
        impl From<$from> for $for_ {
            fn from(value: $from) -> Self {
                Self::$variant(Box::new(value.into()))
            }
        }
    };
}
