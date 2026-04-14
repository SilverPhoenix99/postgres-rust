#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Operator {
    /// `+`
    Addition,
    /// `-`
    Subtraction,
    /// `*`
    Multiplication,
    /// `/`
    Division,
    /// `%`
    Modulo,
    /// `^`
    Exponentiation,
    /// `<`
    Less,
    /// `>`
    Greater,
    /// `=`
    Equals,
    /// `<=`
    LessEquals,
    /// `>=`
    GreaterEquals,
    /// `!=` or `<>`
    NotEquals,
    /// `LIKE`
    Like,
    /// `NOT LIKE`
    NotLike,
    /// `ILIKE`
    ILike,
    /// `NOT ILIKE`
    NotILike,
    /// `->`
    RightArrow,
    /// `|`
    Pipe,
    UserDefined(Box<str>),
}
