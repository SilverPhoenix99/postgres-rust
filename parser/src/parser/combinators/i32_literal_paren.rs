/// Production: `'(' ICONST ')'`
pub(super) fn i32_literal_paren() -> impl Combinator<Output = i32> {

    between_paren(integer()).map(From::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_i32_literal_paren() {
        let mut stream = TokenStream::new(" (123 )", DEFAULT_CONFIG);
        assert_eq!(Ok(123), i32_literal_paren().parse(&mut stream));
    }
}

use crate::parser::combinators::between_paren;
use crate::parser::combinators::foundation::integer;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
