#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypecastExpr<E = ExprNode> {
    arg: E,
    type_name: Type,
}

impl<E> TypecastExpr<E> {
    pub fn new<F, T>(arg: F, type_name: T) -> Self
    where
        F: Into<E>,
        T: Into<Type>
    {
        Self {
            arg: arg.into(),
            type_name: type_name.into(),
        }
    }

    pub fn arg(&self) -> &E {
        &self.arg
    }

    pub fn type_name(&self) -> &Type {
        &self.type_name
    }
}

pub type StringTypecastExpr = TypecastExpr<Box<str>>;

use crate::ExprNode;
use crate::Type;
