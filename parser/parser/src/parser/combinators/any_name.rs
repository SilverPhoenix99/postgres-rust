/// Post-condition: Vec is **Not** empty
pub(super) fn any_name_list() -> impl Combinator<Output = Vec<QualifiedName>> {

    /*
        any_name ( ',' any_name )*
    */

    many_sep(Comma, any_name())
}

/// Post-condition: Vec is **Not** empty
///
/// Alias: `handler_name`
pub(super) fn any_name() -> impl Combinator<Output = QualifiedName> {

    /*
        col_id attrs
    */

    attrs(col_id())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_any_name_list() {
        let source = "qual.name_, second.qualif";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = vec![
            vec!["qual".into(), "name_".into()],
            vec!["second".into(), "qualif".into()]
        ];

        assert_eq!(Ok(expected), any_name_list().parse(&mut stream));
    }

    #[test]
    fn test_any_name() {
        let source = "some_.qualified_.name_";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = any_name().parse(&mut stream);

        let expected: QualifiedName = vec![
            "some_".into(),
            "qualified_".into(),
            "name_".into()
        ];

        assert_eq!(Ok(expected), actual);
    }
}

use crate::parser::combinators::attrs;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::many_sep;
use crate::parser::combinators::foundation::Combinator;
use postgres_basics::QualifiedName;
use postgres_parser_lexer::OperatorKind::Comma;
