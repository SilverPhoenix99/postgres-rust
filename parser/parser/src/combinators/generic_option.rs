/// Alias: `generic_option_list`
pub(super) fn generic_options() -> impl Combinator<Output = Vec<GenericOption>> {

    many!(sep = Comma, generic_option())
}

/// Alias: `generic_option_elem`
pub(super) fn generic_option() -> impl Combinator<Output = GenericOption> {

    seq!(col_label, string)
        .map(|(name, arg)| GenericOption::new(name, arg))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::test_parser;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_generic_options() {
        test_parser!(
            source = "option1 'value1', option2 'value2'",
            parser = generic_options(),
            expected = vec![
                GenericOption::new("option1", "value1"),
                GenericOption::new("option2", "value2")
            ]
        );
    }

    #[test]
    fn test_generic_option() {
        let source = "option_name 'option value'";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = generic_option().parse(&mut stream);

        let expected = GenericOption::new("option_name","option value");

        assert_eq!(Ok(expected), actual);
    }
}

use crate::combinators::col_label;
use crate::combinators::foundation::many;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use pg_ast::GenericOption;
use pg_lexer::OperatorKind::Comma;
