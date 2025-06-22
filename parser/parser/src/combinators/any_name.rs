pub(in crate::combinators) fn any_name_list(stream: &mut TokenStream) -> Result<Vec<QualifiedName>> {

    /*
        any_name ( ',' any_name )*
    */

    many!(sep = Comma, any_name).parse(stream)
}

/// Alias: `handler_name`
pub(in crate::combinators) fn any_name(stream: &mut TokenStream) -> Result<QualifiedName> {

    /*
        col_id attrs
    */

    attrs!(col_id).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_any_name_list() {
        test_parser!(v2,
            source = "qual.name_, second.qualif",
            parser = any_name_list,
            expected = vec![
                vec!["qual".into(), "name_".into()],
                vec!["second".into(), "qualif".into()]
            ]
        )
    }

    #[test]
    fn test_any_name() {
        test_parser!(v2,
            source = "some_.qualified_.name_",
            parser = any_name,
            expected = vec![
                "some_".into(),
                "qualified_".into(),
                "name_".into()
            ]
        )
    }
}

use crate::combinators::foundation::many;
use crate::combinators::foundation::Combinator;
use crate::combinators::v2::attrs;
use crate::combinators::v2::col_id;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_basics::QualifiedName;
use pg_lexer::OperatorKind::Comma;
