#[derive(Debug, Clone, Eq, PartialEq, From)]
pub enum BooleanOrString {
    #[from]
    Boolean(bool),
    #[from(Str, String, &'static str, Box<str>)]
    String(Str)
}

use derive_more::From;
use pg_basics::Str;
