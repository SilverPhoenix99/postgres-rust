pub(in crate::parser) fn param() -> ParamCombi {
    ParamCombi
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::parser) struct ParamCombi;

impl Combinator for ParamCombi {
    type Output = i32;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {
        stream.consume(|tok| match tok {
            Param { index } => Some(*index),
            _ => None
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_param() {
        let mut stream = TokenStream::new("$3", DEFAULT_CONFIG);
        assert_eq!(Ok(3), param().parse(&mut stream))
    }
}

use crate::parser::combinators::Combinator;
use crate::parser::result::ScanResult;
use crate::parser::token_stream::TokenConsumer;
use crate::parser::token_stream::TokenStream;
use crate::parser::token_value::TokenValue::Param;
