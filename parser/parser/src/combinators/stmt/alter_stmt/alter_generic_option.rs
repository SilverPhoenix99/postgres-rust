pub(super) fn alter_generic_options(stream: &mut TokenStream) -> scan::Result<Vec<GenericOptionKind>> {

    /*
        OPTIONS '(' alter_generic_option_list ')'
    */

    let (_, options) = (
        Options,
        between_paren(alter_generic_option_list)
    ).parse(stream)?;

    Ok(options)
}

fn alter_generic_option_list(stream: &mut TokenStream) -> scan::Result<Vec<GenericOptionKind>> {

    /*
        alter_generic_option ( ',' alter_generic_option )*
    */

    many_sep(Comma, alter_generic_option).parse(stream)
}

/// Alias: `alter_generic_option_elem`
fn alter_generic_option(stream: &mut TokenStream) -> scan::Result<GenericOptionKind> {

    /*
          SET generic_option_elem
        | ADD generic_option_elem
        | DROP ColLabel
        | generic_option_elem
    */

    or((
        (Kw::Set, generic_option)
            .map(|(_, opt)| Set(opt)),
        (Kw::Add, generic_option)
            .map(|(_, opt)| Add(opt)),
        (DropKw, col_label)
            .map(|(_, opt)| Drop(opt)),
        generic_option
            .map(Unspecified)
    )).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::GenericOption;
    use test_case::test_case;

    #[test]
    fn test_alter_generic_options() {
        test_parser!(
            source = "options ( add a 'b', set c 'd' )",
            parser = alter_generic_options,
            expected = vec![
                Add(GenericOption::new("a", "b")),
                Set(GenericOption::new("c", "d"))
            ]
        )
    }

    #[test]
    fn test_alter_generic_option_list() {
        test_parser!(
            source = "foo 'bar', drop x, add y '1', set z '2'",
            parser = alter_generic_option_list,
            expected = vec![
                Unspecified(GenericOption::new("foo", "bar")),
                Drop("x".into()),
                Add(GenericOption::new("y", "1")),
                Set(GenericOption::new("z", "2"))
            ]
        )
    }

    #[test_case("set some_opt 'foo'", Set(GenericOption::new("some_opt", "foo")))]
    #[test_case("add some_opt 'foo'", Add(GenericOption::new("some_opt", "foo")))]
    #[test_case("drop some_opt", Drop("some_opt".into()))]
    #[test_case("some_opt 'foo'", Unspecified(GenericOption::new("some_opt", "foo")))]
    fn test_alter_generic_option(source: &str, expected: GenericOptionKind) {
        test_parser!(source, alter_generic_option, expected)
    }
}

use crate::combinators::col_label;
use crate::combinators::foundation::between_paren;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::generic_option;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::GenericOptionKind;
use pg_ast::GenericOptionKind::Add;
use pg_ast::GenericOptionKind::Drop;
use pg_ast::GenericOptionKind::Set;
use pg_ast::GenericOptionKind::Unspecified;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::DropKw;
use pg_lexer::Keyword::Options;
use pg_lexer::OperatorKind::Comma;
