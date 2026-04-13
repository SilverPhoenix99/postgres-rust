#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockingClause {
    pub locked_rels: Vec<RelationName>,
    pub strength: LockClauseStrength,
    pub wait_policy: LockWaitPolicy,
}

impl LockingClause {
    pub fn new(locked_rels: Vec<RelationName>, strength: LockClauseStrength, wait_policy: LockWaitPolicy)
        -> Self
    {
        Self { locked_rels, strength, wait_policy }
    }

    pub fn locked_rels(&self) -> &[RelationName] {
        &self.locked_rels
    }

    pub fn strength(&self) -> LockClauseStrength {
        self.strength
    }

    pub fn wait_policy(&self) -> LockWaitPolicy {
        self.wait_policy
    }
}

use crate::LockClauseStrength;
use crate::LockWaitPolicy;
use crate::RelationName;
