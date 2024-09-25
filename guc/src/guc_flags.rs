bitflags! {
    /**
     * Bit values in "flags" of a GUC variable.  Note that these don't appear
     * on disk, so we can reassign their values freely.
     */
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct GucFlags: u32 {
        const None               = 0;
        /// input can be list format
        const ListInput          = 0x000001;
        /// double-quote list elements
        const ListQuote          = 0x000002;
        /// exclude from `SHOW ALL`
        const NoShowAll          = 0x000004;
        /// disallow `RESET` and `SAVE`
        const NoReset            = 0x000008;
        /// exclude from `RESET ALL`
        const NoResetAll         = 0x000010;
        /// include in `EXPLAIN`
        const Explain            = 0x000020;
        /// auto-report changes to client
        const Report             = 0x000040;
        /// not in `postgresql.conf.sample`
        const NotInSample        = 0x000080;
        /// can't set in `postgresql.conf`
        const DisallowInFile     = 0x000100;
        /// placeholder for custom variable
        const CustomPlaceholder  = 0x000200;
        /// show only to superusers
        const SuperuserOnly      = 0x000400;
        /// limit string to `NAMEDATALEN-1`
        const IsName             = 0x000800;
        /// can't set if security restricted
        const NotWhileSecRest    = 0x001000;
        /// can't set in `PG_AUTOCONF_FILENAME`
        const DisallowInAutoFile = 0x002000;
        /// delay processing in `postgres -C`
        const RuntimeComputed    = 0x004000;
        /// allow setting in parallel mode
        const AllowInParallel    = 0x008000;
        /// value is in kilobytes
        const UnitKb             = 0x01000000;
        /// value is in blocks
        const UnitBlocks         = 0x02000000;
        /// value is in xlog blocks
        const UnitXblocks        = 0x03000000;
        /// value is in megabytes
        const UnitMb             = 0x04000000;
        /// value is in bytes
        const UnitByte           = 0x05000000;
        /// mask for size-related units
        const UnitMemory         = 0x0F000000;
        /// value is in milliseconds
        const UnitMs             = 0x10000000;
        /// value is in seconds
        const UnitS              = 0x20000000;
        /// value is in minutes
        const UnitMin            = 0x30000000;
        /// mask for time-related units
        const UnitTime           = 0x70000000;
        const Unit               = Self::UnitMemory.bits() | Self::UnitTime.bits();
    }
}

impl Default for GucFlags {
    fn default() -> Self {
        Self::None
    }
}

use bitflags::bitflags;
