pub fn alter_generic_options(ctx: &mut ParserContext) -> scan::Result<Vec<GenericOptionKind>> {

    /*
        OPTIONS '(' alter_generic_option_list ')'
    */

    let (_, options) = seq!(
        Options,
        paren!(alter_generic_option_list)
    ).parse(ctx)?;

    Ok(options)
}

fn alter_generic_option_list(ctx: &mut ParserContext) -> scan::Result<Vec<GenericOptionKind>> {

    /*
        alter_generic_option ( ',' alter_generic_option )*
    */

    many!(sep = Comma, alter_generic_option).parse(ctx)
}

/// Alias: `alter_generic_option_elem`
fn alter_generic_option(ctx: &mut ParserContext) -> scan::Result<GenericOptionKind> {

    /*
          SET generic_option_elem
        | ADD generic_option_elem
        | DROP ColLabel
        | generic_option_elem
    */

    alt!(
        seq!(Kw::Set, generic_option)
            .map(|(_, opt)| Set(opt)),
        seq!(Kw::Add, generic_option)
            .map(|(_, opt)| Add(opt)),
        seq!(DropKw, col_label)
            .map(|(_, opt)| Drop(opt)),
        generic_option
            .map(Unspecified)
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_ast::GenericOption;
    use pg_combinators::test_parser;
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

use crate::generic_option;
use pg_ast::GenericOptionKind;
use pg_ast::GenericOptionKind::Add;
use pg_ast::GenericOptionKind::Drop;
use pg_ast::GenericOptionKind::Set;
use pg_ast::GenericOptionKind::Unspecified;
use pg_combinators::alt;
use pg_combinators::many;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::DropKw;
use pg_lexer::Keyword::Options;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_sink_combinators::col_label;
