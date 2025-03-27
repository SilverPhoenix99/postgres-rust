#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RenameStmt {
    target: RenameTarget,
    new_name: Str,
}

impl RenameStmt {
    #[inline(always)]
    pub fn new<T: Into<Str>>(target: RenameTarget, new_name: T) -> Self {
        Self {
            target,
            new_name: new_name.into(),
        }
    }

    pub fn target(&self) -> &RenameTarget {
        &self.target
    }

    pub fn new_name(&self) -> &str {
        &self.new_name
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RenameTarget {
    Aggregate(AggregateWithArgtypes),
    Collation(QualifiedName),
    Conversion(QualifiedName),
    Database(Str),
    Domain(QualifiedName),
    DomainConstraint { domain: QualifiedName, constraint: Str },
    EventTrigger(Str),
    ForeignDataWrapper(Str),
    ForeignServer(Str),
    ForeignTable { target: RelationExpr, missing_ok: bool },
    ForeignTableColumn { table: RelationExpr, column: Str, missing_ok: bool },
    Function(FunctionWithArgtypes),
    Index { target: QualifiedName, missing_ok: bool },
    Language(Str),
    MaterializedView { target: QualifiedName, missing_ok: bool },
    MaterializedViewColumn { view: QualifiedName, column: QualifiedName, missing_ok: bool },
    OperatorClass(QualifiedName),
    OperatorFamily(QualifiedName),
    Policy { table: QualifiedName, policy: Str, missing_ok: bool },
    Procedure(FunctionWithArgtypes),
    Publication(Str),
    /// Aliases:
    /// * `Group`
    /// * `User`
    Role(Str),
    Routine(FunctionWithArgtypes),
    Rule { relation: QualifiedName, rule: Str },
    Schema(Str),
    Sequence { target: QualifiedName, missing_ok: bool },
    Statistic(QualifiedName),
    Subscription(Str),
    Table { target: RelationExpr, missing_ok: bool },
    TableColumn { table: RelationExpr, column: Str, missing_ok: bool },
    TableConstraint { table: RelationExpr, constraint: Str, missing_ok: bool },
    Tablespace(Str),
    TextSearchConfiguration(QualifiedName),
    TextSearchDictionary(QualifiedName),
    TextSearchParser(QualifiedName),
    TextSearchTemplate(QualifiedName),
    Trigger { table: QualifiedName, trigger: Str },
    Type(QualifiedName),
    TypeAttribute { typ: QualifiedName, attribute: Str },
    View { target: QualifiedName, missing_ok: bool },
    ViewColumn { view: QualifiedName, column: Str, missing_ok: bool },
}

use crate::parser::ast_node::{AggregateWithArgtypes, FunctionWithArgtypes, QualifiedName, RelationExpr};
use postgres_basics::Str;
