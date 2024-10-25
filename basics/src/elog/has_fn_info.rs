use crate::FnInfo;

pub trait HasFnInfo {
    fn fn_info(&self) -> &'static FnInfo;
}
