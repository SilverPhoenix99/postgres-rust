fn returning_clause(ctx: &mut ParserContext) -> scan::Result<ReturningClause> {

    /*
        RETURNING ( returning_with_clause )? target_list
    */

    let (_, options, exprs) = seq!(
        Returning,
        returning_with_clause.optional(),
        target_list
    ).parse(ctx)?;

    let mut clause = ReturningClause::new(exprs);
    clause.set_options(options);

    Ok(clause)
}

fn returning_with_clause(ctx: &mut ParserContext) -> scan::Result<Vec<ReturningOptionKind>> {

    /*
        WITH '(' returning_options ')'
    */

    let (_, options) = seq!(Kw::With, paren!(returning_options))
        .parse(ctx)?;

    Ok(options)
}

fn returning_options(ctx: &mut ParserContext) -> scan::Result<Vec<ReturningOptionKind>> {

    /*
        returning_option (',' returning_option )*
    */

    many!(sep = Comma, returning_option).parse(ctx)
}

/// Inlined: `returning_option_kind`
fn returning_option(ctx: &mut ParserContext) -> scan::Result<ReturningOptionKind> {

    /*
        (OLD | NEW) AS ColId
    */

    let (option, _, value) = seq!(
        alt!(Kw::Old, Kw::New),
        As,
        col_id
    ).parse(ctx)?;

    let option = if option == Kw::Old {
        Old(value)
    }
    else {
        New(value)
    };

    Ok(option)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{
        ExprNode::IntegerConst,
        ExprNode::StringConst,
        NamedValue,
        OneOrAll,
    };
    use test_case::test_case;

    #[test_case("returning 1 as foo, 'b' as bar" => Ok(
        ReturningClause::new(
            vec![
                OneOrAll::One(
                    NamedValue::new(Some("foo".into()), IntegerConst(1))
                ),
                OneOrAll::One(
                    NamedValue::new(Some("bar".into()), StringConst("b".into()))
                ),
            ]
        )
    ))]
    #[test_case("returning with(old as qux) *" => Ok(
        ReturningClause::new(vec![OneOrAll::All])
        .with_options(vec![Old("qux".into())])
    ))]
    fn test_returning_clause(source: &str) -> scan::Result<ReturningClause> {
        test_parser!(source, returning_clause)
    }

    #[test]
    fn test_returning_with_clause() {
        test_parser!(
            source = "with (old as foo, new as bar)",
            parser = returning_with_clause,
            expected = vec![
                Old("foo".into()),
                New("bar".into())
            ]
        )
    }

    #[test]
    fn test_returning_option() {
        test_parser!(
            source = "old as foo",
            parser = returning_option,
            expected = Old("foo".into())
        )
    }
}

use crate::alt;
use crate::combinators::col_id;
use crate::combinators::core::Combinator;
use crate::combinators::stmt::target_list;
use crate::context::ParserContext;
use crate::many;
use crate::paren;
use crate::seq;
use pg_ast::ReturningClause;
use pg_ast::ReturningOptionKind;
use pg_ast::ReturningOptionKind::New;
use pg_ast::ReturningOptionKind::Old;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::As;
use pg_lexer::Keyword::Returning;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
