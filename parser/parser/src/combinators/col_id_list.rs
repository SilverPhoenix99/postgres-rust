pub(super) fn col_id_list(separator: OperatorKind) -> impl Combinator<Output = QualifiedName> {

    /*
        col_id ( <separator> col_id )*
    */

    parser(move |stream|
        many!(
            sep = separator.parse(stream),
            col_id(stream)
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use pg_lexer::OperatorKind::Dot;

    #[test]
    /// All these methods are similar, so no point in repeating tests:
    /// * test_var_name
    /// * test_name_list
    fn test_col_id_list() {
        let mut stream = TokenStream::new("test.qualified.name", DEFAULT_CONFIG);
        let expected = vec![
            "test".into(),
            "qualified".into(),
            "name".into()
        ];

        assert_eq!(Ok(expected), col_id_list(Dot).parse(&mut stream));
    }
}

use crate::combinators::foundation::many;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::combinators::v2::col_id;
use pg_basics::QualifiedName;
use pg_lexer::OperatorKind;
