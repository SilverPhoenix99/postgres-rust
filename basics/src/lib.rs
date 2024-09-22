
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

macro_rules! _fn_info {
    () => { FnInfo::new(file!(), qn_fn_name!(), line!()) };
}

#[macro_export]
macro_rules! fn_info {
    () => {{
        use postgres_basics::{FnInfo, qn_fn_name};
        _fn_info!()
    }};
}

pub mod ascii;
mod char_buffer;
mod concealable;
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
        let fn_info = _fn_info!();

        assert_eq!(file!(), fn_info.file_name());
        assert_eq!(qn_fn_name!(), fn_info.function());
        assert_eq!(line!() - 4, fn_info.line_number());
    }
}
