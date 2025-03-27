/// Alias: `generic_option_elem`
pub(super) fn generic_option() -> impl Combinator<Output = GenericOption> {

    col_label()
        .and_then(string(), GenericOption::new)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_generic_option() {
        let source = "option_name 'option value'";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = generic_option().parse(&mut stream);

        let expected = GenericOption::new("option_name","option value");

        assert_eq!(Ok(expected), actual);
    }
}

use crate::parser::ast_node::GenericOption;
use crate::parser::combinators::col_label;
use crate::parser::combinators::foundation::string;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
