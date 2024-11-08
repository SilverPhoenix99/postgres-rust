// see `struct ErrorData` @ [elog.h](https://github.com/postgres/postgres/blob/2ceeb638b7b27da156c10cb9d5ea4f81cabda0d1/src/include/utils/elog.h#L441)
pub trait SqlReport: ErrorReport + HasSqlState + HasFnInfo {}

impl<T> SqlReport for T
where
    T: ErrorReport + HasSqlState + HasFnInfo
{}

use crate::elog::{ErrorReport, HasFnInfo, HasSqlState};
