/// Outputs `P ( '.' col_label )*`.
#[macro_export]
macro_rules! attrs {
    ($prefix:expr) => {
        pg_combinators::parser::<_, pg_basics::QualifiedName>(|stream| {
            let p = pg_combinators::many!(
                pre = $prefix,
                pg_combinators::Combinator::map(
                    pg_combinators::seq!(
                        pg_lexer::OperatorKind::Dot,
                        $crate::col_label
                    ),
                    |(_, name)| name
                )
            );

            pg_combinators::Combinator::parse(&p, stream)
        })
    };
}

#[cfg(test)]
mod tests {
    use pg_combinators::parser;
    use pg_combinators::test_parser;
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
