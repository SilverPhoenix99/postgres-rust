#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnicodeNormalForm {
    /// Alias: `NFC`
    CanonicalComposition,
    /// Alias: `NFD`
    CanonicalDecomposition,
    /// Alias: `NFKC`
    CompatibilityComposition,
    /// Alias: `NFKD`
    CompatibilityDecomposition,
}
