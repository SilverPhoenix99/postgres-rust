#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FuncCall {
    name: QualifiedName,
    args: FuncArgsKind,
    agg_filter: Option<ExprNode>,
    over: Option<OverClause>,
}

impl FuncCall {
    pub fn new(
        name: QualifiedName,
        args: FuncArgsKind,
        agg_filter: Option<ExprNode>,
        over: Option<OverClause>,
    ) -> Self {
        Self { name, args, agg_filter, over }
    }

    pub fn name(&self) -> &[Str] {
        &self.name
    }

    pub fn args(&self) -> &FuncArgsKind {
        &self.args
    }

    pub fn agg_filter(&self) -> Option<&ExprNode> {
        self.agg_filter.as_ref()
    }

    pub fn over(&self) -> Option<&OverClause> {
        self.over.as_ref()
    }
}

use crate::ExprNode;
use crate::FuncArgsKind;
use crate::OverClause;
use pg_basics::QualifiedName;
use pg_basics::Str;
