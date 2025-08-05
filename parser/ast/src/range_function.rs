#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeFunction {
    func_call: FuncExprWindowless,
    column_definitions: Option<Vec<SimpleColumnDefinition>>
}

impl RangeFunction {
    pub fn new(func_call: FuncExprWindowless) -> Self {
        Self {
            func_call,
            column_definitions: None,
        }
    }

    pub fn func_call(&self) -> &FuncExprWindowless {
        &self.func_call
    }

    pub fn set_column_definitions(&mut self, cols: Option<Vec<SimpleColumnDefinition>>) -> &mut Self {
        self.column_definitions = cols.and_then(|cols| {
            if cols.is_empty() { None }
            else { Some(cols) }
        });
        self
    }

    pub fn with_column_definitions(mut self, cols: Vec<SimpleColumnDefinition>) -> Self {
        self.column_definitions = if cols.is_empty() { None } else { Some(cols) };
        self
    }

    pub fn column_definitions(&self) -> Option<&[SimpleColumnDefinition]> {
        self.column_definitions.as_deref()
    }
}

use crate::FuncExprWindowless;
use crate::SimpleColumnDefinition;
