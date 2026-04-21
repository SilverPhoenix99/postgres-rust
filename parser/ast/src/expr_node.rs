#[derive(Debug, Clone, Eq, PartialEq, From)]
pub enum ExprNode {
    /* Constants */
    NullConst,
    StringConst(Box<str>),
    BinaryStringConst(Box<str>),
    HexStringConst(Box<str>),
    IntegerConst(i32),
    NumericConst {
        value: Box<str>,
        radix: NumberRadix,
        negative: bool
    },
    BooleanConst(bool),

    DefaultExpr,
    #[from(CaseExpr)]
    CaseExpr(Box<CaseExpr>),
    ParamRef { index: i32 },
    Row(Option<Vec<ExprNode>>),
    Array(Option<Vec<ExprNode>>),

    #[from(BinaryExpr)]
    BinaryExpr(Box<BinaryExpr>),
    #[from(UnaryExpr)]
    UnaryExpr(Box<UnaryExpr>),
    #[from]
    BoolExpr(BoolExpr),
    #[from(FuncCallExpr)]
    FuncCallExpr(Box<FuncCallExpr>),
    #[from(JsonArrayAggExpr)]
    JsonArrayAggExpr(Box<JsonArrayAggExpr>),
    #[from(JsonObjectAggExpr)]
    JsonObjectAggExpr(Box<JsonObjectAggExpr>),

    /// `IS DISTINCT FROM`
    IsDistinct(BinaryOperands),
    /// `IS NOT DISTINCT FROM`
    IsNotDistinct(BinaryOperands),

    /// `IS DOCUMENT`
    IsDocument(Box<ExprNode>),
    /// `IS NOT DOCUMENT`
    IsNotDocument(Box<ExprNode>),

    #[from(IndirectionExpr)]
    Indirection(Box<IndirectionExpr>),
    #[from]
    ColumnRef(ColumnRef),

    /* Function calls */
    GroupingFunc(Vec<ExprNode>),
    #[from(FuncCall)]
    FuncCall(Box<FuncCall>),
    #[from(SqlFunction)]
    SqlFunction(Box<SqlFunction>),

    #[from]
    Select(SelectStmt),
}

impl From<UnsignedNumber> for ExprNode {
    fn from(value: UnsignedNumber) -> Self {
        match value {
            // SAFETY: `int` is originally parsed by `i32::from_str_radix()`, so `0 <= int <= i32::MAX`
            UnsignedNumber::IntegerConst(int) => Self::IntegerConst(int.into()),
            UnsignedNumber::NumericConst { value, radix } => Self::NumericConst {
                radix,
                value,
                negative: false,
            }
        }
    }
}

impl From<SignedNumber> for ExprNode {
    fn from(value: SignedNumber) -> Self {
        match value {
            SignedNumber::IntegerConst(int) => Self::IntegerConst(int),
            SignedNumber::NumericConst { value, radix, negative } => Self::NumericConst {
                radix,
                value,
                negative,
            }
        }
    }
}

impl From<TypecastExpr> for ExprNode {
    fn from(value: TypecastExpr) -> Self {
        Self::SqlFunction(Typecast(value).into())
    }
}

use crate::BinaryExpr;
use crate::BinaryOperands;
use crate::BoolExpr;
use crate::CaseExpr;
use crate::ColumnRef;
use crate::FuncCall;
use crate::FuncCallExpr;
use crate::IndirectionExpr;
use crate::JsonArrayAggExpr;
use crate::JsonObjectAggExpr;
use crate::SelectStmt;
use crate::SignedNumber;
use crate::SqlFunction;
use crate::SqlFunction::Typecast;
use crate::TypecastExpr;
use crate::UnaryExpr;
use derive_more::From;
use pg_basics::NumberRadix;
use pg_basics::UnsignedNumber;
