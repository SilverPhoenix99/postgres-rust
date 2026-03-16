/// Alias: `AlterSystemStmt`
pub(in crate::combinators::stmt) fn alter_system_stmt(ctx: &mut ParserContext) -> scan::Result<AlterSystemStmt> {

    /*
          ALTER SYSTEM RESET generic_reset
        | ALTER SYSTEM SET var_name generic_set_tail
    */

    let (_, stmt) = seq!(
        SystemKw,
        alt!(
            seq!(Reset, all_or_var_name)
                .map(|(_, reset)| match reset {
                    OneOrAll::All => AlterSystemStmt::ResetAll,
                    OneOrAll::One(name) => AlterSystemStmt::Reset { name }
                }),
            seq!(Set, var_name, generic_set_tail)
                .map(|(_, name, set)| match set {
                    DefaultableValue::Default => AlterSystemStmt::SetDefault { name },
                    DefaultableValue::Null => AlterSystemStmt::SetNull { name },
                    DefaultableValue::Value(values) => AlterSystemStmt::Set { name, values }
                })
        )
    ).parse(ctx)?;

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;

    #[test_case("system reset all", AlterSystemStmt::ResetAll)]
    #[test_case("system reset some_.name_", AlterSystemStmt::Reset { name: vec!["some_".into(), "name_".into()] })]
    #[test_case("system set var_._name to default", AlterSystemStmt::SetDefault { name: vec!["var_".into(), "_name".into()] })]
    #[test_case("system set var_._name = 'x'", AlterSystemStmt::Set { name: vec!["var_".into(), "_name".into()], values: vec!["x".into()] })]
    fn test_(source: &str, expected: AlterSystemStmt) {
        test_parser!(source, alter_system_stmt, expected)
    }
}

use crate::alt;
use crate::combinators::all_or_var_name;
use crate::combinators::core::Combinator;
use crate::combinators::generic_set_tail;
use crate::combinators::var_name;
use crate::seq;
use crate::ParserContext;
use pg_ast::AlterSystemStmt;
use pg_ast::DefaultableValue;
use pg_ast::OneOrAll;
use pg_lexer::Keyword::Reset;
use pg_lexer::Keyword::Set;
use pg_lexer::Keyword::SystemKw;
use pg_parser_core::scan;
