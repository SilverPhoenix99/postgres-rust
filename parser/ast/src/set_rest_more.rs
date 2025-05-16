#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SetRestMore {
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

use crate::ValueOrDefault;
use crate::VarValue;
use crate::XmlNodeKind;
use crate::ZoneValue;
use pg_basics::QualifiedName;
use pg_basics::Str;
