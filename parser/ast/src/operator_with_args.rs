#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OperatorWithArgs {
    name: QualifiedOperator,
    args: OneOrBoth<Type>,
}

impl OperatorWithArgs {
    pub fn new<T>(name: T, args: OneOrBoth<Type>) -> Self
    where
        T: Into<QualifiedOperator>,
    {
        Self {
            name: name.into(),
            args
        }
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
