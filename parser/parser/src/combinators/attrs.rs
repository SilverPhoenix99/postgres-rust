pub(in crate::combinators) fn attrs<P>(prefix: P) -> impl Combinator<Output = QualifiedName>
where
    P: Combinator<Output = Str>
{
    /*
        prefix ( '.' col_label )*
    */

    many_pre(
        prefix,
        (Dot, col_label).map(|(_, name)| name)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinators::foundation::parser;
    use crate::scan;
    use crate::tests::test_parser;

    #[test]
    fn test_attrs() {

        test_parser!(
            source = ".qualified_.name_",
            parser = attrs(parser(|_|
                Ok::<_, scan::Error>("*some*".into())
            )),
            expected = vec![
                "*some*".into(),
                "qualified_".into(),
                "name_".into()
            ]
        )
    }
}

use crate::combinators::col_label::col_label;
use crate::combinators::foundation::many_pre;
use crate::combinators::foundation::Combinator;
use pg_basics::QualifiedName;
use pg_basics::Str;
use pg_lexer::OperatorKind::Dot;
