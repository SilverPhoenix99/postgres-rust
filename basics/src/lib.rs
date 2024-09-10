
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

mod concealable;
pub mod guc;
mod named;
pub mod wchar;
pub mod sql_state;

pub use concealable::Concealable;
pub use named::Named;

pub type Oid = u32;

pub const NAMEDATALEN: usize = 64;
