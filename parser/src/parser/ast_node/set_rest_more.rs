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

use crate::parser::ast_node::QualifiedName;
use crate::parser::ast_node::ValueOrDefault;
use crate::parser::ast_node::VarValue;
use crate::parser::ast_node::XmlNodeKind;
use crate::parser::ast_node::ZoneValue;
use postgres_basics::Str;
