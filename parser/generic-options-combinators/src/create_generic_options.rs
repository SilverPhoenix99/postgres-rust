pub fn create_generic_options(ctx: &mut ParserContext) -> scan::Result<Vec<GenericOption>> {

    /*
        OPTIONS '(' generic_option_list ')'
    */

    let (_, options) = seq!(Options, paren!(generic_options))
        .parse(ctx)?;

    Ok(options)
}

/// Alias: `generic_option_list`
fn generic_options(ctx: &mut ParserContext) -> scan::Result<Vec<GenericOption>> {

    many!(sep = Comma, generic_option).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_create_generic_options() {
        test_parser!(
            source = "options (option1 'value1', option2 'value2')",
            parser = create_generic_options,
            expected = vec![
                GenericOption::new("option1", "value1"),
                GenericOption::new("option2", "value2")
            ]
        );
    }

    #[test]
    fn test_generic_options() {
        test_parser!(
            source = "option1 'value1', option2 'value2'",
            parser = generic_options,
            expected = vec![
                GenericOption::new("option1", "value1"),
                GenericOption::new("option2", "value2")
            ]
        );
    }
}

use crate::generic_option;
use pg_ast::GenericOption;
use pg_combinators::many;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Options;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
