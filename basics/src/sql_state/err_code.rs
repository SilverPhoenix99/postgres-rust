use crate::sql_state::variant_sets::{ERROR_VARIANTS, WARNING_VARIANTS};
use crate::sql_state::SuccessSqlState::SuccessfulCompletion;
use crate::sql_state::{ErrorSqlState, SuccessSqlState, WarningSqlState};
use std::fmt::{Display, Formatter};
use std::mem;
use ErrCode::{Error, Success, Warning};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrCode {
    Success(SuccessSqlState),
    Warning(WarningSqlState),
    Error(ErrorSqlState)
}

impl ErrCode {
    pub fn sqlstate(&self) -> u32 {
        match self {
            Success(code) => u32::from(*code),
            Warning(code) => u32::from(*code),
            Error(code) => u32::from(*code),
        }
    }
}

impl Display for ErrCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Success(code) => code.fmt(f),
            Warning(code) => code.fmt(f),
            Error(code) => code.fmt(f),
        }
    }
}

impl TryFrom<u32> for ErrCode {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, ()> {

        if value == 0 {
            Ok(Success(SuccessfulCompletion))
        }
        else if value > 0x2aaaaaaa /* `ZZZZZ` */ {
            Err(())
        }
        else if value >= 0x000c0000 /* 03000 */ && ERROR_VARIANTS.contains(&value) {
            let code = unsafe { mem::transmute::<u32, ErrorSqlState>(value) };
            Ok(Error(code))
        }
        else if WARNING_VARIANTS.contains(&value) {
            let code = unsafe { mem::transmute::<u32, WarningSqlState>(value) };
            Ok(Warning(code))
        }
        else {
            Err(())
        }
    }
}
