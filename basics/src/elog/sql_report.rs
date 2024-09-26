// see `struct ErrorData` @ [elog.h](https://github.com/postgres/postgres/blob/2ceeb638b7b27da156c10cb9d5ea4f81cabda0d1/src/include/utils/elog.h#L441)
pub trait SqlReport: Error {

    fn sqlstate(&self) -> SqlState;

    fn fn_info(&self) -> FnInfo;

    fn hint(&self) -> Option<Cow<'static, str>> {
        None
    }

    fn detail(&self) -> Option<Cow<'static, str>> {
        None
    }

    fn detail_log(&self) -> Option<Cow<'static, str>> {
        None
    }
}

use crate::sql_state::SqlState;
use crate::FnInfo;
use std::borrow::Cow;
use std::error::Error;
