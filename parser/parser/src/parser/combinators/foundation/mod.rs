mod and;
mod between;
mod bit_string;
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
mod skip;
mod string;
mod try_match;
mod user_defined_operator;

pub(in crate::parser) use self::combinator::Combinator;

#[allow(unused_imports)] // TODO: eventually remove
pub(super) use self::{
    and::{and, sequence, AndCombi},
    between::{between, BetweenCombi},
    bit_string::{bit_string, BitStringCombi},
    combinator_helpers::CombinatorHelpers,
    identifier::{identifier, IdentifierCombi},
    integer::{integer, IntegerCombi},
    keyword::{keyword_if, keyword_result, keyword_when, KeywordCondCombi},
    located::{located, LocCombi},
    many::{many, many_pre, many_sep, ManyCombi, ManyPrefixedCombi, ManySepCombi},
    map::{map, map_err, map_result},
    maybe_match::{maybe_match, MaybeMatchCombi},
    number::{number, NumberCombi},
    operator::{operator_if, operator_result, operator_when, OperatorCondCombi},
    optional::{optional, OptionalCombi},
    or::{match_first, match_first_with_state, or, OrCombi},
    param::{param, ParamCombi},
    parser::{enclosure, parser, ClosureCombi},
    required::{required, RequiredCombi},
    skip::{skip, SkipCombi},
    string::{string, StringCombi},
    try_match::{try_match, TryMatchCombi},
    user_defined_operator::{user_defined_operator, UserOpCombi}
};
