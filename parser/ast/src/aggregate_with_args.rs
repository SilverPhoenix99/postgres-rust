#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AggregateWithArgs {
    name: QualifiedName,
    args: Vec<FunctionParameter>,
    order_by: Vec<FunctionParameter>,
}

impl AggregateWithArgs {
    pub fn new(name: QualifiedName, args: Vec<FunctionParameter>, order_by: Vec<FunctionParameter>) -> Self {
        Self { name, args, order_by }
    }

    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    pub fn args(&self) -> &[FunctionParameter] {
        &self.args
    }

    pub fn order_by(&self) -> &[FunctionParameter] {
        &self.order_by
    }

    pub fn has_args_signature(&self) -> bool {
        !self.args.is_empty() || !self.order_by.is_empty()
    }
}

use crate::FunctionParameter;
use pg_basics::QualifiedName;
