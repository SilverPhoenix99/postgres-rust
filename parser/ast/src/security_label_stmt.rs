/// Alias: `SecLabelStmt`
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityLabelStmt {
    target: SecurityLabelTarget,
    label: SecurityLabel
}

impl SecurityLabelStmt {
    pub fn new(target: SecurityLabelTarget, label: SecurityLabel) -> Self {
        Self { target, label }
    }

    pub fn target(&self) -> &SecurityLabelTarget {
        &self.target
    }

    pub fn label(&self) -> &SecurityLabel {
        &self.label
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SecurityLabelTarget {
    AccessMethod(Str),
    Aggregate(AggregateWithArgs),
    Collation(QualifiedName),
    Column(QualifiedName),
    Conversion(QualifiedName),
    Domain(Type),
    EventTrigger(Str),
    ExtendedStatistics(QualifiedName),
    Extension(Str),
    ForeignDataWrapper(Str),
    ForeignServer(Str),
    ForeignTable(QualifiedName),
    Function(FunctionWithArgs),
    Index(QualifiedName),
    Language(Str),
    LargeObject(SignedNumber),
    MaterializedView(QualifiedName),
    Procedure(FunctionWithArgs),
    PropertyGraph(QualifiedName),
    Publication(Str),
    Role(Str),
    Routine(FunctionWithArgs),
    Schema(Str),
    Sequence(QualifiedName),
    Subscription(Str),
    Table(QualifiedName),
    Tablespace(Str),
    TextSearchConfiguration(QualifiedName),
    TextSearchDictionary(QualifiedName),
    TextSearchParser(QualifiedName),
    TextSearchTemplate(QualifiedName),
    Type(Type),
    View(QualifiedName),
}

use crate::FunctionWithArgs;
use crate::SignedNumber;
use crate::Type;
use crate::{AggregateWithArgs, SecurityLabel};
use pg_basics::QualifiedName;
use pg_basics::Str;
