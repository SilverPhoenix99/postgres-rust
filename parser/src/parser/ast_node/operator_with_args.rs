#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OperatorWithArgs {
    name: QualifiedOperator,
    left_arg: Option<Type>,
    right_arg: Option<Type>,
}

impl OperatorWithArgs {
    pub fn new(name: QualifiedOperator, left_arg: Option<Type>, right_arg: Option<Type>) -> Self {
        debug_assert_ne!((&left_arg, &right_arg), (&None, &None));
        Self { name, left_arg, right_arg }
    }

    pub fn name(&self) -> &QualifiedOperator {
        &self.name
    }

    pub fn left_arg(&self) -> Option<&Type> {
        self.left_arg.as_ref()
    }

    pub fn right_arg(&self) -> Option<&Type> {
        self.right_arg.as_ref()
    }
}

use crate::parser::ast_node::QualifiedOperator;
use crate::parser::ast_node::Type;
