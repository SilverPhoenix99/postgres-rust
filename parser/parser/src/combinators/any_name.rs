pub(in crate::combinators) fn any_name_list(stream: &mut TokenStream) -> scan::Result<Vec<QualifiedName>> {

    /*
        any_name ( ',' any_name )*
    */

    many_sep(Comma, any_name).parse(stream)
}

/// Alias: `handler_name`
pub(in crate::combinators) fn any_name(stream: &mut TokenStream) -> scan::Result<QualifiedName> {

    /*
        col_id attrs
    */

    attrs(col_id).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_any_name_list() {
        test_parser!(
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
        test_parser!(
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

use crate::combinators::attrs::attrs;
use crate::combinators::col_id;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_basics::QualifiedName;
use pg_lexer::OperatorKind::Comma;
