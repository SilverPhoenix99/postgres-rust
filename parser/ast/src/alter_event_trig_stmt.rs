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

    pub fn new<T: Into<Str>>(trigger: T, state: EventTriggerState) -> Self {
        Self {
            trigger: trigger.into(),
            state
        }
    }

    pub fn trigger(&self) -> &str {
        &self.trigger
    }

    pub fn state(&self) -> EventTriggerState {
        self.state
    }
}

use pg_basics::Str;
