/// Alias: `generic_option_elem`
pub(super) fn generic_option(ctx: &mut ParserContext) -> scan::Result<GenericOption> {

    let (name, arg) = seq!(col_label, string).parse(ctx)?;
    let option = GenericOption::new(name, arg);
    Ok(option)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_generic_option() {
        test_parser!(
            source = "option_name 'option value'",
            parser = generic_option,
            expected = GenericOption::new("option_name","option value")
        )
    }
}

use crate::combinators::col_label;
use crate::combinators::core::string;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_ast::GenericOption;
use pg_parser_core::scan;
