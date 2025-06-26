pub(super) fn alter_generic_options() -> impl Combinator<Output = Vec<GenericOptionKind>> {

    /*
        OPTIONS '(' alter_generic_option_list ')'
    */

    Options.and_right(
        parser(|stream| between!(paren : stream =>
            alter_generic_option_list(stream)
        ))
    )
}

fn alter_generic_option_list(stream: &mut TokenStream) -> scan::Result<Vec<GenericOptionKind>> {

    /*
        alter_generic_option ( ',' alter_generic_option )*
    */

    many!(stream => sep = Comma, alter_generic_option)
}

/// Alias: `alter_generic_option_elem`
fn alter_generic_option(stream: &mut TokenStream) -> scan::Result<GenericOptionKind> {

    /*
          SET generic_option_elem
        | ADD generic_option_elem
        | DROP ColLabel
        | generic_option_elem
    */

    let parser = choice!(
        (Kw::Set, generic_option)
            .right()
            .map(Set),
        (Kw::Add, generic_option)
            .right()
            .map(Add),
        (DropKw, col_label)
            .right()
            .map(Drop),
        generic_option
            .map(Unspecified)
    );

    parser.parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::test_parser;
    use crate::tests::DEFAULT_CONFIG;
    use pg_ast::GenericOption;
    use test_case::test_case;

    #[test]
    fn test_alter_generic_options() {
        let source = "options ( add a 'b', set c 'd' )";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = alter_generic_options().parse(&mut stream);

        let expected = vec![
            Add(GenericOption::new("a", "b")),
            Set(GenericOption::new("c", "d"))
        ];

        assert_eq!(Ok(expected), actual);
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
use crate::combinators::foundation::between;
use crate::combinators::foundation::choice;
use crate::combinators::foundation::many;
use crate::combinators::foundation::parser;
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
