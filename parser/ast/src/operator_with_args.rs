#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OperatorWithArgs {
    name: QualifiedOperator,
    args: OneOrBoth<Type>,
}

impl OperatorWithArgs {
    pub fn new(name: QualifiedOperator, args: OneOrBoth<Type>) -> Self {
        Self { name, args }
    }

    pub fn name(&self) -> &QualifiedOperator {
        &self.name
    }

    pub fn args(&self) -> OneOrBoth<&Type> {
        self.args.as_ref()
    }
}

use crate::OneOrBoth;
use crate::Type;
use pg_sink_ast::QualifiedOperator;
