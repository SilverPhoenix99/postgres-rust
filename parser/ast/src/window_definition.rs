#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowDefinition {
    name: Option<Str>,
    partition_clause: Vec<ExprNode>,
    order_clause: Vec<SortBy>,
    frame_clause: Option<WindowFrame>,
}

impl WindowDefinition {
    pub fn new(
        name: Option<Str>,
        partition_clause: Vec<ExprNode>,
        order_by: Vec<SortBy>,
        frame_clause: Option<WindowFrame>,
    ) -> Self {
        Self { name, partition_clause, order_clause: order_by, frame_clause }
    }

    pub fn name(&self) -> Option<&Str> {
        self.name.as_ref()
    }

    pub fn partition_clause(&self) -> &[ExprNode] {
        &self.partition_clause
    }

    pub fn order_clause(&self) -> &[SortBy] {
        &self.order_clause
    }

    pub fn frame_clause(&self) -> Option<&WindowFrame> {
        self.frame_clause.as_ref()
    }
}

use crate::ExprNode;
use crate::SortBy;
use crate::WindowFrame;
use pg_basics::Str;
