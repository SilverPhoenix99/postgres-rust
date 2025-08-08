#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CommentStmt {
    target: CommentTarget,
    comment: Option<Box<str>>
}

impl CommentStmt {
    pub fn new(target: CommentTarget, comment: Option<Box<str>>) -> Self {
        Self { target, comment }
    }

    pub fn target(&self) -> &CommentTarget {
        &self.target
    }

    pub fn comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CommentTarget {
    AccessMethod(Str),
    Aggregate(AggregateWithArgs),
    Collation(QualifiedName),
    Column(QualifiedName),
    Conversion(QualifiedName),
    Database(Str),
    Domain(Type),
    DomainConstraint { domain: TypeName, constraint: Str },
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
    Operator(OperatorWithArgs),
    OperatorClass { name: QualifiedName, index_method: Str },
    OperatorFamily { name: QualifiedName, index_method: Str },
    Policy { name: Str, table: QualifiedName },
    Procedure(FunctionWithArgs),
    Publication(Str),
    Role(Str),
    Routine(FunctionWithArgs),
    Rule { name: Str, table: QualifiedName },
    Schema(Str),
    Sequence(QualifiedName),
    Subscription(Str),
    Table(QualifiedName),
    TableConstraint { table: QualifiedName, constraint: Str },
    Tablespace(Str),
    TextSearchConfiguration(QualifiedName),
    TextSearchDictionary(QualifiedName),
    TextSearchParser(QualifiedName),
    TextSearchTemplate(QualifiedName),
    Transform(Transform),
    Trigger { name: Str, table: QualifiedName },
    Type(Type),
    Typecast(Typecast),
    View(QualifiedName),
}

use crate::AggregateWithArgs;
use crate::FunctionWithArgs;
use crate::OperatorWithArgs;
use crate::Transform;
use crate::Type;
use crate::TypeName;
use crate::Typecast;
use pg_basics::QualifiedName;
use pg_basics::Str;
use pg_sink_ast::SignedNumber;
