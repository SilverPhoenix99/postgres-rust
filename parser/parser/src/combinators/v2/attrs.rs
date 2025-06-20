/// `prefix ( '.' col_label )*`
macro_rules! attrs {
    ($stream:expr, $prefix:expr) => {{
        use $crate::combinators::foundation::seq;
        use $crate::combinators::foundation::many;
        use $crate::combinators::v2::col_label;
        use pg_lexer::OperatorKind::Dot;

        many!(
            pre = $prefix,
            seq!(Dot.parse($stream), col_label($stream))
                .map(|(_, name)| name)
        )
    }};
}

pub(in crate::combinators) use attrs;
