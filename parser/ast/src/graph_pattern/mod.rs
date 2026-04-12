pub(super) mod graph_element_pattern;
pub(super) mod graph_element_pattern_kind;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct GraphPattern {
    path_patterns: Vec<Vec<GraphElementPatternKind>>,
    where_clause: Option<ExprNode>,
}

impl GraphPattern {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_path_patterns(&mut self, path_patterns: Vec<Vec<GraphElementPatternKind>>) -> &mut Self {
        self.path_patterns = path_patterns;
        self
    }

    pub fn with_path_patterns(mut self, path_patterns: Vec<Vec<GraphElementPatternKind>>) -> Self {
        self.path_patterns = path_patterns;
        self
    }

    pub fn path_patterns(&self) -> &[Vec<GraphElementPatternKind>] {
        &self.path_patterns
    }

    pub fn set_where_clause(&mut self, where_clause: Option<ExprNode>) -> &mut Self {
        self.where_clause = where_clause;
        self
    }

    pub fn with_where_clause(mut self, where_clause: ExprNode) -> Self {
        self.where_clause = Some(where_clause);
        self
    }

    pub fn where_clause(&self) -> Option<&ExprNode> {
        self.where_clause.as_ref()
    }
}

use crate::ExprNode;
use crate::GraphElementPatternKind;
