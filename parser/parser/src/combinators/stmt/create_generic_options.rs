pub(super) fn create_generic_options(stream: &mut TokenStream) -> scan::Result<Vec<GenericOption>> {

    /*
        OPTIONS '(' generic_option_list ')'
    */

    let (_, options) = seq!(Options, paren!(generic_options))
        .parse(stream)?;

    Ok(options)
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
}

use crate::combinators::generic_options;
use pg_ast::GenericOption;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Options;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
