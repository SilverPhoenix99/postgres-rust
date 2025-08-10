pub type TypeModifiers = Vec<ExprNode>;

#[derive(Debug, Clone, Eq, PartialEq, Into)]
#[into((TypeName, SetOf, Option<Vec<Option<i32>>>))]
pub struct Type {
    name: TypeName,
    /// If the type is a table (i.e., set) of records, or just a single record.
    mult: SetOf,
    array_bounds: Option<Vec<Option<i32>>>,
}

impl Type {
    pub fn new(name: TypeName, array_bounds: Option<Vec<Option<i32>>>, mult: SetOf) -> Self {
        Self { name, array_bounds, mult }
    }

    pub fn with_array_bounds(self, array_bounds: Option<Vec<Option<i32>>>) -> Self {
        Self::new(self.name, array_bounds, self.mult)
    }

    pub fn returning_table(self) -> Type {
        Self::new(self.name, self.array_bounds, SetOf::Table)
    }

    pub fn name(&self) -> &TypeName {
        &self.name
    }

    pub fn array_bounds(&self) -> Option<&[Option<i32>]> {
        self.array_bounds.as_deref()
    }

    pub fn mult(&self) -> SetOf {
        self.mult
    }
}

#[derive(Debug, Clone, Eq, PartialEq, From)]
pub enum TypeName {
    Json,
    Bool,
    Int2,
    Int4,
    Int8,
    Float4,
    Float8,
    Numeric(Option<TypeModifiers>),
    /// Blank-Padded Character string
    Bpchar {
        length: Option<i32>
    },
    // See https://www.postgresql.org/docs/current/datatype-character.html
    Varchar {
        /// If limited, the maximum is 10MB == 10,485,760.
        max_length: Option<i32>
    },
    Bit(Option<TypeModifiers>),
    Varbit(Option<TypeModifiers>),
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
    #[from] Interval(IntervalRange),
    Oid(Oid),
    /// Non-builtin types
    Generic {
        name: QualifiedName,
        type_modifiers: Option<TypeModifiers>
    },
}

impl TypeName {
    pub fn with_array_bounds(self, array_bounds: Option<Vec<Option<i32>>>) -> Type {
        Type::from(self).with_array_bounds(array_bounds)
    }

    pub fn returning_table(self) -> Type {
        Type::from(self).returning_table()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SetOf {
    /// When the type represents a single record, or scalar
    Record,
    /// When the type represents a set of records (i.e., a table).
    Table,
}

impl From<TypeName> for Type {
    fn from(value: TypeName) -> Self {
        Type::new(value, None, SetOf::Record)
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum FunctionParameterMode {
    #[default]
    Default  = b'd' as isize,
    In       = b'i' as isize,
    Out      = b'o' as isize,
    InOut    = b'b' as isize,
    Variadic = b'v' as isize,
    Table    = b't' as isize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypeReference {
    field: QualifiedName,
    mult: SetOf
}

impl TypeReference {
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FuncType {
    Type(Type),
    /// When the type is specified with `%TYPE`.
    Reference(TypeReference),
}

use crate::ExprNode;
use derive_more::From;
use derive_more::Into;
use pg_basics::Oid;
use pg_basics::QualifiedName;
use pg_interval_ast::IntervalRange;
