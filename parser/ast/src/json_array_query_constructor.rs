#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonArrayQueryConstructor {
    query: SelectStmt,
    format: Option<JsonFormat>,
    output: Option<JsonOutput>,
}

impl JsonArrayQueryConstructor {

    pub fn new(query: SelectStmt) -> Self {
        Self {
            query,
            output: None,
            format: None,
        }
    }

    pub fn query(&self) -> &SelectStmt {
        &self.query
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

    pub fn set_format(&mut self, format: Option<JsonFormat>) -> &mut Self {
        self.format = format;
        self
    }

    pub fn with_format(mut self, format: JsonFormat) -> Self {
        self.format = Some(format);
        self
    }

    pub fn format(&self) -> Option<&JsonFormat> {
        self.format.as_ref()
    }

    pub fn absent_on_null(&self) -> bool {
        // Weird flex from C-PG, but ok
        true
    }
}

use crate::JsonFormat;
use crate::JsonOutput;
use crate::SelectStmt;
