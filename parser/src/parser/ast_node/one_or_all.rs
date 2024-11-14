#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OneOrAll {
    All,
    Name(Str),
}

use postgres_basics::Str;
