/// `prefix ( '.' col_label )*`
macro_rules! attrs {
    ($prefix:expr) => {
        $crate::combinators::foundation::parser(|stream| {
            #[allow(unused_imports)]
            use $crate::combinators::foundation::{ClosureHelpers, Combinator, CombinatorHelpers};
            use $crate::combinators::foundation::many;
            use $crate::combinators::foundation::seq;
            use $crate::combinators::col_label;
            use pg_lexer::OperatorKind::Dot;

            let combinator = many!(
                pre = $prefix,
                seq!(Dot, col_label).right()
            );

            combinator.parse(stream)
        })
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
