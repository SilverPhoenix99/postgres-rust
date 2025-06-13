#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NotifyStmt {
    condition_name: Str,
    payload: Option<Box<str>>
}

impl NotifyStmt {
    #[inline(always)]
    pub fn new<T: Into<Str>>(condition_name: T) -> Self {
        Self {
            condition_name: condition_name.into(),
            payload: None
        }
    }

    #[inline(always)]
    pub fn with_payload<N, P>(condition_name: N, payload: P) -> Self
    where
        N: Into<Str>,
        P: Into<Box<str>>
    {
        Self {
            condition_name: condition_name.into(),
            payload: Some(payload.into())
        }
    }

    #[inline(always)]
    pub fn condition_name(&self) -> &str {
        &self.condition_name
    }

    #[inline(always)]
    pub fn payload(&self) -> Option<&str> {
        self.payload.as_deref()
    }
}

use pg_basics::Str;
