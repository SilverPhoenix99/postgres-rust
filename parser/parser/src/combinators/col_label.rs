/// Aliases:
/// * `ColLabel`
/// * `attr_name`
pub(super) fn col_label() -> impl Combinator<Output = Str> {
    match_first!(
        identifier().map(From::from),
        keyword_if(|_| true).map(From::from)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_col_label() {
        let source = "sequence xxyyzz character";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("sequence".into()), col_label().parse(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), col_label().parse(&mut stream));
        assert_eq!(Ok("character".into()), col_label().parse(&mut stream));
    }
}

use crate::combinators::foundation::identifier;
use crate::combinators::foundation::keyword_if;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use pg_basics::Str;
