mod bool_config;
mod config_group;
mod config_type;
mod guc_context;
mod guc_source;
mod guc_stack_state;
mod guc_flags;
mod guc_status;
mod opaque;
mod config_var;
mod guc_stack;
mod config_metadata;
mod generic_config;
mod enum_config;

pub use self::{
    bool_config::{BoolConfig, SimpleBoolConfig},
    config_group::ConfigGroup,
    config_metadata::ConfigMetadata,
    config_type::ConfigType,
    config_var::ConfigVar,
    enum_config::{EnumConfig, SimpleEnumConfig},
    generic_config::GenericConfig,
    guc_context::GucContext,
    guc_flags::GucFlags,
    guc_source::GucSource,
    guc_stack::GucStack,
    guc_stack_state::GucStackState,
    guc_status::GucStatus,
    opaque::Opaque,
};

pub struct Guc {
    enable_seqscan: SimpleBoolConfig,
    escape_string_warning: SimpleBoolConfig,
    standard_conforming_strings: SimpleBoolConfig,
    backslash_quote: SimpleEnumConfig<BackslashQuote>
}

impl Guc {
    pub fn enable_seqscan(&self) -> &SimpleBoolConfig {
        &self.enable_seqscan
    }

    pub fn escape_string_warning(&self) -> &SimpleBoolConfig {
        &self.escape_string_warning
    }

    pub fn standard_conforming_strings(&self) -> &SimpleBoolConfig {
        &self.standard_conforming_strings
    }

    pub fn backslash_quote(&self) -> &SimpleEnumConfig<BackslashQuote> {
        &self.backslash_quote
    }
}

lazy_static! {
    pub static ref GUC: Guc = Guc {

        enable_seqscan: SimpleBoolConfig::new(
            ConfigMetadata::new(
                "enable_seqscan",
                GucContext::Userset,
                ConfigGroup::QueryTuningMethod,
                "Enables the planner's use of sequential-scan plans.",
                None,
                GucFlags::Explain,
                ConfigType::Bool
            ),
            BoolConfig::new(true)
        ),

        escape_string_warning: SimpleBoolConfig::new(
            ConfigMetadata::new(
                "escape_string_warning",
                GucContext::Userset,
                ConfigGroup::CompatOptionsPrevious,
                "Warn about backslash escapes in ordinary string literals.",
                None,
                GucFlags::None,
                ConfigType::Bool
            ),
            BoolConfig::new(true)
        ),

        standard_conforming_strings: SimpleBoolConfig::new(
            ConfigMetadata::new(
                "standard_conforming_strings",
                GucContext::Userset,
                ConfigGroup::CompatOptionsPrevious,
                "Causes '...' strings to treat backslashes literally.",
                None,
                GucFlags::Report,
                ConfigType::Bool
            ),
            BoolConfig::new(true)
        ),

        backslash_quote: SimpleEnumConfig::new(
            ConfigMetadata::new(
                "backslash_quote",
                GucContext::Userset,
                ConfigGroup::CompatOptionsPrevious,
                r#"Sets whether "'" is allowed in string literals."#,
                None,
                GucFlags::None,
                ConfigType::Enum
            ),
            EnumConfig::new(BackslashQuote::SafeEncoding)
        ),
    };
}

use lazy_static::lazy_static;
use postgres_basics::guc::BackslashQuote;
