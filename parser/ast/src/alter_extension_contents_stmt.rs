#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterExtensionContentsStmt {
    name: Str,
    action: AddDrop,
    target: AlterExtensionContentsTarget
}

impl AlterExtensionContentsStmt {
    pub fn new<T: Into<Str>>(name: T, action: AddDrop, target: AlterExtensionContentsTarget) -> Self {
        Self {
            name: name.into(),
            action,
            target
        }
    }

    pub fn name(&self) -> &Str {
        &self.name
    }

    pub fn action(&self) -> AddDrop {
        self.action
    }

    pub fn target(&self) -> &AlterExtensionContentsTarget {
        &self.target
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AlterExtensionContentsTarget {
    AccessMethod(Str),
    Aggregate(AggregateWithArgs),
    Collation(QualifiedName),
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
    MaterializedView(QualifiedName),
    Operator(OperatorWithArgs),
    OperatorClass { name: QualifiedName, index_method: Str },
    OperatorFamily { name: QualifiedName, index_method: Str },
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
    Transform(Transform),
    Type(Type),
    Typecast(Typecast),
    View(QualifiedName),
}

use crate::AddDrop;
use crate::AggregateWithArgs;
use crate::FunctionWithArgs;
use crate::OperatorWithArgs;
use crate::Transform;
use crate::Type;
use crate::Typecast;
use postgres_basics::QualifiedName;
use postgres_basics::Str;
