#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RelationPersistence {
    /// regular table
    Permanent = b'p',
    /// unlogged permanent table
    Unlogged  = b'u',
    /// temporary table
    Temp      = b't',
}
