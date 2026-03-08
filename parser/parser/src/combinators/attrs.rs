/// Outputs `P ( '.' col_label )*`.
#[macro_export]
macro_rules! attrs {
    ($prefix:expr) => {
        $crate::combinators::core::parser::<_, pg_basics::QualifiedName>(|ctx| {
            let p = $crate::many!(
                pre = $prefix,
                $crate::combinators::core::Combinator::map(
                    $crate::seq!(
                        pg_lexer::OperatorKind::Dot,
                        $crate::combinators::col_label
                    ),
                    |(_, name)| name
                )
            );

            $crate::combinators::core::Combinator::parse(&p, ctx)
        })
    };
}

#[cfg(test)]
mod tests {
    use crate::combinators::core::parser;
    use crate::test_parser;
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
