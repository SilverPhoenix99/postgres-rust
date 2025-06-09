#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypecastExpr {
    arg: ExprNode,
    type_name: Type,
}

impl TypecastExpr {
    pub fn new<E, T>(arg: E, type_name: T) -> Self
    where
        E: Into<ExprNode>,
        T: Into<Type>
    {
        Self {
            arg: arg.into(),
            type_name: type_name.into(),
        }
    }

    pub fn arg(&self) -> &ExprNode {
        &self.arg
    }

    pub fn type_name(&self) -> &Type {
        &self.type_name
    }
}

use crate::ExprNode;
use crate::Type;
