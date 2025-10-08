#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum FunctionParameterMode {
    #[default]
    Default  = b'd' as isize,
    In       = b'i' as isize,
    Out      = b'o' as isize,
    InOut    = b'b' as isize,
    Variadic = b'v' as isize,
    Table    = b't' as isize,
}
