/// Alias: `opt_target_list`
pub(super) fn target_list(ctx: &mut ParserContext) -> scan::Result<Vec<OneOrAll<NamedValue>>> {

    /*
        target_el ( ',' target_el )*
    */

    many!(sep = Comma, target_el).parse(ctx)
}

fn target_el(ctx: &mut ParserContext) -> scan::Result<OneOrAll<NamedValue>> {

    /*
          '*'
        | a_expr ( target_el_alias )?
    */

    alt!(
        Mul.map(|_| OneOrAll::All),
        seq!(
            a_expr,
            target_el_alias.optional()
        )
            .map(|(value, name)| {
                OneOrAll::One(NamedValue::new(name, value))
            })
    ).parse(ctx)
}

fn target_el_alias(ctx: &mut ParserContext) -> scan::Result<Str> {

    /*
          AS ColLabel
        | BareColLabel
    */

    alt!(
        seq!(As, col_label).map(|(_, label)| label),
        bare_col_label
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::IntegerConst;
    use test_case::test_case;

    #[test_case("1, 2 as x, *" => matches Ok(_))]
    fn test_target_list(source: &str) -> scan::Result<Vec<OneOrAll<NamedValue>>> {
        test_parser!(source, target_list)
    }

    #[test_case("*" => Ok(OneOrAll::All))]
    #[test_case("1" => Ok(OneOrAll::One(
        NamedValue::unnamed(IntegerConst(1))
    )))]
    #[test_case("2 foo" => Ok(OneOrAll::One(
        NamedValue::new(Some("foo".into()), IntegerConst(2))
    )))]
    fn test_target_el(source: &str) -> scan::Result<OneOrAll<NamedValue>> {
        test_parser!(source, target_el)
    }

    #[test_case("as foo" => Ok("foo".into()))]
    #[test_case("bar" => Ok("bar".into()))]
    fn test_target_el_alias(source: &str) -> scan::Result<Str> {
        test_parser!(source, target_el_alias)
    }
}

use crate::alt;
use crate::combinators::bare_col_label;
use crate::combinators::col_label;
use crate::combinators::core::Combinator;
use crate::combinators::expr::a_expr;
use crate::context::ParserContext;
use crate::many;
use crate::seq;
use pg_ast::NamedValue;
use pg_ast::OneOrAll;
use pg_basics::Str;
use pg_lexer::Keyword::As;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::Mul;
use pg_parser_core::scan;
