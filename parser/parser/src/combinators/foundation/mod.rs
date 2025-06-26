mod bit_string;
mod combinator;
mod identifier;
mod integer;
mod keyword;
mod macros;
mod map;
mod maybe_match;
mod number;
mod operator;
mod optional;
mod or;
mod param;
mod parser;
mod required;
mod string;
mod user_defined_operator;

pub(crate) use self::combinator::Combinator;

#[allow(unused_imports)] // TODO: eventually remove
pub(in crate::combinators) use self::{
    bit_string::bit_string,
    identifier::identifier,
    integer::integer,
    keyword::{any_keyword, keyword_if, keyword_result, keyword_when, KeywordCondCombi},
    macros::{between, choice, located, many, seq},
    map::{map, map_result, MapResultCombi},
    maybe_match::{maybe_match, MaybeMatchCombi},
    number::number,
    operator::{operator_if, operator_result, OperatorCondCombi},
    optional::{optional, OptionalCombi},
    or::{match_first, match_first_with_state, or, OrCombi},
    param::param,
    parser::{parser, ClosureCombi},
    required::{required, RequiredCombi},
    string::string,
    user_defined_operator::{user_defined_operator},
};
