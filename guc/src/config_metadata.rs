use crate::{ConfigGroup, ConfigType, GucContext, GucFlags};

/// Constant fields, must be set correctly in initial value.
#[derive(Debug)]
pub struct ConfigMetadata {
    /// name of variable
    name: &'static str,
    /// context required to set the variable
    context: GucContext,
    /// to help organize variables by function
    group: ConfigGroup,
    /// short desc. of this variable's purpose
    short_desc: &'static str,
    /// long desc. of this variable's purpose
    long_desc: Option<&'static str>,
    /// flag bits
    flags: GucFlags,
    /// type of variable (set only at startup)
    vartype: ConfigType,
}

impl ConfigMetadata {
    pub fn new(
        name: &'static str,
        context: GucContext,
        group: ConfigGroup,
        short_desc: &'static str,
        long_desc: Option<&'static str>,
        flags: GucFlags,
        vartype: ConfigType,
    ) -> Self {
        Self {
            name,
            context,
            group,
            short_desc,
            long_desc,
            flags,
            vartype,
        }
    }

    pub fn name(self: &Self) -> &'static str {
        self.name
    }

    pub fn context(self: &Self) -> GucContext {
        self.context
    }

    pub fn group(self: &Self) -> ConfigGroup {
        self.group
    }

    pub fn short_desc(self: &Self) -> &'static str {
        self.short_desc
    }

    pub fn long_desc(self: &Self) -> Option<&'static str> {
        self.long_desc
    }

    pub fn flags(self: &Self) -> GucFlags {
        self.flags
    }

    pub fn vartype(self: &Self) -> ConfigType {
        self.vartype
    }
}
