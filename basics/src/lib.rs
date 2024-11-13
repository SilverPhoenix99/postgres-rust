
pub const fn clean_fn_name(fn_name: &'static str) -> &'static str {

    let mut name = (*fn_name).as_bytes();

    while name.len() >= 13 {
        let suffix_start = name.len() - 13;
        match name.split_at_checked(suffix_start) {
            Some((prefix, b"::{{closure}}")) => name = prefix,
            _ => break,
        }
    }

    // SAFETY: The origin was already `str`, and the excluded suffix is ASCII,
    // so the boundary is always correct.
    unsafe { core::str::from_utf8_unchecked(name) }
}

/// Qualified function name
#[macro_export]
macro_rules! qual_fn_name {
    () => { $crate::clean_fn_name(std::any::type_name_of_val(&||{})) };
}

#[macro_export]
macro_rules! fn_info {
    () => { $crate::FnInfo::new(file!(), line!(), $crate::qual_fn_name!()) };

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

pub use self::{
    char_buffer::{CharBuffer, Position, UnicodeChar, UnicodeCharError},
    concealable::Concealable,
    fn_info::FnInfo,
    location::{Located, Location},
    named::Named,
    non_negative::NonNegative,
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
