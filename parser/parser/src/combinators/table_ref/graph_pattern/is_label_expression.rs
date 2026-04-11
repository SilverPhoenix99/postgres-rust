/// Alias: `opt_is_label_expression`
/// Inlined: `label_expression`
pub(super) fn is_label_expression(ctx: &mut ParserContext) -> scan::Result<Vec<Str>> {

    /*
        IS ColId ( '|' ColId )*
    */

    let (_, names) = seq!(Is, many!(sep = Pipe, col_id)).parse(ctx)?;

    Ok(names)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_is_label_expression() {
        test_parser! {
            source = "is foo | bar",
            parser = is_label_expression,
            expected = vec!["foo".into(), "bar".into()]
        }
    }
}

use crate::combinators::col_id;
use crate::combinators::core::Combinator;
use crate::context::ParserContext;
use crate::many;
use crate::seq;
use pg_basics::Str;
use pg_lexer::Keyword::Is;
use pg_lexer::OperatorKind::Pipe;
use pg_parser_core::scan;
