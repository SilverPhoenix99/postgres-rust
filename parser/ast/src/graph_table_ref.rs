#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphTableRef {
    graph_name: RelationName,
    graph_pattern: GraphPattern,
    columns: Vec<NamedValue>,
    alias: Option<Alias>,
}

impl GraphTableRef {
    pub fn new<T>(graph_name: T, graph_pattern: GraphPattern, columns: Vec<NamedValue>) -> Self
    where
        T: Into<RelationName>,
    {
        Self {
            graph_name: graph_name.into(),
            graph_pattern,
            columns,
            alias: None,
        }
    }

    pub fn graph_name(&self) -> &RelationName {
        &self.graph_name
    }

    pub fn graph_pattern(&self) -> &GraphPattern {
        &self.graph_pattern
    }

    pub fn columns(&self) -> &[NamedValue] {
        &self.columns
    }

    pub fn set_alias(&mut self, alias: Option<Alias>) -> &mut Self {
        self.alias = alias;
        self
    }

    pub fn with_alias<T: Into<Alias>>(mut self, alias: T) -> Self {
        self.alias = Some(alias.into());
        self
    }

    pub fn alias(&self) -> Option<&Alias> {
        self.alias.as_ref()
    }
}

use crate::Alias;
use crate::GraphPattern;
use crate::NamedValue;
use crate::RelationName;
