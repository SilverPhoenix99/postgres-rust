#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonArrayAgg {
    arg: JsonValueExpr,
    output: Option<JsonOutput>,
    absent_on_null: bool,
    sort: Option<Vec<SortBy>>,
}

impl JsonArrayAgg {
    pub fn new(
        arg: JsonValueExpr,
        output: Option<JsonOutput>,
        absent_on_null: bool,
        sort: Option<Vec<SortBy>>,
    ) -> Self {
        Self {
            arg,
            output,
            absent_on_null,
            sort,
        }
    }

    pub fn arg(&self) -> &JsonValueExpr {
        &self.arg
    }

    pub fn output(&self) -> Option<&JsonOutput> {
        self.output.as_ref()
    }

    pub fn absent_on_null(&self) -> bool {
        self.absent_on_null
    }

    pub fn sort(&self) -> Option<&[SortBy]> {
        self.sort.as_deref()
    }
}

use crate::JsonOutput;
use crate::JsonValueExpr;
use crate::SortBy;
