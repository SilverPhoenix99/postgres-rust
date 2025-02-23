#[derive(Debug)]
pub struct BoolConfig {
    value: bool,
    /// constant field, must be set correctly in initial value
    boot_val: bool
}

impl BoolConfig {
    pub fn new(boot_val: bool) -> Self {
        Self {
            value: boot_val,
            boot_val
        }
    }
}

pub struct SimpleBoolConfig {
    metadata: ConfigMetadata,
    generic: GenericConfig,
    config: BoolConfig,
}

impl SimpleBoolConfig {
    pub fn new(metadata: ConfigMetadata, config: BoolConfig) -> Self {
        Self { metadata, config, generic: GenericConfig::default() }
    }
}

use crate::ConfigMetadata;
use crate::GenericConfig;
