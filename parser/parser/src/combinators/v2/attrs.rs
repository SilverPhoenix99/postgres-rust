/// `prefix ( '.' col_label )*`
macro_rules! attrs {
    ($prefix:expr) => {
        $crate::combinators::foundation::parser(|stream| {
            #[allow(unused_imports)]
            use $crate::combinators::foundation::{ClosureHelpers, CombinatorHelpers};
            use $crate::combinators::foundation::many;
            use $crate::combinators::foundation::seq;
            use $crate::combinators::v2::col_label;
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
