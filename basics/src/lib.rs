
/// Qualified function name
#[macro_export]
macro_rules! qn_fn_name {
    () => {{
        let mut name = std::any::type_name_of_val(&||{});
        while let Some(rest) = name.strip_suffix("::{{closure}}") {
            name = rest
        }
        name
    }};
}

#[macro_export]
macro_rules! fn_info {
    () => { $crate::FnInfo::new(file!(), line!(), $crate::qn_fn_name!()) };
}

pub mod ascii;
mod char_buffer;
mod concealable;
pub mod elog;
mod fn_info;
pub mod guc;
mod location;
mod named;
pub mod sql_state;
pub mod wchar;

pub use self::{
    char_buffer::{CharBuffer, Position, UnicodeChar, UnicodeCharError},
    concealable::Concealable,
    fn_info::FnInfo,
    location::{Located, Location},
    named::Named,
};

pub type Oid = u32;

pub const NAMEDATALEN: usize = 64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qn_fn_name() {
        let fn_name = module_path!().to_owned() + "::test_qn_fn_name";
        assert_eq!(fn_name, qn_fn_name!());
    }

    #[test]
    fn test_fn_info() {
        let fn_info = fn_info!();
        let fn_name = module_path!().to_owned() + "::test_fn_info";
        assert_eq!(fn_name, fn_info.function());
    }
}
