#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DatabaseStmtOption {

    /// `CREATE DATABASE`
    Create(Vec<CreatedbOption>),

    /// `ALTER DATABASE REFRESH COLLATION VERSION`
    RefreshCollationVersion,

    /// `ALTER DATABASE ( WITH )?`
    /// `ALTER DATABASE SET TABLESPACE`
    AlterOptions(Vec<AlterdbOption>),

    /// `ALTER DATABASE SET`
    Set(SetRest),

    /// `ALTER DATABASE RESET`
    Reset(VariableTarget),

    /// `ALTER DATABASE OWNER TO`
    AlterOwner { new_owner: RoleSpec },

    /// `ALTER DATABASE RENAME TO`
    Rename { new_name: Str },

    /// `COMMENT ON`
    Comment(Option<Box<str>>),
    
    /// `SECURITY LABEL ON`
    SecurityLabel(SecurityLabel),
}

use crate::AlterdbOption;
use crate::CreatedbOption;
use crate::RoleSpec;
use crate::SecurityLabel;
use crate::SetRest;
use crate::VariableTarget;
use pg_basics::Str;
