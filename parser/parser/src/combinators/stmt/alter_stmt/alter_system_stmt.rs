/// Alias: `AlterSystemStmt`
pub(super) fn alter_system_stmt() -> impl Combinator<Output = AlterSystemStmt> {

    /*
          ALTER SYSTEM RESET generic_reset
        | ALTER SYSTEM SET var_name generic_set_tail
    */

    SystemKw
        .and_right(match_first! {
            Reset.and_then(all_or_var_name, |_, reset| match reset {
                OneOrAll::All => AlterSystemStmt::ResetAll,
                OneOrAll::One(name) => AlterSystemStmt::Reset { name }
            }),
            Set.and_right(var_name)
                .and_then(generic_set_tail, |name, set| match set {
                ValueOrDefault::Default => AlterSystemStmt::SetDefault { name },
                ValueOrDefault::Value(values) => AlterSystemStmt::Set { name, values }
            })
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("system reset all", AlterSystemStmt::ResetAll)]
    #[test_case("system reset some_.name_", AlterSystemStmt::Reset { name: vec!["some_".into(), "name_".into()] })]
    #[test_case("system set var_._name to default", AlterSystemStmt::SetDefault { name: vec!["var_".into(), "_name".into()] })]
    #[test_case("system set var_._name = 'x'", AlterSystemStmt::Set { name: vec!["var_".into(), "_name".into()], values: vec!["x".into()] })]
    fn test_(source: &str, expected: AlterSystemStmt) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = alter_system_stmt().parse(&mut stream);

        assert_eq!(Ok(expected), actual)
    }
}

use crate::combinators::all_or_var_name;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::Combinator;
use crate::combinators::generic_set_tail;
use crate::combinators::var_name;
use pg_ast::AlterSystemStmt;
use pg_ast::OneOrAll;
use pg_ast::ValueOrDefault;
use pg_lexer::Keyword::Reset;
use pg_lexer::Keyword::Set;
use pg_lexer::Keyword::SystemKw;
