pub type TypeModifiers = Vec<ExprNode>;

#[derive(Debug, Clone, PartialEq)]
pub struct SystemType {
    name: TypeName,
    array_bounds: Vec<Option<i32>>,
    /// If the type is a table (i.e., set) of records
    mult: SetOf
}

impl SystemType {
    pub fn new(name: TypeName, array_bounds: Vec<Option<i32>>, mult: SetOf) -> Self {
        Self { name, array_bounds, mult }
    }

    pub fn with_array_bounds(self, array_bounds: Vec<Option<i32>>) -> Self {
        Self::new(self.name, array_bounds, self.mult)
    }

    pub fn returning_table(self) -> SystemType {
        Self::new(self.name, self.array_bounds, SetOf::Table)
    }

    pub fn name(&self) -> &TypeName {
        &self.name
    }

    pub fn array_bounds(&self) -> &Vec<Option<i32>> {
        &self.array_bounds
    }

    pub fn mult(&self) -> SetOf {
        self.mult
    }
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
    HourToSecond { precision: Option<i32> },
    Minute,
    MinuteToSecond { precision: Option<i32> },
    Second { precision: Option<i32> },
}

impl Default for IntervalRange {
    fn default() -> Self {
        Self::Full { precision: None }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericTypeName {
    name: QualifiedName,
    type_modifiers: TypeModifiers
}

impl GenericTypeName {
    pub fn new(name: QualifiedName, type_modifiers: TypeModifiers) -> Self {
        Self { name, type_modifiers }
    }

    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    pub fn type_modifiers(&self) -> &Vec<ExprNode> {
        &self.type_modifiers
    }
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
    Numeric(TypeModifiers),
    /// Blank-Padded Character string
    Bpchar {
        length: Option<i32>
    },
    // See https://www.postgresql.org/docs/current/datatype-character.html
    Varchar {
        /// If limited, the maximum is 10MB == 10,485,760.
        max_length: Option<i32>
    },
    Bit(TypeModifiers),
    Varbit(TypeModifiers),
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
    Interval(IntervalRange),
    Oid(Oid),
    /// Non-builtin types
    Generic(GenericTypeName),
}

impl_from!(IntervalRange for TypeName => Interval);
impl_from!(GenericTypeName for TypeName => Generic);

impl TypeName {
    pub fn with_array_bounds(self, array_bounds: Vec<Option<i32>>) -> SystemType {
        SystemType::from(self).with_array_bounds(array_bounds)
    }

    pub fn returning_table(self) -> SystemType {
        SystemType::from(self).returning_table()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SetOf {
    Scalar,
    Table,
}

impl From<TypeName> for SystemType {
    fn from(value: TypeName) -> Self {
        SystemType::new(value, Vec::new(), SetOf::Scalar)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FuncArgClass {
    In,
    Out,
    InOut,
    Variadic
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypeOf {
    field: QualifiedName,
    mult: SetOf
}

impl TypeOf {
    pub fn new(field: QualifiedName, mult: SetOf) -> Self {
        Self { field, mult }
    }

    pub fn field(&self) -> &QualifiedName {
        &self.field
    }

    pub fn mult(&self) -> SetOf {
        self.mult
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FuncType {
    System(SystemType),
    /// When the type is specified with `%TYPE`.
    TypeOf(TypeOf),
}

use crate::parser::{
    ast_node::{impl_from, QualifiedName},
    ExprNode
};
use postgres_basics::Oid;
