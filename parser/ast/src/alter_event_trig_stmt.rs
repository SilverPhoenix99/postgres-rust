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
    pub fn new<T: Into<Str>>(trigger: T, state: EventTriggerState) -> Self {
        Self {
            trigger: trigger.into(),
            state
        }
    }

    #[inline(always)]
    pub fn trigger(&self) -> &str {
        &self.trigger
    }

    #[inline(always)]
    pub fn state(&self) -> EventTriggerState {
        self.state
    }
}

use pg_basics::Str;
