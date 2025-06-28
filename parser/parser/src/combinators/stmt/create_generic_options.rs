pub(super) fn create_generic_options(stream: &mut TokenStream) -> scan::Result<Option<Vec<GenericOption>>> {

    /*
        OPTIONS '(' generic_option_list ')'
    */

    let result = seq!(=>
        Options.parse(stream),
        between!(paren : stream =>
            generic_options(stream)
        )
    );

    let stmt = result.optional()?
        .map(|(_, options)| options);

    Ok(stmt)
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
        test_parser!(source, create_generic_options, expected);
    }
}

use crate::combinators::foundation::between;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::generic_options;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::GenericOption;
use pg_lexer::Keyword::Options;
