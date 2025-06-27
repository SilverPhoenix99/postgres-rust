/// `prefix ( '.' col_label )*`
macro_rules! attrs {
    ($prefix:expr) => {
        $crate::combinators::foundation::many!(
            pre = $prefix,
            $crate::combinators::foundation::Combinator::right((
                pg_lexer::OperatorKind::Dot,
                $crate::combinators::col_label
            ))
        )
    };

    ($stream:ident => $prefix:expr) => {
        $crate::combinators::foundation::many!(=>
            pre = $prefix,
            $crate::combinators::foundation::seq!($stream =>
                pg_lexer::OperatorKind::Dot,
                $crate::combinators::col_label
            )
                .map(|(_, item)| item)
        )
    };
}

pub(in crate::combinators) use attrs;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinators::foundation::parser;
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
