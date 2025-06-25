
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FnInfo {
    file_name: &'static str,
    function: &'static str,
    line_number: u32,
}

impl FnInfo {

    // The order of the parameters is important here.
    // qual_fn_name!() is used last, because the macro expands to multiple lines.
    pub const fn new(file_name: &'static str, line_number: u32, function: &'static str) -> Self {
        Self { file_name, function, line_number }
    }

    pub fn file_name(&self) -> &'static str {
        self.file_name
    }

    pub fn function(&self) -> &'static str {
        self.function
    }

    pub fn line_number(&self) -> u32 {
        self.line_number
    }
}
