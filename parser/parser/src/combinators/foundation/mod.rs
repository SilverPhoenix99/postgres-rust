mod bit_string;
mod combinator;
mod identifier;
mod integer;
mod keyword;
mod macros;
mod map;
mod number;
mod operator;
mod optional;
mod param;
mod parser;
mod string;
mod user_defined_operator;

pub(crate) use self::combinator::Combinator;

#[allow(unused_imports)] // TODO: eventually remove
pub(in crate::combinators) use self::{
    bit_string::bit_string,
    identifier::identifier,
    integer::integer,
    keyword::{any_keyword, keyword_if, keyword_result, KeywordCondCombi},
    macros::{between, choice, located, many, seq},
    map::{map, MapResultCombi},
    number::number,
    optional::{optional, OptionalCombi},
    param::param,
    parser::{parser, ClosureCombi},
    string::string,
    user_defined_operator::user_defined_operator,
};
