/// Alias: `BareColLabel`
pub(super) fn bare_col_label() -> impl Combinator<Output = Str> {
    match_first!(
        identifier().map(From::from),
        keyword_if(|kw| kw.details().bare()).map(From::from)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_bare_col_label() {
        let source = "sequence xxyyzz";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("sequence".into()), bare_col_label().parse(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), bare_col_label().parse(&mut stream));
    }
}

use crate::parser::combinators::foundation::identifier;
use crate::parser::combinators::foundation::keyword_if;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
