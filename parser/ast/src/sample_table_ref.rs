/// Alias: `RangeTableSample`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SampleTableRef {
    relation: RelationExpr,
    alias: Option<Alias>,
    table_sample: Option<TableSample>,
}

impl SampleTableRef {
    pub fn new<T: Into<RelationExpr>>(relation: T) -> Self {
        Self {
            relation: relation.into(),
            alias: None,
            table_sample: None,
        }
    }

    pub fn relation(&self) -> &RelationExpr {
        &self.relation
    }

    pub fn alias(&self) -> Option<&Alias> {
        self.alias.as_ref()
    }

    pub fn set_alias(&mut self, alias: Option<Alias>) -> &mut Self {
        self.alias = alias;
        self
    }

    pub fn with_alias<T: Into<Alias>>(mut self, alias: T) -> Self {
        self.alias = Some(alias.into());
        self
    }

    pub fn table_sample(&self) -> Option<&TableSample> {
        self.table_sample.as_ref()
    }

    pub fn set_table_sample(&mut self, table_sample: Option<TableSample>) -> &mut Self {
        self.table_sample = table_sample;
        self
    }

    pub fn with_table_sample(mut self, table_sample: TableSample) -> Self {
        self.table_sample = Some(table_sample);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableSample {
    function_name: QualifiedName,
    args: Vec<ExprNode>,
    repeatable: Option<ExprNode>,
}

impl TableSample {
    pub fn new(function_name: QualifiedName, args: Vec<ExprNode>) -> Self {
        Self {
            function_name,
            args,
            repeatable: None,
        }
    }

    pub fn function_name(&self) -> &QualifiedName {
        &self.function_name
    }

    pub fn args(&self) -> &[ExprNode] {
        &self.args
    }

    pub fn repeatable(&self) -> Option<&ExprNode> {
        self.repeatable.as_ref()
    }

    pub fn set_repeatable(&mut self, repeatable: Option<ExprNode>) -> &mut Self {
        self.repeatable = repeatable;
        self
    }

    pub fn with_repeatable(mut self, repeatable: ExprNode) -> Self {
        self.repeatable = Some(repeatable);
        self
    }
}


use crate::Alias;
use crate::ExprNode;
use crate::RelationExpr;
use pg_basics::QualifiedName;
