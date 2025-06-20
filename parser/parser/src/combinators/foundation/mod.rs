mod and;
mod between;
mod bit_string;
mod choice;
mod combinator;
mod combinator_helpers;
mod identifier;
mod integer;
mod keyword;
mod located;
mod many;
mod map;
mod maybe_match;
mod number;
mod operator;
mod optional;
mod or;
mod param;
mod parser;
mod required;
mod seq;
mod skip;
mod string;
mod try_match;
mod user_defined_operator;

pub(crate) use self::combinator::Combinator;

#[allow(unused_imports)] // TODO: eventually remove
pub(in crate::combinators) use self::{
    and::{and, sequence},
    between::between,
    bit_string::bit_string,
    choice::choice,
    combinator_helpers::CombinatorHelpers,
    identifier::identifier,
    integer::integer,
    keyword::{any_keyword, keyword_if, keyword_result, keyword_when},
    located::located,
    many::many,
    map::{map, map_err, map_result},
    maybe_match::maybe_match,
    number::number,
    operator::{operator_if, operator_result, operator_when},
    optional::optional,
    or::{match_first, match_first_with_state, or},
    param::param,
    parser::{enclosure, parser},
    required::required,
    seq::seq,
    skip::skip,
    string::string,
    try_match::try_match,
    user_defined_operator::user_defined_operator,
};
