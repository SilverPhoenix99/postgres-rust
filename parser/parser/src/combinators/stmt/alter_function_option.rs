/// Alias: `common_func_opt_item`
/// Inlined: `FunctionSetResetClause`
pub(super) fn alter_function_option(ctx: &mut ParserContext) -> scan::Result<AlterFunctionOption> {

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
    ).parse(ctx)
}

fn security(ctx: &mut ParserContext) -> scan::Result<AlterFunctionOption> {

    let (_, definer) = seq!(
        Kw::Security,
        alt!(
            Definer.map(|_| true),
            Invoker.map(|_| false)
        )
    ).parse(ctx)?;

    Ok(Security(definer))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::SetRestMore::TimeZone;
    use pg_ast::SignedNumber::IntegerConst;
    use pg_ast::VariableTarget::All;
    use pg_ast::ZoneValue::Local;
    use test_case::test_matrix;

    #[test_matrix("called on null input" => Ok(Strict(false)))]
    #[test_matrix("returns null on null input" => Ok(Strict(true)))]
    #[test_matrix("strict" => Ok(Strict(true)))]
    #[test_matrix("immutable" => Ok(Volatility(Immutable)))]
    #[test_matrix("stable" => Ok(Volatility(Stable)))]
    #[test_matrix("volatile" => Ok(Volatility(Volatile)))]
    #[test_matrix("external security definer" => Ok(Security(true)))]
    #[test_matrix("external security invoker" => Ok(Security(false)))]
    #[test_matrix("security definer" => Ok(Security(true)))]
    #[test_matrix("security invoker" => Ok(Security(false)))]
    #[test_matrix("leakproof" => Ok(Leakproof(true)))]
    #[test_matrix("not leakproof" => Ok(Leakproof(false)))]
    #[test_matrix("cost 10" => Ok(Cost(IntegerConst(10))))]
    #[test_matrix("rows 5" => Ok(Rows(IntegerConst(5))))]
    #[test_matrix("support some_function" => Ok(Support(vec!["some_function".into()])))]
    #[test_matrix("parallel safe" => Ok(Parallel("safe".into())))]
    #[test_matrix("set time zone local" => Ok(Set(TimeZone(Local))))]
    #[test_matrix("reset all" => Ok(Reset(All)))]
    fn test_common_func_opt_item(source: &str) -> scan::Result<AlterFunctionOption> {
        test_parser!(source, alter_function_option)
    }
}

use super::reset_stmt;
use super::set_rest_more;
use crate::alt;
use crate::combinators::any_name;
use crate::combinators::col_id;
use crate::combinators::core::Combinator;
use crate::combinators::signed_number;
use crate::seq;
use crate::ParserContext;
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
use pg_parser_core::scan;
