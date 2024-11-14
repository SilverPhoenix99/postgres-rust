#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterOwnerStmt {
    target: AlterOwnerTarget,
    new_owner: RoleSpec,
}

impl AlterOwnerStmt {
    #[inline(always)]
    pub fn new(target: AlterOwnerTarget, new_owner: RoleSpec) -> Self {
        Self { target, new_owner }
    }

    #[inline(always)]
    pub fn target(&self) -> &AlterOwnerTarget {
        &self.target
    }

    #[inline(always)]
    pub fn new_owner(&self) -> &RoleSpec {
        &self.new_owner
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AlterOwnerTarget {
    Aggregate(AggregateWithArgtypes),
    Collation(QualifiedName),
    Conversion(QualifiedName),
    Database(Str),
    Domain(QualifiedName),
    EventTrigger(Str),
    ForeignDataWrapper(Str),
    ForeignServer(Str),
    Function(FunctionWithArgtypes),
    Language(Str),
    LargeObject(SignedNumber),
    Operator(OperatorWithArgtypes),
    OperatorClass(QualifiedName),
    OperatorFamily(QualifiedName),
    Procedure(FunctionWithArgtypes),
    Publication(Str),
    Routine(FunctionWithArgtypes),
    Schema(Str),
    Statistic(QualifiedName),
    Subscription(Str),
    Tablespace(Str),
    TextSearchConfiguration(QualifiedName),
    TextSearchDictionary(QualifiedName),
    Type(QualifiedName),
}

use crate::parser::ast_node::{
    AggregateWithArgtypes,
    FunctionWithArgtypes,
    OperatorWithArgtypes,
    QualifiedName,
    RoleSpec,
    SignedNumber
};
use postgres_basics::Str;
