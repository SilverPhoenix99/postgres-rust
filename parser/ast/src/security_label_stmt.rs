/// Alias: `SecLabelStmt`
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityLabelStmt {
    target: SecurityLabelTarget,
    provider: Option<Str>,
    label: Option<Box<str>>
}

impl SecurityLabelStmt {
    pub fn new(provider: Option<Str>, target: SecurityLabelTarget, label: Option<Box<str>>) -> Self {
        Self { provider, target, label }
    }

    pub fn target(&self) -> &SecurityLabelTarget {
        &self.target
    }

    pub fn provider(&self) -> Option<&str> {
        self.provider.as_deref()
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SecurityLabelTarget {
    AccessMethod(Str),
    Aggregate(AggregateWithArgs),
    Collation(QualifiedName),
    Column(QualifiedName),
    Conversion(QualifiedName),
    Database(Str),
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

use crate::AggregateWithArgs;
use crate::FunctionWithArgs;
use crate::SignedNumber;
use crate::Type;
use pg_basics::QualifiedName;
use pg_basics::Str;
