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
    pub fn new(name: TypeName) -> Self {
        Self {
            name,
            mult: Default::default(),
            array_bounds: Default::default(),
        }
    }

    pub fn name(&self) -> &TypeName {
        &self.name
    }

    pub fn with_array_bounds(mut self, array_bounds: Vec<Option<i32>>) -> Self {
        self.array_bounds = if array_bounds.is_empty() { None } else { Some(array_bounds) };
        self
    }

    pub fn set_array_bounds(&mut self, array_bounds: Option<Vec<Option<i32>>>) -> &mut Self {
        self.array_bounds = array_bounds;
        self
    }

    pub fn array_bounds(&self) -> Option<&[Option<i32>]> {
        self.array_bounds.as_deref()
    }

    pub fn with_mult(mut self, mult: SetOf) -> Self {
        self.mult = mult;
        self
    }

    pub fn set_mult(&mut self, mult: SetOf) -> &mut Self {
        self.mult = mult;
        self
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

impl From<TypeName> for Type {
    fn from(value: TypeName) -> Self {
        Type::new(value)
    }
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum SetOf {
    /// When the type represents a single record, or scalar
    #[default]
    Record,
    /// When the type represents a set of records (i.e., a table).
    Table,
}

impl From<bool> for SetOf {
    fn from(value: bool) -> Self {
        if value {
            SetOf::Table
        } else {
            SetOf::Record
        }
    }
}

impl From<SetOf> for bool {
    fn from(value: SetOf) -> Self {
        value == SetOf::Table
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

#[derive(Debug, Clone, Eq, PartialEq, From)]
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
