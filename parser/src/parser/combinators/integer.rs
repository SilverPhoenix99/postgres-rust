/// Alias: `ICONST`
pub(in crate::parser) fn integer() -> IntegerCombi {
    IntegerCombi
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::parser) struct IntegerCombi;

impl Combinator for IntegerCombi {
    type Output = NonNegative;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {

        stream.consume_with_slice(|(tok, slice, _)| {
            let NumberLiteral(radix) = tok else { return None };
            let Some(IntegerConst(int)) = parse_number(slice, radix) else { return None };
            Some(int)
        })
    }
}

pub(in crate::parser::combinators) fn parse_number(value: &str, radix: NumberRadix) -> Option<UnsignedNumber> {

    let value = value.replace("_", "");

    if let Ok(int) = i32::from_str_radix(&value, radix as u32) {
        // SAFETY: `0 <= int <= i32::MAX`
        Some(IntegerConst(int.into()))
    }
    else {
        Some(NumericConst {
            radix,
            value: value.into_boxed_str()
        })
    }
}

use crate::lexer::RawTokenKind::NumberLiteral;
use crate::parser::ast_node::UnsignedNumber;
use crate::parser::ast_node::UnsignedNumber::{IntegerConst, NumericConst};
use crate::parser::combinators::Combinator;
use crate::parser::result::ScanResult;
use crate::parser::token_stream::{SlicedTokenConsumer, TokenStream};
use crate::NumberRadix;
use postgres_basics::NonNegative;
