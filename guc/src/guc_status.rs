use bitflags::bitflags;

bitflags! {
    /// bit values in status field
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct GucStatus: u32 {
        const None = 0;
        /// found it in config file
        const IsInFile = 0x0001;
        /**
         * Caution: the IsInFile bit is transient state for ProcessConfigFile.
         * Do not assume that its value represents useful information elsewhere.
         */
        const PendingRestart = 0x0002; /* changed value cannot be applied yet */
        const NeedsReport    = 0x0004; /* new value must be reported to client */
    }
}

impl Default for GucStatus {
    fn default() -> Self {
        Self::None
    }
}
