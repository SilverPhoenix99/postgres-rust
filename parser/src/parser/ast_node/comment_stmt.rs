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
    Transform { for_type: Type, language: Str },
    Trigger { name: Str, table: QualifiedName },
    Type(Type),
    Typecast { from_type: Type, to_type: Type },
    View(QualifiedName),
}

use crate::parser::ast_node::AggregateWithArgs;
use crate::parser::ast_node::FunctionWithArgs;
use crate::parser::ast_node::OperatorWithArgs;
use crate::parser::ast_node::QualifiedName;
use crate::parser::ast_node::SignedNumber;
use crate::parser::ast_node::Type;
use crate::parser::ast_node::TypeName;
use postgres_basics::Str;
