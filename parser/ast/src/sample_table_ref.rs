/// Alias: `RangeTableSample`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SampleTableRef {
    relation: RelationTableRef,
    function_name: QualifiedName,
    args: Vec<ExprNode>,
    repeatable: Option<ExprNode>,
}

impl SampleTableRef {

    pub fn new<T: Into<RelationTableRef>>(relation: T, function_name: QualifiedName, args: Vec<ExprNode>) -> Self {
        Self {
            relation: relation.into(),
            function_name,
            args,
            repeatable: None,
        }
    }

    pub fn relation(&self) -> &RelationTableRef {
        &self.relation
    }

    pub fn function_name(&self) -> &QualifiedName {
        &self.function_name
    }

    pub fn args(&self) -> &[ExprNode] {
        &self.args
    }

    pub fn set_repeatable(&mut self, repeatable: Option<ExprNode>) -> &mut Self {
        self.repeatable = repeatable;
        self
    }

    pub fn with_repeatable(mut self, repeatable: ExprNode) -> Self {
        self.repeatable = Some(repeatable);
        self
    }

    pub fn repeatable(&self) -> Option<&ExprNode> {
        self.repeatable.as_ref()
    }
}

use crate::ExprNode;
use crate::RelationTableRef;
use pg_basics::QualifiedName;
