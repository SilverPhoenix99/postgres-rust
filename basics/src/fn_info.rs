
#[derive(Debug, Copy, Clone, PartialEq)]
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
}
