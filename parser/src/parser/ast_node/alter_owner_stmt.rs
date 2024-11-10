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
    Database(CowStr),
    Domain(QualifiedName),
    EventTrigger(CowStr),
    ForeignDataWrapper(CowStr),
    ForeignServer(CowStr),
    Function(FunctionWithArgtypes),
    Language(CowStr),
    LargeObject(SignedNumber),
    Operator(OperatorWithArgtypes),
    OperatorClass(QualifiedName),
    OperatorFamily(QualifiedName),
    Procedure(FunctionWithArgtypes),
    Publication(CowStr),
    Routine(FunctionWithArgtypes),
    Schema(CowStr),
    Statistic(QualifiedName),
    Subscription(CowStr),
    Tablespace(CowStr),
    TextSearchConfiguration(QualifiedName),
    TextSearchDictionary(QualifiedName),
    Type(QualifiedName),
}

use crate::parser::{
    ast_node::{
        AggregateWithArgtypes,
        FunctionWithArgtypes,
        OperatorWithArgtypes,
        QualifiedName,
        RoleSpec,
        SignedNumber
    },
    CowStr
};
