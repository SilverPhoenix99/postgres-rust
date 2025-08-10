/// Alias: `generic_option_list`
pub(super) fn generic_options(ctx: &mut ParserContext) -> scan::Result<Vec<GenericOption>> {

    many!(sep = Comma, generic_option).parse(ctx)
}

/// Alias: `generic_option_elem`
pub(super) fn generic_option(ctx: &mut ParserContext) -> scan::Result<GenericOption> {

    let (name, arg) = seq!(col_label, string).parse(ctx)?;
    let option = GenericOption::new(name, arg);
    Ok(option)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

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

    #[test]
    fn test_generic_option() {
        test_parser!(
            source = "option_name 'option value'",
            parser = generic_option,
            expected = GenericOption::new("option_name","option value")
        )
    }
}

use pg_ast::GenericOption;
use pg_combinators::many;
use pg_combinators::seq;
use pg_combinators::string;
use pg_combinators::Combinator;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
use pg_sink_combinators::col_label;
