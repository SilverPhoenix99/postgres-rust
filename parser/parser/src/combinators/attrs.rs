/// `prefix ( '.' col_label )*`
macro_rules! attrs {
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
    use crate::scan;
    use crate::tests::test_parser;

    #[test]
    fn test_attrs() {

        test_parser!(
            source = ".qualified_.name_",
            parser = parser(|stream|
                attrs!(stream =>
                    Ok::<_, scan::Error>("*some*".into())
                )
            ),
            expected = vec![
                "*some*".into(),
                "qualified_".into(),
                "name_".into()
            ]
        )
    }
}
