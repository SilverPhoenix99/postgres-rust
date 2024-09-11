
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
    () => { postgres_basics::FnInfo::new(file!(), postgres_basics::qn_fn_name!(), line!()) };
}

mod concealable;
mod fn_info;
pub mod guc;
mod named;
pub mod wchar;
pub mod sql_state;

pub use concealable::Concealable;
pub use fn_info::FnInfo;
pub use named::Named;

pub type Oid = u32;

pub const NAMEDATALEN: usize = 64;
