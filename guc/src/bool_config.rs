use crate::{ConfigMetadata, GenericConfig};

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
    gen: GenericConfig,
    config: BoolConfig,
}

impl SimpleBoolConfig {
    pub fn new(metadata: ConfigMetadata, config: BoolConfig) -> Self {
        Self { metadata, config, gen: GenericConfig::default() }
    }
}
