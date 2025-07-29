#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonObjectAgg {
    arg: JsonKeyValue,
    output: Option<JsonOutput>,
    unique: bool,
    absent_on_null: bool,
}

impl JsonObjectAgg {
    pub fn new(arg: JsonKeyValue, output: Option<JsonOutput>, unique: bool, absent_on_null: bool) -> Self {
        Self {
            arg,
            output,
            unique,
            absent_on_null,
        }
    }

    pub fn arg(&self) -> &JsonKeyValue {
        &self.arg
    }

    pub fn output(&self) -> Option<&JsonOutput> {
        self.output.as_ref()
    }

    pub fn unique(&self) -> bool {
        self.unique
    }

    pub fn absent_on_null(&self) -> bool {
        self.absent_on_null
    }
}

use crate::JsonKeyValue;
use crate::JsonOutput;
