/// Alias: `AlterSystemStmt`
pub(super) fn alter_system_stmt() -> impl Combinator<Output = RawStmt> {

    /*
          ALTER SYSTEM_P RESET generic_reset
        | ALTER SYSTEM_P SET var_name generic_set_tail
    */

    SystemKw
        .and_right(match_first! {
            Reset.and_then(generic_reset(), |_, reset| match reset {
                OneOrAll::All => AlterSystemStmt::ResetAll,
                OneOrAll::One(name) => AlterSystemStmt::Reset { name }
            }),
            Set.and_right(var_name())
                .and_then(generic_set_tail(), |name, set| match set {
                ValueOrDefault::Default => AlterSystemStmt::SetDefault { name },
                ValueOrDefault::Value(values) => AlterSystemStmt::Set { name, values }
            })
        })
        .map(From::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test_case("system reset all", AlterSystemStmt::ResetAll)]
    #[test_case("system reset some_.name_", AlterSystemStmt::Reset { name: vec!["some_".into(), "name_".into()] })]
    #[test_case("system set var_._name to default", AlterSystemStmt::SetDefault { name: vec!["var_".into(), "_name".into()] })]
    #[test_case("system set var_._name = 'x'", AlterSystemStmt::Set { name: vec!["var_".into(), "_name".into()], values: vec!["x".into()] })]
    fn test_(source: &str, expected: AlterSystemStmt) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = alter_system_stmt().parse(&mut stream);

        assert_eq!(Ok(expected.into()), actual)
    }
}

use crate::lexer::Keyword::Reset;
use crate::lexer::Keyword::Set;
use crate::lexer::Keyword::SystemKw;
use crate::parser::ast_node::AlterSystemStmt;
use crate::parser::ast_node::OneOrAll;
use crate::parser::ast_node::RawStmt;
use crate::parser::ast_node::ValueOrDefault;
use crate::parser::combinators::match_first;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::generic_reset::generic_reset;
use crate::parser::generic_set_tail::generic_set_tail;
use crate::parser::var_name;
