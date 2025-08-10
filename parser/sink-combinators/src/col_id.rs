/// Alias: `columnList`
pub fn name_list(ctx: &mut ParserContext) -> scan::Result<Vec<Str>> {

    /*
        col_id ( ',' col_id )*
    */

    many!(sep = Comma, col_id).parse(ctx)
}

pub fn var_name(ctx: &mut ParserContext) -> scan::Result<QualifiedName> {

    /*
        col_id ( '.' col_id )*
    */

    many!(sep = Dot, col_id).parse(ctx)
}

/// Aliases:
/// * `ColId`
/// * `name`
/// * `opt_single_name`
pub fn col_id(ctx: &mut ParserContext) -> scan::Result<Str> {

    alt!(
        identifier.map(From::from),
        Unreserved.map(From::from),
        ColumnName.map(From::from)
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_col_id() {
        let source = "cascaded xxyyzz coalesce";
        let mut ctx = ParserContext::from(source);

        assert_eq!(Ok("cascaded".into()), col_id(&mut ctx));
        assert_eq!(Ok("xxyyzz".into()), col_id(&mut ctx));
        assert_eq!(Ok("coalesce".into()), col_id(&mut ctx));
    }
}

use pg_basics::QualifiedName;
use pg_basics::Str;
use pg_combinators::alt;
use pg_combinators::identifier;
use pg_combinators::many;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::KeywordCategory::ColumnName;
use pg_lexer::KeywordCategory::Unreserved;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::Dot;
use pg_parser_core::scan;
