#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GrantStmt {
    is_grant: bool,
    privileges: AccessPrivilege,
    object_type: PrivilegeDefaultsTarget,
    grantees: Vec<RoleSpec>,
    grant_option: bool,
    drop_behavior: DropBehavior,
}

impl GrantStmt {
    pub fn grant(
        privileges: AccessPrivilege,
        object_type: PrivilegeDefaultsTarget,
        grantees: Vec<RoleSpec>,
        grant_option: bool
    ) -> Self {
        Self {
            is_grant: true,
            privileges,
            object_type,
            grantees,
            grant_option,
            drop_behavior: DropBehavior::Cascade
        }
    }

    pub fn revoke(
        privileges: AccessPrivilege,
        object_type: PrivilegeDefaultsTarget,
        grantees: Vec<RoleSpec>,
        grant_option: bool,
        drop_behavior: DropBehavior
    ) -> Self {
        Self {
            is_grant: false,
            privileges,
            object_type,
            grantees,
            grant_option,
            drop_behavior,
        }
    }

    pub fn is_grant(&self) -> bool {
        self.is_grant
    }

    pub fn is_revoke(&self) -> bool {
        !self.is_grant
    }

    pub fn privileges(&self) -> &AccessPrivilege {
        &self.privileges
    }

    pub fn object_type(&self) -> PrivilegeDefaultsTarget {
        self.object_type
    }

    pub fn grantees(&self) -> &[RoleSpec] {
        &self.grantees
    }

    pub fn grant_option(&self) -> bool {
        self.grant_option
    }
}


use crate::AccessPrivilege;
use crate::DropBehavior;
use crate::PrivilegeDefaultsTarget;
use crate::RoleSpec;
