pub(super) fn attrs<F>(prefix: F) -> impl Combinator<Output = QualifiedName>
where
    F: Combinator<Output = Str>
{
    /*
        prefix ( '.' col_label )*
    */

    many_pre(
        prefix,
        Dot.and_right(col_label())
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinators;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_attrs() {
        let source = ".qualified_.name_";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let parser = combinators::foundation::parser(|_| Ok("*some*".into()));
        let actual = attrs(parser).parse(&mut stream);

        let expected: QualifiedName = vec![
            "*some*".into(),
            "qualified_".into(),
            "name_".into()
        ];

        assert_eq!(Ok(expected), actual);
    }
}

use crate::combinators::col_label;
use crate::combinators::foundation::many_pre;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use pg_basics::QualifiedName;
use pg_basics::Str;
use pg_lexer::OperatorKind::Dot;
