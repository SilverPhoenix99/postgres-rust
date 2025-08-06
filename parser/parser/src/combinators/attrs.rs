/// Outputs `P ( '.' col_label )*`.
macro_rules! attrs {
    ($prefix:expr) => {
        $crate::combinators::foundation::parser::<_, pg_basics::QualifiedName>(|stream| {
            $crate::combinators::foundation::many!(
                pre = $prefix,
                $crate::combinators::foundation::Combinator::map(
                    $crate::combinators::foundation::seq!(
                        pg_lexer::OperatorKind::Dot,
                        $crate::combinators::col_label
                    ),
                    |(_, name)| name
                )
            ).parse(stream)
        })
    };
}

pub(in crate::combinators) use attrs;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinators::foundation::parser;
    use crate::tests::test_parser;
    use pg_parser_core::scan;

    #[test]
    fn test_attrs() {

        test_parser!(
            source = ".qualified_.name_",
            parser = attrs!(parser(|_|
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
