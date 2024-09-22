
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FnInfo {
    file_name: &'static str,
    function: &'static str,
    line_number: u32,
}

impl FnInfo {

    #[inline(always)]
    pub fn new(file_name: &'static str, function: &'static str, line_number: u32) -> Self {
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
