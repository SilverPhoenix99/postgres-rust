#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonArrayAgg {
    arg: JsonValueExpr,
    output: Option<JsonOutput>,
    absent_on_null: bool,
    sort: Option<Vec<SortBy>>,
}

impl JsonArrayAgg {
    pub fn new<T: Into<JsonValueExpr>>(arg: T) -> Self {
        Self {
            arg: arg.into(),
            output: None,
            absent_on_null: false,
            sort: None,
        }
    }

    pub fn arg(&self) -> &JsonValueExpr {
        &self.arg
    }

    pub fn set_output(&mut self, output: Option<JsonOutput>) -> &mut Self {
        self.output = output;
        self
    }

    pub fn with_output<T : Into<JsonOutput>>(mut self, output: T) -> Self {
        self.output = Some(output.into());
        self
    }

    pub fn output(&self) -> Option<&JsonOutput> {
        self.output.as_ref()
    }

    pub fn set_absent_on_null(&mut self, absent_on_null: bool) -> &mut Self {
        self.absent_on_null = absent_on_null;
        self
    }

    pub fn with_absent_on_null(mut self, absent_on_null: bool) -> Self {
        self.absent_on_null = absent_on_null;
        self
    }

    pub fn absent_on_null(&self) -> bool {
        self.absent_on_null
    }

    pub fn set_sort(&mut self, sort: Option<Vec<SortBy>>) -> &mut Self {
        self.sort = sort;
        self
    }

    pub fn with_sort(mut self, sort: Vec<SortBy>) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn sort(&self) -> Option<&[SortBy]> {
        self.sort.as_deref()
    }
}

use crate::JsonOutput;
use crate::JsonValueExpr;
use crate::SortBy;
