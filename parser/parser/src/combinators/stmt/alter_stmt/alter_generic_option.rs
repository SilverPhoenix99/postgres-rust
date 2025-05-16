pub(super) fn alter_generic_options() -> impl Combinator<Output = Vec<GenericOptionKind>> {

    /*
        OPTIONS '(' alter_generic_option_list ')'
    */

    Options.and_right(between_paren(
        alter_generic_option_list(),
    ))
}

/// Post-condition: Vec is **Not** empty
fn alter_generic_option_list() -> impl Combinator<Output = Vec<GenericOptionKind>> {

    /*
        alter_generic_option ( ',' alter_generic_option )*
    */

    many_sep(Comma, alter_generic_option())
}

/// Alias: `alter_generic_option_elem`
fn alter_generic_option() -> impl Combinator<Output = GenericOptionKind> {

    /*
          SET generic_option_elem
        | ADD generic_option_elem
        | DROP ColLabel
        | generic_option_elem
    */

    match_first! {
        Kw::Set
            .and_right(generic_option())
            .map(Set),
        Kw::Add
            .and_right(generic_option())
            .map(Add),
        DropKw
            .and_right(col_label())
            .map(Drop),
        generic_option()
            .map(Unspecified)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use postgres_parser_ast::GenericOption;
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
        let source = "foo 'bar', drop x, add y '1', set z '2'";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = alter_generic_option_list().parse(&mut stream);

        let expected = vec![
            Unspecified(GenericOption::new("foo", "bar")),
            Drop("x".into()),
            Add(GenericOption::new("y", "1")),
            Set(GenericOption::new("z", "2"))
        ];

        assert_eq!(Ok(expected), actual);
    }

    #[test_case("set some_opt 'foo'", Set(GenericOption::new("some_opt", "foo")))]
    #[test_case("add some_opt 'foo'", Add(GenericOption::new("some_opt", "foo")))]
    #[test_case("drop some_opt", Drop("some_opt".into()))]
    #[test_case("some_opt 'foo'", Unspecified(GenericOption::new("some_opt", "foo")))]
    fn test_alter_generic_option(source: &str, expected: GenericOptionKind) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = alter_generic_option().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::combinators::between_paren;
use crate::combinators::col_label;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::generic_option;
use postgres_parser_ast::GenericOptionKind;
use postgres_parser_ast::GenericOptionKind::Add;
use postgres_parser_ast::GenericOptionKind::Drop;
use postgres_parser_ast::GenericOptionKind::Set;
use postgres_parser_ast::GenericOptionKind::Unspecified;
use postgres_parser_lexer::Keyword as Kw;
use postgres_parser_lexer::Keyword::DropKw;
use postgres_parser_lexer::Keyword::Options;
use postgres_parser_lexer::OperatorKind::Comma;
