
pub fn fn_name_of_val(f: impl Fn() + 'static) -> &'static str {

    // Rust doesn't guarantee what the format of `type_name_of_val()` will be,
    // so we'll attempt to get the slice safely, assuming anything might not match.
    let mut name = type_name_of_val(&f);

    // Remove "<<" prefixed (generics from lazy_static)
    let num_bytes = name.chars()
        .take_while(|c| *c == '<')
        .count();

    // SAFETY: '<' is ASCII, so it's 1 byte
    name = &name[num_bytes..];

    // Get length until whitespace, if there's any (cast from lazy_static)
    let num_bytes = name.chars()
        .take_while(|c| !c.is_whitespace())
        .map(char::len_utf8)
        .sum();

    // SAFETY: Summing the number of bytes per UTF-8 char will be at a char boundary.
    name = &name[..num_bytes];

    // The current format should be something like: <crate>(::<module>)?::<fn>::<STATIC_NAME>
    // We'll just try to remove the last bit.

    if let Some(index) = name.rfind("::") {
        name = &name[..index]
    }

    // It might still be inside a closure, so we'll try to remove that too.
    while name.ends_with("::{{closure}}") {
        name = &name[..name.len() - "::{{closure}}".len()]
    }

    name
}

/// Qualified function name
#[macro_export]
macro_rules! qual_fn_name {
    () => {{
        lazy_static::lazy_static! {
            static ref FN_NAME: &'static str = $crate::fn_name_of_val(||{});
        };
        *FN_NAME
    }};
}

#[macro_export]
macro_rules! fn_info {
    () => {{
        lazy_static::lazy_static! {
            static ref FN_INFO: $crate::FnInfo = $crate::FnInfo::new(file!(), line!(), $crate::qual_fn_name!());
        }
        &FN_INFO
    }};

    ($fn_name:expr) => {{
        const FN_INFO: $crate::FnInfo = $crate::FnInfo::new(file!(), line!(), $fn_name);
        &FN_INFO
    }};
}

pub mod ascii;
pub mod elog;
pub mod guc;
pub mod mphf;
pub mod sql_state;
pub mod wchar;
mod char_buffer;
mod concealable;
mod fn_info;
mod location;
mod named;
mod non_negative;
mod str;

pub use self::{
    char_buffer::{CharBuffer, Position, UnicodeChar, UnicodeCharError},
    concealable::Concealable,
    fn_info::FnInfo,
    location::{Located, Location},
    named::Named,
    non_negative::NonNegative,
    str::Str,
};

pub type Oid = u32;

pub const NAMEDATALEN: usize = 64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qual_fn_name() {
        let fn_name = module_path!().to_owned() + "::test_qual_fn_name";
        assert_eq!(fn_name, qual_fn_name!());
    }

    #[test]
    fn test_fn_info() {
        let fn_info = fn_info!();
        let fn_name = module_path!().to_owned() + "::test_fn_info";
        assert_eq!(fn_name, fn_info.function());
    }

    #[test]
    fn test_fn_info_inside_closure() {

        #[allow(clippy::redundant_closure_call)]
        let fn_info = (|| { fn_info!() })();

        let fn_name = module_path!().to_owned() + "::test_fn_info_inside_closure";
        assert_eq!(fn_name, fn_info.function());
    }
}

use std::any::type_name_of_val;
