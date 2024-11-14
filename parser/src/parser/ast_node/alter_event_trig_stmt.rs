#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EventTriggerState {
    FiresOnReplica,
    FiresOnOrigin,
    FiresAlways,
    Disabled,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterEventTrigStmt {
    trigger: Str,
    state: EventTriggerState,
}

impl AlterEventTrigStmt {
    #[inline(always)]
    pub fn new(trigger: Str, state: EventTriggerState) -> Self {
        Self { trigger, state }
    }

    #[inline(always)]
    pub fn trigger(&self) -> &Str {
        &self.trigger
    }

    #[inline(always)]
    pub fn state(&self) -> EventTriggerState {
        self.state
    }
}

use postgres_basics::Str;
