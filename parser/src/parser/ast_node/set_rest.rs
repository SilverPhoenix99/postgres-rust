#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SetRest {
    /// `SESSION CHARACTERISTICS AS TRANSACTION transaction_mode+`
    SessionTransactionCharacteristics(Vec<TransactionMode>),
    /// `TRANSACTION transaction_mode+`
    LocalTransactionCharacteristics(Vec<TransactionMode>),
    SessionAuthorization {
        user: ValueOrDefault<Str>
    },
    TransactionSnapshot(Box<str>),
    TimeZone(ZoneValue),
    Catalog(Box<str>),
    Schema(Box<str>),
    /// Alias: `Names`
    ClientEncoding(ValueOrDefault<Box<str>>),
    Role(Str),
    XmlOption(XmlNodeKind),
    FromCurrent {
        name: QualifiedName
    },
    ConfigurationParameter {
        name: QualifiedName,
        value: ValueOrDefault<Vec<VarValue>>
    }
}

impl From<SetRestMore> for SetRest {
    fn from(value: SetRestMore) -> Self {
        match value {
            SetRestMore::SessionAuthorization { user } => Self::SessionAuthorization { user },
            SetRestMore::TransactionSnapshot(id) => Self::TransactionSnapshot(id),
            SetRestMore::TimeZone(zone_value) => Self::TimeZone(zone_value),
            SetRestMore::Catalog(name) => Self::Catalog(name),
            SetRestMore::Schema(name) => Self::Schema(name),
            SetRestMore::ClientEncoding(name) => Self::ClientEncoding(name),
            SetRestMore::Role(name) => Self::Role(name),
            SetRestMore::XmlOption(option) => Self::XmlOption(option),
            SetRestMore::FromCurrent { name } => Self::FromCurrent { name },
            SetRestMore::ConfigurationParameter { name, value } => Self::ConfigurationParameter { name, value },
        }
    }
}

use crate::parser::ast_node::QualifiedName;
use crate::parser::ast_node::SetRestMore;
use crate::parser::ast_node::TransactionMode;
use crate::parser::ast_node::ValueOrDefault;
use crate::parser::ast_node::VarValue;
use crate::parser::ast_node::XmlNodeKind;
use crate::parser::ast_node::ZoneValue;
use postgres_basics::Str;
