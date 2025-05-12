#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ExtendedStringWarning {
    NonstandardEscape,
    NonstandardQuoteEscape,
    NonstandardBackslashEscape,
}
