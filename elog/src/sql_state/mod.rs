mod sql_states;

pub use self::sql_states::SqlState;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SqlStateCategory {
    Success,
    Warning,
    Error,
}

impl SqlState {
    pub fn category(&self) -> SqlStateCategory {
        match *self as u32 {
            0 => SqlStateCategory::Success,
            /* [`01000`, `08000`) */
            0x40000..0x200000 => SqlStateCategory::Warning,
            /* [`08000`.. */
            _ => SqlStateCategory::Error,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct UnknownSqlState;

impl TryFrom<u32> for SqlState {
    type Error = UnknownSqlState;

    fn try_from(value: u32) -> Result<Self, UnknownSqlState> {

        if value == 0 {
            return Ok(Self::SuccessfulCompletion)
        }

        if MAP.get(&value).is_none() {
            return Err(UnknownSqlState)
        }

        // SAFETY: value is in MAP => known SQL State
        let code = unsafe { mem::transmute::<u32, SqlState>(value) };
        Ok(code)
    }
}

impl Display for SqlState {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        let code = *self as u32;

        (0..5).rev()
            .map(|i| {
                // get the corresponding 6 bits
                let mut c = (code >> (6 * i)) as u8;
                c &= 0x3f;
                // convert to char
                c += b'0';
                c as char
            })
            .try_for_each(|c| f.write_char(c))
    }
}

#[cfg(test)]
mod tests {
    use super::SqlState::{SuccessfulCompletion, SyntaxError};
    use super::*;

    #[test]
    fn test_try_from() {
        assert_eq!(Ok(SuccessfulCompletion), SqlState::try_from(0));
        assert_eq!(Ok(SqlState::Warning), SqlState::try_from(0x40000));
        assert_eq!(Ok(SyntaxError), SqlState::try_from(0x4086001));
    }

    #[test]
    fn test_to_string() {
        assert_eq!("00000", SuccessfulCompletion.to_string());
        assert_eq!("01000", SqlState::Warning.to_string());
        assert_eq!("42601", SyntaxError.to_string());
    }
}

use crate::sql_state::sql_states::MAP;
use core::fmt;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Write;
use core::mem;
