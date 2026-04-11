#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct GraphElementPattern {
    variable: Option<Str>,
    label_expr: Option<Vec<Str>>,
    where_clause: Option<ExprNode>,
    quantifier: Option<RangeInclusive<NonNegative>>,
}

impl GraphElementPattern {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_variable(&mut self, variable: Option<Str>) -> &mut Self {
        self.variable = variable;
        self
    }

    pub fn with_variable<T: Into<Str>>(mut self, variable: T) -> Self {
        self.variable = Some(variable.into());
        self
    }

    pub fn variable(&self) -> Option<&str> {
        self.variable.as_deref()
    }

    pub fn set_label_expr(&mut self, label_expr: Option<Vec<Str>>) -> &mut Self {

        self.label_expr = label_expr.and_then(|labels|
            if labels.is_empty() { None }
            else { Some(labels) }
        );

        self
    }

    pub fn with_label_expr(mut self, label_expr: Vec<Str>) -> Self {
        self.label_expr = if label_expr.is_empty() { None } else { Some(label_expr) };
        self
    }

    pub fn label_expr(&self) -> Option<&[Str]> {
        self.label_expr.as_deref()
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

    pub fn set_quantifier(&mut self, quantifier: Option<RangeInclusive<NonNegative>>) -> &mut Self {
        self.quantifier = quantifier;
        self
    }

    pub fn with_quantifier(mut self, quantifier: RangeInclusive<NonNegative>) -> Self {
        self.quantifier = Some(quantifier);
        self
    }

    pub fn quantifier(&self) -> Option<&RangeInclusive<NonNegative>> {
        self.quantifier.as_ref()
    }
}

use crate::ExprNode;
use core::ops::RangeInclusive;
use pg_basics::NonNegative;
use pg_basics::Str;
