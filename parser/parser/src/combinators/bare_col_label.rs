/// Alias: `BareColLabel`
pub(super) fn bare_col_label() -> impl Combinator<Output = Str> {
    match_first!(
        parser(identifier).map(From::from),
        keyword_if(|kw| kw.details().bare()).map(From::from)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_bare_col_label() {
        let source = "sequence xxyyzz";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("sequence".into()), bare_col_label().parse(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), bare_col_label().parse(&mut stream));
    }
}

use crate::combinators::foundation::identifier;
use crate::combinators::foundation::keyword_if;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use pg_basics::Str;
