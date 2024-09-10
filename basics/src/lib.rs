mod concealable;
pub mod guc;
mod named;
pub mod sql_state;

pub use concealable::Concealable;
pub use named::Named;

pub type Oid = u32;

pub const NAMEDATALEN: usize = 64;
