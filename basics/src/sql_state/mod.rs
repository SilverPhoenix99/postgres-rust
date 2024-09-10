mod enums;
mod err_code;
mod variant_sets;

use core::fmt;
use core::fmt::{Display, Formatter, Write};
pub use enums::{ErrorSqlState, SuccessSqlState, WarningSqlState};
pub use err_code::ErrCode;

fn fmt(code: u32, formatter: &mut Formatter<'_>) -> fmt::Result {

    (0..5).rev()
        .map(|i| {
            // get the corresponding 6 bits
            let mut c = (code >> (6 * i)) as u8;
            c &= 0x3f;
            // convert to char
            c += b'0';
            c as char
        })
        .try_fold((), |_, c| formatter.write_char(c))
}

impl From<SuccessSqlState> for u32 {

    #[inline(always)]
    fn from(value: SuccessSqlState) -> Self {
        value as u32
    }
}

impl Display for SuccessSqlState {

    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt(u32::from(*self), f)
    }
}

impl From<WarningSqlState> for u32 {

    #[inline(always)]
    fn from(value: WarningSqlState) -> Self {
        value as u32
    }
}

impl Display for WarningSqlState {

    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt(u32::from(*self), f)
    }
}

impl From<ErrorSqlState> for u32 {

    #[inline(always)]
    fn from(value: ErrorSqlState) -> Self {
        value as u32
    }
}

impl Display for ErrorSqlState {

    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt(u32::from(*self), f)
    }
}
