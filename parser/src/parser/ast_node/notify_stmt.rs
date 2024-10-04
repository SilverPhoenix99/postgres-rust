use crate::parser::ast_node::CowStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NotifyStmt {
    condition_name: CowStr,
    payload: Option<String>
}

impl NotifyStmt {
    #[inline(always)]
    pub fn new(condition_name: CowStr) -> Self {
        Self { condition_name, payload: None }
    }

    #[inline(always)]
    pub fn with_payload(condition_name: CowStr, payload: String) -> Self {
        Self { condition_name, payload: Some(payload) }
    }

    #[inline(always)]
    pub fn condition_name(&self) -> &CowStr {
        &self.condition_name
    }

    #[inline(always)]
    pub fn payload(&self) -> &Option<String> {
        &self.payload
    }
}