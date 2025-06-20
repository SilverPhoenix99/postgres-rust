/// Production: `'(' ICONST ')'`
pub(super) fn i32_literal_paren() -> impl Combinator<Output = i32> {

    between_paren(parser(integer)).map(From::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_i32_literal_paren() {
        let mut stream = TokenStream::new(" (123 )", DEFAULT_CONFIG);
        assert_eq!(Ok(123), i32_literal_paren().parse(&mut stream));
    }
}

use crate::combinators::between_paren;
use crate::combinators::foundation::integer;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
