use crate::sql_state::SqlState;

pub trait HasSqlState {
    fn sql_state(&self) -> SqlState;
}