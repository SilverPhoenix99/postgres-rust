/// Aliases:
/// * `utility_option_list`
/// * `opt_utility_option_list`
pub(super) fn utility_options(ctx: &mut ParserContext) -> scan::Result<Vec<UtilityOption>> {

    /*
        '(' utility_option_list ')'
    */

    paren!(utility_option_list)
        .parse(ctx)
}

fn utility_option_list(ctx: &mut ParserContext) -> scan::Result<Vec<UtilityOption>> {

    /*
        utility_option ( ',' utility_option )*
    */

    many!(sep = Comma, utility_option).parse(ctx)
}

/// Alias: `utility_option_elem`
fn utility_option(ctx: &mut ParserContext) -> scan::Result<UtilityOption> {

    /*
        utility_option_name ( var_value )?
    */

    let (name, value) = seq!(
        utility_option_name,
        var_value.optional()
    ).parse(ctx)?;

    let mut option = UtilityOption::new(name);
    option.set_value(value);

    Ok(option)
}

fn utility_option_name(ctx: &mut ParserContext) -> scan::Result<UtilityOptionName> {

    /*
          NonReservedWord
        | analyze_keyword
        | FORMAT
    */

    alt!(
        Kw::Format.map(|_| Format),
        analyze_keyword.map(|_| Analyze),
        non_reserved_word.map(Generic)
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_matrix;

    #[test]
    fn test_utility_options() {
        test_parser!(
            source = "(analyze, format)",
            parser = utility_options,
            expected = vec![
                Analyze.into(),
                Format.into()
            ]
        )
    }

    #[test]
    fn test_utility_option_list() {
        test_parser!(
            source = "analyze false, format csv, bar true",
            parser = utility_option_list,
            expected = vec![
                UtilityOption::new(Analyze).with_value(false),
                UtilityOption::new(Format).with_value("csv"),
                UtilityOption::new(Generic("bar".into())).with_value(true)
            ]
        )
    }

    #[test_matrix("format 'json'" => Ok(
        UtilityOption::new(Format)
            .with_value("json")
    ))]
    #[test_matrix("analyse" => Ok(Analyze.into()))]
    #[test_matrix("foo false" => Ok(
        UtilityOption::new(Generic("foo".into()))
            .with_value(false)
    ))]
    fn test_utility_option(source: &str)  -> scan::Result<UtilityOption> {
        test_parser!(source, utility_option)
    }

    #[test_matrix("analyze" => Ok(Analyze))]
    #[test_matrix("analyse" => Ok(Analyze))]
    #[test_matrix("format" => Ok(Format))]
    #[test_matrix("xxyyzz" => Ok(Generic("xxyyzz".into())))]
    #[test_matrix("breadth" => Ok(Generic("breadth".into())))]
    #[test_matrix("boolean" => Ok(Generic("boolean".into())))]
    #[test_matrix("authorization" => Ok(Generic("authorization".into())))]
    fn test_utility_option_name(source: &str) -> scan::Result<UtilityOptionName> {
        test_parser!(source, utility_option_name)
    }
}

use crate::alt;
use crate::combinators::analyze_keyword;
use crate::combinators::core::Combinator;
use crate::combinators::non_reserved_word;
use crate::combinators::var_value;
use crate::many;
use crate::paren;
use crate::seq;
use crate::ParserContext;
use pg_ast::UtilityOption;
use pg_ast::UtilityOptionName;
use pg_ast::UtilityOptionName::Analyze;
use pg_ast::UtilityOptionName::Format;
use pg_ast::UtilityOptionName::Generic;
use pg_lexer::Keyword as Kw;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
