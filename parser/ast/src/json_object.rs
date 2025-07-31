#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonObjectExpr {
    ExplicitCall(Option<Vec<NamedValue>>),
    SqlSyntax(JsonObjectArgs),
}

impl Default for JsonObjectExpr {
    fn default() -> Self {
        Self::ExplicitCall(None)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct JsonObjectArgs {
    exprs: Option<Vec<JsonKeyValue>>,
    output: Option<JsonOutput>,
    unique: bool,
    absent_on_null: bool,
}

impl JsonObjectArgs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_exprs(&mut self, exprs: Option<Vec<JsonKeyValue>>) -> &mut Self {
        self.exprs = exprs;
        self
    }

    pub fn with_exprs(mut self, exprs: Vec<JsonKeyValue>) -> Self {
        self.exprs = if exprs.is_empty() { None} else { Some(exprs) };
        self
    }

    pub fn exprs(&self) -> Option<&[JsonKeyValue]> {
        self.exprs.as_deref()
    }

    pub fn set_output(&mut self, output: Option<JsonOutput>) -> &mut Self {
        self.output = output;
        self
    }

    pub fn with_output(mut self, output: JsonOutput) -> Self {
        self.output = Some(output);
        self
    }

    pub fn output(&self) -> Option<&JsonOutput> {
        self.output.as_ref()
    }

    pub fn set_unique(&mut self, unique: bool) -> &mut Self {
        self.unique = unique;
        self
    }

    pub fn with_unique(mut self, unique: bool) -> Self {
        self.unique = unique;
        self
    }

    pub fn unique(&self) -> bool {
        self.unique
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
}

use crate::JsonKeyValue;
use crate::JsonOutput;
use crate::NamedValue;
