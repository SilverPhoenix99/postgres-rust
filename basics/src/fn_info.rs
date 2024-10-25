
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FnInfo {
    file_name: &'static str,
    function: &'static str,
    line_number: u32,
}

impl FnInfo {

    // The order of the parameters is important here.
    // qn_fn_name!() is used last, because the macro expands to multiple lines.
    #[inline(always)]
    pub const fn new(file_name: &'static str, line_number: u32, function: &'static str) -> Self {
        Self { file_name, function, line_number }
    }

    #[inline(always)]
    pub fn file_name(&self) -> &'static str {
        self.file_name
    }

    #[inline(always)]
    pub fn function(&self) -> &'static str {
        self.function
    }

    #[inline(always)]
    pub fn line_number(&self) -> u32 {
        self.line_number
    }
}
