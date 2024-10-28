mod alter_event_trig_stmt;
mod alter_role_stmt;
mod ast_literal;
mod discard_stmt;
mod notify_stmt;
mod numeric_spec;
mod reassign_owned_stmt;
mod role_spec;
mod system_type;
mod transaction_stmt;
mod variable_show_stmt;

pub(super) use self::system_type::CharacterSystemType;
pub use self::{
    alter_event_trig_stmt::{AlterEventTrigStmt, EventTriggerState},
    alter_role_stmt::{AlterRoleAction, AlterRoleOption, AlterRoleStmt},
    ast_literal::AstLiteral,
    discard_stmt::DiscardStmt,
    notify_stmt::NotifyStmt,
    numeric_spec::NumericSpec,
    reassign_owned_stmt::ReassignOwnedStmt,
    role_spec::RoleSpec,
    system_type::SystemType,
    transaction_stmt::{IsolationLevel, TransactionMode, TransactionStmt},
    variable_show_stmt::VariableShowStmt,
};

/// Generates `From` impls, where the input is wrapped in an output enum variant.
macro_rules! impl_from {
    ($variant:ident for $for_:ident) => {
        impl_from!($variant for $for_ => $variant);
    };
    (box $variant:ident for $for_:ident) => {
        impl_from!(box $variant for $for_ => $variant);
    };
    ($from:ident for $for_:ident => $variant:ident) => {
        impl From<$from> for $for_ {
            #[inline(always)]
            fn from(value: $from) -> Self {
                Self::$variant(value)
            }
        }
    };
    (box $from:ident for $for_:ident => $variant:ident) => {
        impl From<$from> for $for_ {
            #[inline(always)]
            fn from(value: $from) -> Self {
                Self::$variant(Box::new(value))
            }
        }
    };
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OneOrAll {
    All,
    Name(CowStr),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    Exponentiation,
    Less,
    Greater,
    Equals,
    LessEquals,
    GreaterEquals,
    NotEquals,
    Like,
    NotLike,
    ILike,
    NotILike,
    UserDefined(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QnOperator(pub Vec<CowStr>, pub Operator);

impl From<Operator> for QnOperator {
    fn from(value: Operator) -> Self {
        Self(vec![], value)
    }
}

type QnName = Vec<CowStr>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AggregateWithArgtypes {
    // TODO
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionWithArgtypes {
    // TODO
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OperatorWithArgtypes {
    // TODO
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RelationExpr {
    // TODO
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RenameTarget {
    Aggregate(AggregateWithArgtypes),
    Collation(QnName),
    Conversion(QnName),
    Database(CowStr),
    Domain(QnName),
    DomainConstraint { domain: QnName, constraint: CowStr },
    EventTrigger(CowStr),
    ForeignDataWrapper(CowStr),
    ForeignServer(CowStr),
    ForeignTable { target: RelationExpr, missing_ok: bool },
    ForeignTableColumn { table: RelationExpr, column: CowStr, missing_ok: bool },
    Function(FunctionWithArgtypes),
    Index { target: QnName, missing_ok: bool },
    Language(CowStr),
    MaterializedView { target: QnName, missing_ok: bool },
    MaterializedViewColumn { view: QnName, column: QnName, missing_ok: bool },
    OperatorClass(QnName),
    OperatorFamily(QnName),
    Policy { table: QnName, policy: CowStr, missing_ok: bool },
    Procedure(FunctionWithArgtypes),
    Publication(CowStr),
    /// Aliases:
    /// * `Group`
    /// * `User`
    Role(CowStr),
    Routine(FunctionWithArgtypes),
    Rule { relation: QnName, rule: CowStr },
    Schema(CowStr),
    Sequence { target: QnName, missing_ok: bool },
    Statistic(QnName),
    Subscription(CowStr),
    Table { target: RelationExpr, missing_ok: bool },
    TableColumn { table: RelationExpr, column: CowStr, missing_ok: bool },
    TableConstraint { table: RelationExpr, constraint: CowStr, missing_ok: bool },
    Tablespace(CowStr),
    TextSearchConfiguration(QnName),
    TextSearchDictionary(QnName),
    TextSearchParser(QnName),
    TextSearchTemplate(QnName),
    Trigger { table: QnName, trigger: CowStr },
    Type(QnName),
    TypeAttribute { typ: QnName, attribute: CowStr },
    View { target: QnName, missing_ok: bool },
    ViewColumn { view: QnName, column: CowStr, missing_ok: bool },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RenameStmt {
    target: RenameTarget,
    new_name: CowStr,
}

impl RenameStmt {
    #[inline(always)]
    pub fn new(target: RenameTarget, new_name: CowStr) -> Self {
        Self { target, new_name }
    }

    pub fn target(&self) -> &RenameTarget {
        &self.target
    }

    pub fn new_name(&self) -> &CowStr {
        &self.new_name
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnsignedNumber {
    IConst(u32),
    Numeric { value: String, radix: u32 },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SignedNumber {
    SignedIConst(i32),
    Numeric { value: String, radix: u32, negative: bool },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AlterOwnerTarget {
    Aggregate(AggregateWithArgtypes),
    Collation(QnName),
    Conversion(QnName),
    Database(CowStr),
    Domain(QnName),
    EventTrigger(CowStr),
    ForeignDataWrapper(CowStr),
    ForeignServer(CowStr),
    Function(FunctionWithArgtypes),
    Language(CowStr),
    LargeObject(SignedNumber),
    Operator(OperatorWithArgtypes),
    OperatorClass(QnName),
    OperatorFamily(QnName),
    Procedure(FunctionWithArgtypes),
    Publication(CowStr),
    Routine(FunctionWithArgtypes),
    Schema(CowStr),
    Statistic(QnName),
    Subscription(CowStr),
    Tablespace(CowStr),
    TextSearchConfiguration(QnName),
    TextSearchDictionary(QnName),
    Type(QnName),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterOwnerStmt {
    target: AlterOwnerTarget,
    new_owner: RoleSpec,
}

impl AlterOwnerStmt {
    #[inline(always)]
    pub fn new(target: AlterOwnerTarget, new_owner: RoleSpec) -> Self {
        Self { target, new_owner }
    }

    #[inline(always)]
    pub fn target(&self) -> &AlterOwnerTarget {
        &self.target
    }

    #[inline(always)]
    pub fn new_owner(&self) -> &RoleSpec {
        &self.new_owner
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AlterObjectSchemaTarget {
    Aggregate(AggregateWithArgtypes),
    Collation(QnName),
    Conversion(QnName),
    Domain(QnName),
    Extension(CowStr),
    ForeignTable { target: RelationExpr, missing_ok: bool },
    Function(FunctionWithArgtypes),
    MaterializedView { target: QnName, missing_ok: bool },
    Operator(OperatorWithArgtypes),
    OperatorClass(QnName),
    OperatorFamily(QnName),
    Procedure(FunctionWithArgtypes),
    Routine(FunctionWithArgtypes),
    Sequence { target: QnName, missing_ok: bool },
    Statistic(QnName),
    Table { target: RelationExpr, missing_ok: bool },
    TextSearchConfiguration(QnName),
    TextSearchDictionary(QnName),
    TextSearchParser(QnName),
    TextSearchTemplate(QnName),
    Type(QnName),
    View { target: QnName, missing_ok: bool },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterObjectSchemaStmt {
    target: AlterObjectSchemaTarget,
    new_schema: CowStr,
}

impl AlterObjectSchemaStmt {
    #[inline(always)]
    pub fn new(target: AlterObjectSchemaTarget, new_schema: CowStr) -> Self {
        Self { target, new_schema }
    }

    pub fn target(&self) -> &AlterObjectSchemaTarget {
        &self.target
    }

    pub fn new_schema(&self) -> &CowStr {
        &self.new_schema
    }
}

pub type BinaryOperands = Box<(ExprNode, ExprNode)>;

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    operator: QnOperator,
    left_operand: ExprNode,
    right_operand: ExprNode,
}

impl BinaryExpr {
    pub fn new(operator: QnOperator, left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self { operator, left_operand, right_operand, }
    }

    pub fn addition(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::Addition.into(), left_operand, right_operand)
    }

    pub fn subtraction(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::Subtraction.into(), left_operand, right_operand)
    }

    pub fn multiplication(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::Multiplication.into(), left_operand, right_operand)
    }

    pub fn division(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::Division.into(), left_operand, right_operand)
    }

    pub fn modulo(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::Modulo.into(), left_operand, right_operand)
    }

    pub fn exponentiation(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::Exponentiation.into(), left_operand, right_operand)
    }

    pub fn less(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::Less.into(), left_operand, right_operand)
    }

    pub fn greater(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::Greater.into(), left_operand, right_operand)
    }

    pub fn equals(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::Equals.into(), left_operand, right_operand)
    }

    pub fn less_equals(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::LessEquals.into(), left_operand, right_operand)
    }

    pub fn greater_equals(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::GreaterEquals.into(), left_operand, right_operand)
    }

    pub fn not_equals(left_operand: ExprNode, right_operand: ExprNode) -> Self {
        Self::new(Operator::NotEquals.into(), left_operand, right_operand)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr {
    operator: QnOperator,
    operand: ExprNode,
}

impl UnaryExpr {
    pub fn new(operator: QnOperator, operand: ExprNode) -> Self {
        Self { operator, operand }
    }

    pub fn unary_plus(operand: ExprNode) -> Self {
        Self::new(Operator::Addition.into(), operand)
    }

    /// Aka `unary_minus`
    pub fn negation(operand: ExprNode) -> Self {
        Self::new(Operator::Subtraction.into(), operand)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum XmlNodeKind {
    Document,
    Content,
}

impl XmlNodeKind {
    #[inline(always)]
    pub fn is_document(&self) -> bool {
        *self == Self::Document
    }

    #[inline(always)]
    pub fn is_content(&self) -> bool {
        *self == Self::Content
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum XmlStandalone {
    Yes,
    No,
    NoValue,
    Omitted,
}

#[derive(Debug, Clone, PartialEq)]
pub struct XmlElement {
    name: CowStr,
    attributes: Vec<ExprNode>,
    args: Vec<ExprNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct XmlParse {
    text: ExprNode,
    kind: XmlNodeKind,
    preserve_whitespace: bool
}

#[derive(Debug, Clone, PartialEq)]
pub struct XmlProcessingInstruction {
    name: CowStr,
    args: Option<ExprNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct XmlRoot {
    version: Option<ExprNode>,
    standalone: XmlStandalone,
    xml: ExprNode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Typename {
    // TODO
}

#[derive(Debug, Clone, PartialEq)]
pub struct XmlSerialize {
    kind: XmlNodeKind,
    x: ExprNode,
    type_name: Typename,
    indent: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrepareStmt {
    name: CowStr,
    arg_types: Vec<Typename>,
    query: RawStmt,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RawStmt {
    AlterEventTrigStmt(AlterEventTrigStmt),
    AlterObjectSchemaStmt(AlterObjectSchemaStmt),
    AlterOwnerStmt(AlterOwnerStmt),
    AlterRoleStmt(AlterRoleStmt),
    CheckPoint,
    ClosePortalStmt(OneOrAll),
    DeallocateStmt(OneOrAll),
    DiscardStmt(DiscardStmt),
    ListenStmt(CowStr),
    LoadStmt(String),
    NotifyStmt(NotifyStmt),
    PrepareStmt(Box<PrepareStmt>),
    PrepareTransactionStmt(String),
    ReassignOwnedStmt(ReassignOwnedStmt),
    RefreshCollationVersionStmt(QnName),
    RenameStmt(RenameStmt),
    TransactionStmt(TransactionStmt),
    UnlistenStmt(OneOrAll),
    VariableShowStmt(VariableShowStmt),
}

impl_from!(AlterEventTrigStmt for RawStmt);
impl_from!(AlterObjectSchemaStmt for RawStmt);
impl_from!(AlterOwnerStmt for RawStmt);
impl_from!(AlterRoleStmt for RawStmt);
impl_from!(DiscardStmt for RawStmt);
impl_from!(NotifyStmt for RawStmt);
impl_from!(box PrepareStmt for RawStmt);
impl_from!(ReassignOwnedStmt for RawStmt);
impl_from!(RenameStmt for RawStmt);
impl_from!(TransactionStmt for RawStmt);
impl_from!(VariableShowStmt for RawStmt);

#[derive(Debug, Clone, PartialEq)]
pub enum BoolExpr {
    Not(Box<ExprNode>),
    Or(BinaryOperands),
    And(BinaryOperands)
}

impl BoolExpr {
    pub fn not(expr: ExprNode) -> Self {
        Self::Not(Box::new(expr))
    }

    pub fn or(left: ExprNode, right: ExprNode) -> Self {
        Self::Or(Box::new((left, right)))
    }

    pub fn and(left: ExprNode, right: ExprNode) -> Self {
        Self::And(Box::new((left, right)))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Indirection {
    /// `.*`
    All,
    /// `.ColLabel`
    Property(CowStr),
    /// `[expr]`
    Index(ExprNode),
    /// `[:]`
    FullSlice,
    /// `[ expr : ]`
    SliceFrom(ExprNode),
    /// `[ : expr ]`
    SliceTo(ExprNode),
    /// `[ expr : expr ]`
    Slice(ExprNode, ExprNode),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprNode {
    SetToDefault,
    Literal(AstLiteral),
    SystemType(SystemType),
    Typecast((/* TODO */)),
    Indirection(Vec<Indirection>),

    BinaryExpr(Box<BinaryExpr>),
    UnaryExpr(Box<UnaryExpr>),
    /// `IS DISTINCT FROM`
    Distinct(BinaryOperands),
    /// `IS NOT DISTINCT FROM`
    NotDistinct(BinaryOperands),
    BoolExpr(BoolExpr),

    /* Xml operations */
    IsXmlDocument(Box<ExprNode>),
    XmlConcat(Vec<ExprNode>),
    XmlElement(XmlElement),
    XmlForest(Vec<ExprNode>),
    XmlParse(Box<XmlParse>),
    XmlProcessingInstruction(Box<XmlProcessingInstruction>),
    XmlRoot(Box<XmlRoot>),
}

impl ExprNode {
    #[inline(always)]
    pub fn addition(left: Self, right: Self) -> Self {
        BinaryExpr::addition(left, right).into()
    }

    #[inline(always)]
    pub fn unary_plus(operand: Self) -> Self {
        UnaryExpr::new(Operator::Addition.into(), operand).into()
    }

    #[inline(always)]
    pub fn subtraction(left: Self, right: Self) -> Self {
        BinaryExpr::subtraction(left, right).into()
    }

    /// Aka `unary_minus`
    #[inline(always)]
    pub fn negation(operand: Self) -> Self {
        UnaryExpr::new(Operator::Subtraction.into(), operand).into()
    }

    #[inline(always)]
    pub fn multiplication(left: Self, right: Self) -> Self {
        BinaryExpr::multiplication(left, right).into()
    }

    #[inline(always)]
    pub fn division(left: Self, right: Self) -> Self {
        BinaryExpr::division(left, right).into()
    }

    #[inline(always)]
    pub fn modulo(left: Self, right: Self) -> Self {
        BinaryExpr::modulo(left, right).into()
    }

    #[inline(always)]
    pub fn exponentiation(left: Self, right: Self) -> Self {
        BinaryExpr::exponentiation(left, right).into()
    }

    #[inline(always)]
    pub fn less(left: Self, right: Self) -> Self {
        BinaryExpr::less(left, right).into()
    }

    #[inline(always)]
    pub fn greater(left: Self, right: Self) -> Self {
        BinaryExpr::greater(left, right).into()
    }

    #[inline(always)]
    pub fn equals(left: Self, right: Self) -> Self {
        BinaryExpr::equals(left, right).into()
    }

    #[inline(always)]
    pub fn greater_equals(left: Self, right: Self) -> Self {
        BinaryExpr::greater_equals(left, right).into()
    }

    #[inline(always)]
    pub fn less_equals(left: Self, right: Self) -> Self {
        BinaryExpr::less_equals(left, right).into()
    }

    #[inline(always)]
    pub fn not_equals(left: Self, right: Self) -> Self {
        BinaryExpr::not_equals(left, right).into()
    }

    #[inline(always)]
    pub fn distinct(left: Self, right: Self) -> Self {
        Self::Distinct(Box::new((left, right)))
    }

    #[inline(always)]
    pub fn not_distinct(left: Self, right: Self) -> Self {
        Self::NotDistinct(Box::new((left, right)))
    }

    #[inline(always)]
    pub fn is_xml_document(operand: Self) -> Self {
        Self::IsXmlDocument(Box::new(operand))
    }

    #[inline(always)]
    pub fn not(expr: Self) -> Self {
        BoolExpr::not(expr).into()
    }

    #[inline(always)]
    pub fn or(left: Self, right: Self) -> Self {
        BoolExpr::or(left, right).into()
    }

    #[inline(always)]
    pub fn and(left: Self, right: Self) -> Self {
        BoolExpr::and(left, right).into()
    }
}

impl_from!(box BinaryExpr for ExprNode);
impl_from!(box UnaryExpr for ExprNode);
impl_from!(box XmlParse for ExprNode);
impl_from!(box XmlProcessingInstruction for ExprNode);
impl_from!(box XmlRoot for ExprNode);
impl_from!(AstLiteral for ExprNode => Literal);
impl_from!(BoolExpr for ExprNode);
impl_from!(SystemType for ExprNode);
impl_from!(XmlElement for ExprNode);

use crate::parser::CowStr;
