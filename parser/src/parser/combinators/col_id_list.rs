/// Post-condition: Vec is **Not** empty
pub(super) fn col_id_list(separator: OperatorKind) -> impl Combinator<Output = QualifiedName> {

    /*
        col_id ( <separator> col_id )*
    */

    many_sep(separator, col_id())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::OperatorKind::Dot;
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

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

use crate::lexer::OperatorKind;
use crate::parser::ast_node::QualifiedName;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::many_sep;
use crate::parser::combinators::foundation::Combinator;
