pub(in crate::combinators) fn param() -> ParamCombi {
    ParamCombi
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::combinators) struct ParamCombi;

impl Combinator for ParamCombi {
    type Output = i32;

    fn parse(&self, stream: &mut TokenStream<'_>) -> scan::Result<Self::Output> {
        stream.consume(|tok| match tok {
            Param { index } => Some(*index),
            _ => None
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_param() {
        let mut stream = TokenStream::new("$3", DEFAULT_CONFIG);
        assert_eq!(Ok(3), param().parse(&mut stream))
    }
}

use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenConsumer;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Param;
