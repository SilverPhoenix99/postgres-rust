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

    // Broken down into smaller combinators, due to large Rust type names.
    or((
        alter_function_option_1,
        alter_function_option_2,
        alter_function_option_3,
        alter_function_option_4,
    )).parse(stream)
}

fn alter_function_option_1(stream: &mut TokenStream) -> scan::Result<AlterFunctionOption> {
    or((
        (Called, On, Null, Input).map(|_| Strict(false)),
        (Returns, Null, On, Null, Input).map(|_| Strict(true)),
        Kw::Strict.map(|_| Strict(true)),
        Kw::Immutable.map(|_| Volatility(Immutable)),
    )).parse(stream)
}

fn alter_function_option_2(stream: &mut TokenStream) -> scan::Result<AlterFunctionOption> {
    or((
        Kw::Stable.map(|_| Volatility(Stable)),
        Kw::Volatile.map(|_| Volatility(Volatile)),
        {
            (External, security)
                .map(|(_, option)| option)
        },
        security,
    )).parse(stream)
}

fn security(stream: &mut TokenStream) -> scan::Result<AlterFunctionOption> {

    let (_, definer) = (
        Kw::Security,
        or((
            Definer.map(|_| true),
            Invoker.map(|_| false)
        ))
    ).parse(stream)?;

    Ok(Security(definer))
}

fn alter_function_option_3(stream: &mut TokenStream) -> scan::Result<AlterFunctionOption> {
    or((
        Kw::Leakproof
            .map(|_| Leakproof(true)),
        (Not, Kw::Leakproof)
            .map(|_| Leakproof(false)),
        (Kw::Cost, signed_number)
            .map(|(_, execution_cost)| Cost(execution_cost)),
        (Kw::Rows, signed_number)
            .map(|(_, result_rows)| Rows(result_rows)),
    )).parse(stream)
}

fn alter_function_option_4(stream: &mut TokenStream) -> scan::Result<AlterFunctionOption> {
    or((
        (Kw::Support, any_name)
            .map(|(_, support_function)| Support(support_function)),
        (Kw::Parallel, col_id)
            .map(|(_, mode)| Parallel(mode)),
        (Kw::Set, set_rest_more)
            .map(|(_, option)| Set(option)),
        reset_stmt.map(Reset)
    )).parse(stream)
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
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::signed_number;
use crate::combinators::stmt::reset_stmt;
use crate::combinators::stmt::set_rest_more;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::AlterFunctionOption;
use pg_ast::AlterFunctionOption::*;
use pg_ast::Volatility::*;
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
