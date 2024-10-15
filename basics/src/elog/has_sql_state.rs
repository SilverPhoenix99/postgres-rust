pub trait HasSqlState {
    fn sql_state(&self) -> SqlState;
}

use crate::sql_state::SqlState;
