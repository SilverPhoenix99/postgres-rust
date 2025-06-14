#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FuncArgsKind {
    Empty {
        order_within_group: Option<Vec<SortBy>>,
    },
    /// `*` in SQL
    Wildcard {
        order_within_group: Option<Vec<SortBy>>,
    },
    All {
        args: Vec<FuncArgExpr>,
        order: Option<FuncArgsOrder>,
    },
    Variadic {
        args: Vec<FuncArgExpr>,
        order: Option<Vec<SortBy>>,
    },
    Distinct {
        args: Vec<FuncArgExpr>,
        order: Option<Vec<SortBy>>,
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FuncArgsOrder {
    OrderBy(Vec<SortBy>),
    WithinGroup(Vec<SortBy>),
}

use crate::FuncArgExpr;
use crate::SortBy;
