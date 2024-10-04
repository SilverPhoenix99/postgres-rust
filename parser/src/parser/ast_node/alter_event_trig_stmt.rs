#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EventTriggerState {
    FiresOnReplica,
    FiresOnOrigin,
    FiresAlways,
    Disabled,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterEventTrigStmt {
    trigger: CowStr,
    state: EventTriggerState,
}

impl AlterEventTrigStmt {
    #[inline(always)]
    pub fn new(trigger: CowStr, state: EventTriggerState) -> Self {
        Self { trigger, state }
    }

    #[inline(always)]
    pub fn trigger(&self) -> &CowStr {
        &self.trigger
    }

    #[inline(always)]
    pub fn state(&self) -> EventTriggerState {
        self.state
    }
}

use crate::parser::ast_node::CowStr;
