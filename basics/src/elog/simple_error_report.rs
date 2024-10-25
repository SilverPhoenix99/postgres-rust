#[derive(Debug, Clone, PartialEq)]
pub struct SimpleErrorReport<T> {
    source: T,
    fn_info: &'static FnInfo,
}

impl<T: Eq> Eq for SimpleErrorReport<T> {}

impl<T> SimpleErrorReport<T> {
    #[inline(always)]
    pub fn new(source: T, fn_info: &'static FnInfo) -> Self {
        Self { source, fn_info }
    }

    #[inline(always)]
    pub fn source(&self) -> &T {
        &self.source
    }
}

impl<T> HasFnInfo for SimpleErrorReport<T> {
    #[inline(always)]
    fn fn_info(&self) -> &'static FnInfo {
        self.fn_info
    }
}

impl<T: Error + 'static> Error for SimpleErrorReport<T> {
    #[inline(always)]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl<T: Error> Display for SimpleErrorReport<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.source, f)
    }
}

impl<T: Error + 'static> ErrorReport for SimpleErrorReport<T> {}

use crate::elog::{ErrorReport, HasFnInfo};
use crate::FnInfo;
use std::error::Error;
use std::fmt::{Display, Formatter};
