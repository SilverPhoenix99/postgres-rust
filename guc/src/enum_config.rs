#[derive(Debug)]
pub struct EnumConfig<T>
where
    T: Debug + Copy + Clone + Sized {

    value: T,

    /// constant field, must be set correctly in initial value
    boot_val: T,
}

impl<T> EnumConfig<T>
where
    T: Debug + Copy + Clone + Sized {

    pub fn new(boot_val: T) -> Self {
        Self {
            value: boot_val,
            boot_val: boot_val.clone()
        }
    }
}

#[derive(Debug)]
pub struct SimpleEnumConfig<T>
where
    T: Debug + Copy + Clone + Sized + 'static {

    metadata: ConfigMetadata,
    generic: GenericConfig,
    config: EnumConfig<T>
}

impl<T> SimpleEnumConfig<T>
where
    T: Debug + Copy + Clone + Sized {

    pub fn new(metadata: ConfigMetadata, config: EnumConfig<T>) -> Self {
        Self { metadata, config, generic: GenericConfig::default() }
    }
}

use crate::ConfigMetadata;
use crate::GenericConfig;
use core::fmt::Debug;
