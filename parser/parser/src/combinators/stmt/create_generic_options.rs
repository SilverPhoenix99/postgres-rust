pub(super) fn create_generic_options() -> impl Combinator<Output = Option<Vec<GenericOption>>> {

    /*
        OPTIONS '(' generic_option_list ')'
    */

    Options.and_right(between_paren(generic_options()))
        .optional()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("options (option1 'value1', option2 'value2')",
        Some(vec![
            GenericOption::new("option1", "value1"),
            GenericOption::new("option2", "value2")
        ])
    )]
    #[test_case("", None)]
    fn test_create_generic_options(source: &str, expected: Option<Vec<GenericOption>>) {
        test_parser!(source, create_generic_options(), expected);
    }
}

use crate::combinators::between_paren;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::generic_options;
use pg_ast::GenericOption;
use pg_lexer::Keyword::Options;
