/// `prefix ( '.' col_label )*`
macro_rules! attrs {
    ($prefix:expr) => {{
        #[allow(unused_imports)]
        use $crate::combinators::foundation::Combinator;
        use $crate::combinators::foundation::many;
        use $crate::combinators::col_label;
        use pg_lexer::OperatorKind::Dot;

        many!(
            pre = $prefix,
            (Dot, col_label).right()
        )
    }};

    ($stream:ident => $prefix:expr) => {{
        #[allow(unused_imports)]
        use $crate::combinators::foundation::Combinator;
        use $crate::combinators::foundation::many;
        use $crate::combinators::foundation::seq;
        use $crate::combinators::col_label;
        use pg_lexer::OperatorKind::Dot;

        many!(=>
            pre = $prefix,
            seq!($stream => Dot, col_label)
                .map(|(_, item)| item)
        )
    }};
}

pub(in crate::combinators) use attrs;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinators::foundation::{many, parser};
    use crate::tests::test_parser;

    #[test]
    fn test_attrs() {

        test_parser!(
            source = ".qualified_.name_",
            parser = attrs!(parser(|_| Ok("*some*".into()))),
            expected = vec![
                "*some*".into(),
                "qualified_".into(),
                "name_".into()
            ]
        )
    }
}
