#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowDefinition {
    name: Option<Str>,
    partition_clause: Option<Vec<ExprNode>>,
    order_clause: Option<Vec<SortBy>>,
    frame_clause: Option<WindowFrame>,
}

impl WindowDefinition {
    pub fn new(
        name: Option<Str>,
        partition_clause: Option<Vec<ExprNode>>,
        order_by: Option<Vec<SortBy>>,
        frame_clause: Option<WindowFrame>,
    ) -> Self {
        Self { name, partition_clause, order_clause: order_by, frame_clause }
    }

    pub fn name(&self) -> Option<&Str> {
        self.name.as_ref()
    }

    pub fn partition_clause(&self) -> Option<&[ExprNode]> {
        self.partition_clause.as_deref()
    }

    pub fn order_clause(&self) -> Option<&[SortBy]> {
        self.order_clause.as_deref()
    }

    pub fn frame_clause(&self) -> Option<&WindowFrame> {
        self.frame_clause.as_ref()
    }
}

use crate::ExprNode;
use crate::SortBy;
use crate::WindowFrame;
use pg_basics::Str;
