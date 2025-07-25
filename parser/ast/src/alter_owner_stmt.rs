#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterOwnerStmt {
    target: AlterOwnerTarget,
    new_owner: RoleSpec,
}

impl AlterOwnerStmt {

    pub fn new(target: AlterOwnerTarget, new_owner: RoleSpec) -> Self {
        Self { target, new_owner }
    }

    pub fn target(&self) -> &AlterOwnerTarget {
        &self.target
    }

    pub fn new_owner(&self) -> &RoleSpec {
        &self.new_owner
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AlterOwnerTarget {
    Aggregate(AggregateWithArgs),
    Collation(QualifiedName),
    Conversion(QualifiedName),
    Database(Str),
    Domain(QualifiedName),
    EventTrigger(Str),
    ForeignDataWrapper(Str),
    ForeignServer(Str),
    Function(FunctionWithArgs),
    Language(Str),
    LargeObject(SignedNumber),
    Operator(OperatorWithArgs),
    OperatorClass(QualifiedName),
    OperatorFamily(QualifiedName),
    Procedure(FunctionWithArgs),
    Publication(Str),
    Routine(FunctionWithArgs),
    Schema(Str),
    Statistic(QualifiedName),
    Subscription(Str),
    Tablespace(Str),
    TextSearchConfiguration(QualifiedName),
    TextSearchDictionary(QualifiedName),
    Type(QualifiedName),
}

use crate::AggregateWithArgs;
use crate::FunctionWithArgs;
use crate::OperatorWithArgs;
use crate::RoleSpec;
use crate::SignedNumber;
use pg_basics::QualifiedName;
use pg_basics::Str;
