/// Alias: `utility_option_list`
pub(super) fn utility_options(stream: &mut TokenStream) -> scan::Result<Vec<UtilityOption>> {

    /*
        '(' utility_option_list ')'
    */

    between_paren(utility_option_list)
        .parse(stream)
}

fn utility_option_list(stream: &mut TokenStream) -> scan::Result<Vec<UtilityOption>> {

    /*
        utility_option ( ',' utility_option )*
    */

    many_sep(Comma, utility_option).parse(stream)
}

/// Alias: `utility_option_elem`
fn utility_option(stream: &mut TokenStream) -> scan::Result<UtilityOption> {

    /*
        utility_option_name ( var_value )?
    */

    let (name, value) = (
        utility_option_name,
        var_value.optional()
    ).parse(stream)?;

    Ok(UtilityOption::new(name, value))
}

fn utility_option_name(stream: &mut TokenStream) -> scan::Result<UtilityOptionName> {

    /*
          NonReservedWord
        | analyze_keyword
        | FORMAT
    */

    or((
        Kw::Format.map(|_| Format),
        analyze_keyword.map(|_| Analyze),
        non_reserved_word.map(Generic)
    )).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test]
    fn test_utility_options() {
        test_parser!(
            source = "(analyze, format)",
            parser = utility_options,
            expected = vec![
                UtilityOption::new(Analyze, None),
                UtilityOption::new(Format, None)
            ]
        )
    }

    #[test]
    fn test_utility_option_list() {
        test_parser!(
            source = "analyze false, format csv, bar true",
            parser = utility_option_list,
            expected = vec![
                UtilityOption::new(Analyze, Some(false.into())),
                UtilityOption::new(Format, Some("csv".into())),
                UtilityOption::new(Generic("bar".into()), Some(true.into()))
            ]
        )
    }

    #[test_case("format 'json'", UtilityOption::new(Format, Some("json".into())))]
    #[test_case("analyse", UtilityOption::new(Analyze, None))]
    #[test_case("foo false", UtilityOption::new(Generic("foo".into()), Some(false.into())))]
    fn test_utility_option(source: &str, expected: UtilityOption) {
        test_parser!(source, utility_option, expected)
    }

    #[test_case("analyze", Analyze)]
    #[test_case("analyse", Analyze)]
    #[test_case("format", Format)]
    #[test_case("xxyyzz", Generic("xxyyzz".into()))]
    #[test_case("breadth", Generic("breadth".into()))]
    #[test_case("boolean", Generic("boolean".into()))]
    #[test_case("authorization", Generic("authorization".into()))]
    fn test_utility_option_name(source: &str, expected: UtilityOptionName) {
        test_parser!(source, utility_option_name, expected)
    }
}

use crate::combinators::foundation::between_paren;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::non_reserved_word;
use crate::combinators::stmt::analyze_keyword;
use crate::combinators::var_value;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::UtilityOption;
use pg_ast::UtilityOptionName;
use pg_ast::UtilityOptionName::Analyze;
use pg_ast::UtilityOptionName::Format;
use pg_ast::UtilityOptionName::Generic;
use pg_lexer::Keyword as Kw;
use pg_lexer::OperatorKind::Comma;
