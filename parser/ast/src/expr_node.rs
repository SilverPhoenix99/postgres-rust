#[derive(Debug, Clone, Eq, PartialEq, From)]
pub enum ExprNode {
    /*
        Constants/Literals
    */

    NullConst,
    StringConst(Box<str>),
    BinaryStringConst(Box<str>),
    HexStringConst(Box<str>),
    IntegerConst(i32),
    #[from] NumericConst(Number),
    BooleanConst(bool),

    /*
        Boolean test expressions
    */

    /// * `expr IS NULL`
    /// * `expr ISNULL`
    IsNull(Box<ExprNode>),

    /// * `expr IS NOT NULL`
    /// * `expr NOTNULL`
    IsNotNull(Box<ExprNode>),

    /// `expr IS TRUE`
    IsTrue(Box<ExprNode>),

    /// `expr IS NOT TRUE`
    IsNotTrue(Box<ExprNode>),

    /// `expr IS FALSE`
    IsFalse(Box<ExprNode>),

    /// `expr IS NOT FALSE`
    IsNotFalse(Box<ExprNode>),

    /// `expr IS UNKNOWN`
    IsUnknown(Box<ExprNode>),

    /// `expr IS NOT UNKNOWN`
    IsNotUnknown(Box<ExprNode>),

    /// `expr IS DISTINCT FROM expr`
    IsDistinct(BinaryOperands),

    /// `expr IS NOT DISTINCT FROM expr`
    IsNotDistinct(BinaryOperands),

    /// `expr IS DOCUMENT`
    IsDocument(Box<ExprNode>),

    /// `expr IS NORMALIZED`
    IsNormalized(Box<ExprNode>, Option<UnicodeNormalForm>),

    #[from(JsonIsPredicate)]
    IsJson(Box<JsonIsPredicate>),

    /// `expr IN ( expr... )`
    InArray(Box<ExprNode>, Vec<ExprNode>),

    /// `expr IN ( select_stmt )`
    InSubquery(Box<ExprNode>, SelectStmt),

    /*
        Other Expressions
    */

    DefaultExpr,
    #[from(CaseExpr)]
    CaseExpr(Box<CaseExpr>),
    ParamRef { index: i32 },
    #[from]
    Row(RowExpr),
    #[from(RowOverlaps)]
    RowOverlaps(Box<RowOverlaps>),
    Array(Option<Vec<ExprNode>>),

    #[from(BinaryExpr)]
    BinaryExpr(Box<BinaryExpr>),
    #[from(UnaryExpr)]
    UnaryExpr(Box<UnaryExpr>),
    #[from]
    BoolExpr(BoolExpr),

    #[from(JsonArrayAggExpr)]
    JsonArrayAggExpr(Box<JsonArrayAggExpr>),
    #[from(JsonObjectAggExpr)]
    JsonObjectAggExpr(Box<JsonObjectAggExpr>),

    #[from(IndirectionExpr)]
    Indirection(Box<IndirectionExpr>),

    #[from]
    ColumnRef(ColumnRef),

    #[from]
    Select(SelectStmt),

    /// EXISTS ( subquery )
    Exists(SelectStmt),

    /*
        Function calls
    */

    /// `GROUPING '(' expr_list ')'`
    GroupingFunc(Vec<ExprNode>),

    /// Generic function call.
    #[from(FuncCall)]
    FuncCall(Box<FuncCall>),

    /// Function call with extra clauses.
    #[from(FuncCallExpr)]
    FuncCallExpr(Box<FuncCallExpr>),

    /// Function calls that might not fit the normal function call pattern,
    /// usually associated with a predefined PG function.
    #[from(SqlFunction)]
    SqlFunction(Box<SqlFunction>),

    /// `expr COLLATE collation`
    #[from(CollationExpr)]
    Collate(Box<CollationExpr>),

    /// * `lhs AT TIME ZONE zone`
    /// * `lhs AT LOCAL`
    #[from(TimezoneExpr)]
    Timezone(Box<TimezoneExpr>),
}

impl From<UnsignedNumber> for ExprNode {
    fn from(value: UnsignedNumber) -> Self {
        SignedNumber::from(value).into()
    }
}

impl From<SignedNumber> for ExprNode {
    fn from(value: SignedNumber) -> Self {
        match value {
            SignedNumber::IntegerConst(int) => Self::IntegerConst(int),
            SignedNumber::NumericConst(number) => Self::NumericConst(number),
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
use crate::CollationExpr;
use crate::ColumnRef;
use crate::FuncCall;
use crate::FuncCallExpr;
use crate::IndirectionExpr;
use crate::JsonArrayAggExpr;
use crate::JsonIsPredicate;
use crate::JsonObjectAggExpr;
use crate::Number;
use crate::RowExpr;
use crate::RowOverlaps;
use crate::SelectStmt;
use crate::SignedNumber;
use crate::SqlFunction;
use crate::SqlFunction::Typecast;
use crate::TimezoneExpr;
use crate::TypecastExpr;
use crate::UnaryExpr;
use crate::UnicodeNormalForm;
use derive_more::From;
use pg_basics::UnsignedNumber;
