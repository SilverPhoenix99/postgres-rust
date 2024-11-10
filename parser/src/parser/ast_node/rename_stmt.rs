#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RenameStmt {
    target: RenameTarget,
    new_name: CowStr,
}

impl RenameStmt {
    #[inline(always)]
    pub fn new(target: RenameTarget, new_name: CowStr) -> Self {
        Self { target, new_name }
    }

    pub fn target(&self) -> &RenameTarget {
        &self.target
    }

    pub fn new_name(&self) -> &CowStr {
        &self.new_name
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RenameTarget {
    Aggregate(AggregateWithArgtypes),
    Collation(QualifiedName),
    Conversion(QualifiedName),
    Database(CowStr),
    Domain(QualifiedName),
    DomainConstraint { domain: QualifiedName, constraint: CowStr },
    EventTrigger(CowStr),
    ForeignDataWrapper(CowStr),
    ForeignServer(CowStr),
    ForeignTable { target: RelationExpr, missing_ok: bool },
    ForeignTableColumn { table: RelationExpr, column: CowStr, missing_ok: bool },
    Function(FunctionWithArgtypes),
    Index { target: QualifiedName, missing_ok: bool },
    Language(CowStr),
    MaterializedView { target: QualifiedName, missing_ok: bool },
    MaterializedViewColumn { view: QualifiedName, column: QualifiedName, missing_ok: bool },
    OperatorClass(QualifiedName),
    OperatorFamily(QualifiedName),
    Policy { table: QualifiedName, policy: CowStr, missing_ok: bool },
    Procedure(FunctionWithArgtypes),
    Publication(CowStr),
    /// Aliases:
    /// * `Group`
    /// * `User`
    Role(CowStr),
    Routine(FunctionWithArgtypes),
    Rule { relation: QualifiedName, rule: CowStr },
    Schema(CowStr),
    Sequence { target: QualifiedName, missing_ok: bool },
    Statistic(QualifiedName),
    Subscription(CowStr),
    Table { target: RelationExpr, missing_ok: bool },
    TableColumn { table: RelationExpr, column: CowStr, missing_ok: bool },
    TableConstraint { table: RelationExpr, constraint: CowStr, missing_ok: bool },
    Tablespace(CowStr),
    TextSearchConfiguration(QualifiedName),
    TextSearchDictionary(QualifiedName),
    TextSearchParser(QualifiedName),
    TextSearchTemplate(QualifiedName),
    Trigger { table: QualifiedName, trigger: CowStr },
    Type(QualifiedName),
    TypeAttribute { typ: QualifiedName, attribute: CowStr },
    View { target: QualifiedName, missing_ok: bool },
    ViewColumn { view: QualifiedName, column: CowStr, missing_ok: bool },
}

use crate::parser::{
    ast_node::{AggregateWithArgtypes, FunctionWithArgtypes, QualifiedName, RelationExpr},
    CowStr
};
