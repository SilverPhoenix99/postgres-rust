#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NotifyStmt {
    condition_name: Str,
    payload: Option<Box<str>>
}

impl NotifyStmt {
    #[inline(always)]
    pub fn new(condition_name: Str) -> Self {
        Self { condition_name, payload: None }
    }

    #[inline(always)]
    pub fn with_payload(condition_name: Str, payload: Box<str>) -> Self {
        Self { condition_name, payload: Some(payload) }
    }

    #[inline(always)]
    pub fn condition_name(&self) -> &Str {
        &self.condition_name
    }

    #[inline(always)]
    pub fn payload(&self) -> Option<&str> {
        match self.payload.as_ref() {
            None => None,
            Some(payload) => Some(payload.as_ref())
        }
    }
}

use postgres_basics::Str;
