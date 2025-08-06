/// Alias: `common_func_opt_item`
/// Inlined: `FunctionSetResetClause`
pub(super) fn alter_function_option(stream: &mut TokenStream) -> scan::Result<AlterFunctionOption> {

    /*
          CALLED ON NULL INPUT
        | RETURNS NULL ON NULL INPUT
        | STRICT
        | IMMUTABLE
        | STABLE
        | VOLATILE
        | EXTERNAL SECURITY DEFINER
        | EXTERNAL SECURITY INVOKER
        | SECURITY DEFINER
        | SECURITY INVOKER
        | LEAKPROOF
        | NOT LEAKPROOF
        | COST NumericOnly
        | ROWS NumericOnly
        | SUPPORT any_name
        | PARALLEL ColId
        | SET set_rest_more
        | reset_stmt
    */

    alt!(
        seq!(Called, On, Null, Input)
            .map(|_| Strict(false)),
        seq!(Returns, Null, On, Null, Input)
            .map(|_| Strict(true)),
        Kw::Strict
            .map(|_| Strict(true)),
        Kw::Immutable
            .map(|_| Volatility(Immutable)),
        Kw::Stable
            .map(|_| Volatility(Stable)),
        Kw::Volatile
            .map(|_| Volatility(Volatile)),
        seq!(External, security)
            .map(|(_, option)| option),
        security,
        Kw::Leakproof
            .map(|_| Leakproof(true)),
        seq!(Not, Kw::Leakproof)
            .map(|_| Leakproof(false)),
        seq!(Kw::Cost, signed_number)
            .map(|(_, execution_cost)| Cost(execution_cost)),
        seq!(Kw::Rows, signed_number)
            .map(|(_, result_rows)| Rows(result_rows)),
        seq!(Kw::Support, any_name)
            .map(|(_, support_function)| Support(support_function)),
        seq!(Kw::Parallel, col_id)
            .map(|(_, mode)| Parallel(mode)),
        seq!(Kw::Set, set_rest_more)
            .map(|(_, option)| Set(option)),
        reset_stmt.map(Reset)
    ).parse(stream)
}

fn security(stream: &mut TokenStream) -> scan::Result<AlterFunctionOption> {

    let (_, definer) = seq!(
        Kw::Security,
        alt!(
            Definer.map(|_| true),
            Invoker.map(|_| false)
        )
    ).parse(stream)?;

    Ok(Security(definer))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{
        SetRestMore::TimeZone,
        SignedNumber::IntegerConst,
        VariableTarget::All,
        ZoneValue::Local
    };
    use test_case::test_case;

    #[test_case("called on null input", Strict(false))]
    #[test_case("returns null on null input", Strict(true))]
    #[test_case("strict", Strict(true))]
    #[test_case("immutable", Volatility(Immutable))]
    #[test_case("stable", Volatility(Stable))]
    #[test_case("volatile", Volatility(Volatile))]
    #[test_case("external security definer", Security(true))]
    #[test_case("external security invoker", Security(false))]
    #[test_case("security definer", Security(true))]
    #[test_case("security invoker", Security(false))]
    #[test_case("leakproof", Leakproof(true))]
    #[test_case("not leakproof", Leakproof(false))]
    #[test_case("cost 10", Cost(IntegerConst(10)))]
    #[test_case("rows 5", Rows(IntegerConst(5)))]
    #[test_case("support some_function", Support(vec!["some_function".into()]))]
    #[test_case("parallel safe", Parallel("safe".into()))]
    #[test_case("set time zone local", Set(TimeZone(Local)))]
    #[test_case("reset all", Reset(All))]
    fn test_common_func_opt_item(source: &str, expected: AlterFunctionOption) {
        test_parser!(source, alter_function_option, expected);
    }
}

use crate::combinators::any_name;
use crate::combinators::col_id;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::seq;
use crate::combinators::signed_number;
use crate::combinators::stmt::reset_stmt;
use crate::combinators::stmt::set_rest_more;
use pg_ast::AlterFunctionOption;
use pg_ast::AlterFunctionOption::*;
use pg_ast::Volatility::*;
use pg_combinators::Combinator;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Called;
use pg_lexer::Keyword::Definer;
use pg_lexer::Keyword::External;
use pg_lexer::Keyword::Input;
use pg_lexer::Keyword::Invoker;
use pg_lexer::Keyword::Not;
use pg_lexer::Keyword::Null;
use pg_lexer::Keyword::On;
use pg_lexer::Keyword::Returns;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
