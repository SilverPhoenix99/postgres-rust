/// Alias: `AlterSystemStmt`
pub(super) fn alter_system_stmt(ctx: &mut ParserContext) -> scan::Result<AlterSystemStmt> {

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
                    ValueOrDefault::Default => AlterSystemStmt::SetDefault { name },
                    ValueOrDefault::Value(values) => AlterSystemStmt::Set { name, values }
                })
        )
    ).parse(ctx)?;

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("system reset all", AlterSystemStmt::ResetAll)]
    #[test_case("system reset some_.name_", AlterSystemStmt::Reset { name: vec!["some_".into(), "name_".into()] })]
    #[test_case("system set var_._name to default", AlterSystemStmt::SetDefault { name: vec!["var_".into(), "_name".into()] })]
    #[test_case("system set var_._name = 'x'", AlterSystemStmt::Set { name: vec!["var_".into(), "_name".into()], values: vec!["x".into()] })]
    fn test_(source: &str, expected: AlterSystemStmt) {
        test_parser!(source, alter_system_stmt, expected)
    }
}

use pg_ast::AlterSystemStmt;
use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_generic_set_combinators::all_or_var_name;
use pg_generic_set_combinators::generic_set_tail;
use pg_lexer::Keyword::Reset;
use pg_lexer::Keyword::Set;
use pg_lexer::Keyword::SystemKw;
use pg_parser_core::scan;
use pg_sink_ast::OneOrAll;
use pg_sink_ast::ValueOrDefault;
use pg_sink_combinators::var_name;
