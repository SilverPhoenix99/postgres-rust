/// Outputs `P ( '.' col_label )*`.
macro_rules! attrs {
    ($prefix:expr) => {
        $crate::combinators::foundation::parser::<_, pg_basics::QualifiedName>(move |stream| {
            use $crate::combinators::col_label;
            use $crate::combinators::foundation::Combinator;
            use $crate::combinators::foundation::many_m;
            use $crate::combinators::foundation::seq;
            use pg_lexer::OperatorKind::Dot;

            many_m!(
                pre = $prefix,
                seq!(Dot, col_label).map(|(_, name)| name)
            ).parse(stream)
        })
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
