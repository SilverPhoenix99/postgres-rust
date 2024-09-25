mod enums;
mod variant_sets;

pub use self::enums::{ErrorSqlState, SuccessSqlState, WarningSqlState};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SqlState {
    Success(SuccessSqlState),
    Warning(WarningSqlState),
    Error(ErrorSqlState)
}

impl SqlState {
    pub fn sqlstate(&self) -> u32 {
        match self {
            Self::Success(code) => u32::from(*code),
            Self::Warning(code) => u32::from(*code),
            Self::Error(code) => u32::from(*code),
        }
    }
}

impl Display for SqlState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success(code) => code.fmt(f),
            Self::Warning(code) => code.fmt(f),
            Self::Error(code) => code.fmt(f),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct UnknownSqlState;

impl TryFrom<u32> for SqlState {
    type Error = UnknownSqlState;

    fn try_from(value: u32) -> Result<Self, UnknownSqlState> {

        if value == 0 {
            Ok(Self::Success(SuccessfulCompletion))
        }
        else if value > 0x2aaaaaaa /* `ZZZZZ` */ {
            Err(UnknownSqlState)
        }
        else if value >= 0x000c0000 /* `03000` */ && ERROR_VARIANTS.contains(&value) {
            let code = unsafe { mem::transmute::<u32, ErrorSqlState>(value) };
            Ok(Self::Error(code))
        }
        else if WARNING_VARIANTS.contains(&value) {
            let code = unsafe { mem::transmute::<u32, WarningSqlState>(value) };
            Ok(Self::Warning(code))
        }
        else {
            Err(UnknownSqlState)
        }
    }
}

impl From<SqlState> for u32 {
    fn from(value: SqlState) -> Self {
        match value {
            SqlState::Success(code) => code.into(),
            SqlState::Warning(code) => code.into(),
            SqlState::Error(code) => code.into(),
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::ErrorSqlState::SyntaxError;
    use super::SqlState::*;
    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(0, u32::from(Success(SuccessfulCompletion)));
        assert_eq!(0x40000, u32::from(Warning(WarningSqlState::Warning)));
        assert_eq!(0x4086001, u32::from(Error(SyntaxError)));
    }

    #[test]
    fn test_try_from() {
        assert_eq!(Ok(Success(SuccessfulCompletion)), SqlState::try_from(0));
        assert_eq!(Ok(Warning(WarningSqlState::Warning)), SqlState::try_from(0x40000));
        assert_eq!(Ok(Error(SyntaxError)), SqlState::try_from(0x4086001));
    }
    
    #[test]
    fn test_to_string() {
        assert_eq!("00000", Success(SuccessfulCompletion).to_string());
        assert_eq!("01000", Warning(WarningSqlState::Warning).to_string());
        assert_eq!("42601", Error(SyntaxError).to_string());
    }
}

use self::{
    variant_sets::{ERROR_VARIANTS, WARNING_VARIANTS},
    SuccessSqlState::SuccessfulCompletion,
};
use core::fmt;
use core::fmt::{Display, Formatter, Write};
use std::mem;
