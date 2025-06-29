/// Alias: `generic_option_list`
pub(super) fn generic_options(stream: &mut TokenStream) -> scan::Result<Vec<GenericOption>> {

    many_sep(Comma, generic_option).parse(stream)
}

/// Alias: `generic_option_elem`
pub(super) fn generic_option(stream: &mut TokenStream) -> scan::Result<GenericOption> {

    let (name, arg) = (col_label, string).parse(stream)?;
    let option = GenericOption::new(name, arg);
    Ok(option)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

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

use crate::combinators::col_label;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::GenericOption;
use pg_lexer::OperatorKind::Comma;
