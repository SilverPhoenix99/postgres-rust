#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CreateCastStmt {
    typecast: Typecast,
    conversion: CastConversion,
    coercion: CoercionContext,
}

impl CreateCastStmt {
    pub fn new(typecast: Typecast, conversion: CastConversion, coercion: CoercionContext) -> Self {
        Self {
            typecast,
            conversion,
            coercion,
        }
    }

    pub fn typecast(&self) -> &Typecast {
        &self.typecast
    }

    pub fn conversion(&self) -> &CastConversion {
        &self.conversion
    }

    pub fn coercion(&self) -> CoercionContext {
        self.coercion
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum CoercionContext {
    #[default]
    Explicit,
    Implicit,
    Assignment,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CastConversion {
    WithInout,
    WithoutFunction,
    WithFunction(FunctionWithArgs),
}

use crate::parser::ast_node::FunctionWithArgs;
use crate::parser::ast_node::Typecast;
