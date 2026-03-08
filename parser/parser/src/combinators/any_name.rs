pub(super) fn any_name_list(ctx: &mut ParserContext) -> scan::Result<Vec<QualifiedName>> {

    /*
        any_name ( ',' any_name )*
    */

    many!(sep = Comma, any_name).parse(ctx)
}

/// Aliases:
/// * `handler_name`
/// * `opt_qualified_name`
pub(super) fn any_name(ctx: &mut ParserContext) -> scan::Result<QualifiedName> {

    /*
        col_id attrs
    */

    attrs!(col_id).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

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

use super::col_id;
use crate::attrs;
use crate::combinators::core::Combinator;
use crate::many;
use crate::ParserContext;
use pg_basics::QualifiedName;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
