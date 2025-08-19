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
    fn test_generic_option() {
        test_parser!(
            source = "option_name 'option value'",
            parser = generic_option,
            expected = GenericOption::new("option_name","option value")
        )
    }
}

use pg_combinators::seq;
use pg_combinators::string;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_generic_options_ast::GenericOption;
use pg_parser_core::scan;
use pg_sink_combinators::col_label;
