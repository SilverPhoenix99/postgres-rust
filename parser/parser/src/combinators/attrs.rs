pub(super) fn attrs<F>(prefix: F) -> impl Combinator<Output = QualifiedName>
where
    F: Combinator<Output = Str>
{
    /*
        prefix ( '.' col_label )*
    */

    parser(move |stream|
        v2::attrs!(stream, prefix.parse(stream))
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_attrs() {
        let parser = parser(|_| Ok("*some*".into()));

        test_parser!(
            source = ".qualified_.name_",
            parser = attrs(parser),
            expected = vec![
                "*some*".into(),
                "qualified_".into(),
                "name_".into()
            ]
        )
    }
}

use crate::combinators::foundation::many;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::v2;
use pg_basics::QualifiedName;
use pg_basics::Str;
