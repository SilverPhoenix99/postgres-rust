pub(super) fn col_id_list(separator: OperatorKind) -> impl Combinator<Output = QualifiedName> {

    /*
        col_id ( <separator> col_id )*
    */

    many_sep(separator, col_id())
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

use crate::combinators::col_id;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::Combinator;
use pg_basics::QualifiedName;
use pg_lexer::OperatorKind;
