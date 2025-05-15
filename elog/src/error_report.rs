pub trait ErrorReport {

    #[inline(always)]
    fn hint(&self) -> Option<Cow<'static, str>> {
        None
    }

    #[inline(always)]
    fn detail(&self) -> Option<Cow<'static, str>> {
        None
    }

    #[inline(always)]
    fn detail_log(&self) -> Option<Cow<'static, str>> {
        None
    }
}

use std::borrow::Cow;
