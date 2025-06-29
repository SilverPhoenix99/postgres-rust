mod and;
mod between;
mod bit_string;
mod combinator;
mod identifier;
mod integer;
mod keyword;
mod located;
mod many;
mod map;
mod number;
mod operator;
mod optional;
mod or;
mod param;
mod parser;
mod string;
mod user_defined_operator;

pub(crate) use self::combinator::Combinator;

#[allow(unused_imports)] // TODO: eventually remove
pub(in crate::combinators) use self::{
    between::{between_paren, between_square},
    bit_string::bit_string,
    identifier::identifier,
    integer::integer,
    keyword::{any_keyword, keyword_if, keyword_result, KeywordCondCombi},
    located::located,
    many::{many, many_pre, many_sep},
    map::{map, MapResultCombi},
    number::number,
    optional::{optional, OptionalCombi},
    or::or,
    param::param,
    parser::{parser, ClosureCombi},
    string::string,
    user_defined_operator::user_defined_operator,
};
