/// Redundant enum, to avoid using `unreachable!()`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::parser) enum CharacterSystemType {
    Varchar,
    Bpchar
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IntervalRange {
    Full { precision: Option<i32> },
    Year,
    YearToMonth,
    Month,
    Day,
    DayToHour,
    DayToMinute,
    DayToSecond { precision: Option<i32> },
    Hour,
    HourToMinute,
    HourToToSecond { precision: Option<i32> },
    Minute,
    MinuteToSecond { precision: Option<i32> },
    Second { precision: Option<i32> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeName {
    Json,
    Bool,
    Int2,
    Int4,
    Int8,
    Float4,
    Float8,
    Numeric {
        type_modifiers: Vec<ExprNode>
    },
    /// Blank-Padded Character string
    Bpchar {
        length: Option<i32>
    },
    // See https://www.postgresql.org/docs/current/datatype-character.html
    Varchar {
        /// If limited, the maximum is 10MB == 10,485,760.
        max_length: Option<i32>
    },
    Bit {
        type_modifiers: Vec<ExprNode>
    },
    Varbit {
        type_modifiers: Vec<ExprNode>
    },
    Time {
        precision: Option<i32>
    },
    TimeTz {
        precision: Option<i32>
    },
    Timestamp {
        precision: Option<i32>
    },
    TimestampTz {
        precision: Option<i32>
    },
    Interval {
        range: IntervalRange
    },
    /// Non-builtin types
    Generic {
        name: QualifiedName,
        type_modifiers: Vec<ExprNode>
    },
    /// When the type is specified with `%TYPE`.
    TypeOf {
        field: QualifiedName
    },
    Oid(Oid),
}

impl TypeName {
    pub fn with_array_bounds(self, array_bounds: Vec<i32>) -> SystemType {
        SystemType::from(self).with_array_bounds(array_bounds)
    }

    pub fn into_set(self) -> SystemType {
        SystemType::from(self).into_set()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SystemType {
    name: TypeName,
    array_bounds: Vec<i32>,
    /// If the type is a table (i.e., set) of records
    set_of: bool
}

impl SystemType {
    pub fn new(name: TypeName, array_bounds: Vec<i32>, set_of: bool) -> Self {
        Self { name, array_bounds, set_of }
    }

    pub fn with_array_bounds(self, array_bounds: Vec<i32>) -> Self {
        Self::new(self.name, array_bounds, self.set_of)
    }

    pub fn into_set(self) -> SystemType {
        Self::new(self.name, self.array_bounds, true)
    }

    pub fn name(&self) -> &TypeName {
        &self.name
    }

    pub fn array_bounds(&self) -> &Vec<i32> {
        &self.array_bounds
    }

    pub fn set_of(&self) -> bool {
        self.set_of
    }
}

impl From<TypeName> for SystemType {
    fn from(value: TypeName) -> Self {
        SystemType::new(value, Vec::new(), false)
    }
}

use crate::parser::{
    ast_node::QualifiedName,
    ExprNode
};
use postgres_basics::Oid;
