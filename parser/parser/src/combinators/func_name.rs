pub(super) fn func_name(ctx: &mut ParserContext) -> scan::Result<QualifiedName> {

    /*
        Original production:
              type_function_name
            | ColId attrs

        The original production used `indirection` instead of `attrs`,
        but the only valid rule from `indirection` is: `'.' ColLabel`.
        See [function_with_argtypes](https://github.com/postgres/postgres/blob/97173536ed4b1c29dce0dc4119db136e142f60a2/src/backend/parser/gram.y#L17267).

        Refactored production:
              type_func_name_keyword
            | col_name_keyword attrs
            | unreserved_keyword ( attrs )?
            | IDENT ( attrs )?
    */

    alt!(
        TypeFuncName.map(|kw| vec![kw.into()]),
        attrs!(
            alt!(
                Unreserved.map(Str::from),
                identifier.map(Str::from)
            )
        ),
        column_name
    ).parse(ctx)
}

fn column_name(ctx: &mut ParserContext) -> scan::Result<QualifiedName> {

    let Located(name, loc) = located!(
        attrs!(ColumnName.map(Str::from))
    ).parse(ctx)?;

    if name.len() == 1 {
        return Err(syntax(loc))
    }

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("authorization", vec!["authorization".into()])]
    #[test_case("trim.something", vec!["trim".into(), "something".into()])]
    fn test_type_func_name(source: &str, expected: QualifiedName) {
        test_parser!(source, func_name, expected)
    }

    #[test]
    fn test_unreserved_keyword() {
        let source = "attribute inline.some_thing";
        let mut ctx = ParserContext::from(source);
        assert_eq!(Ok(vec!["attribute".into()]), func_name(&mut ctx));
        assert_eq!(Ok(vec!["inline".into(), "some_thing".into()]), func_name(&mut ctx));
    }

    #[test]
    fn test_identifier() {
        let source = "some_ident another_ident.something";
        let mut ctx = ParserContext::from(source);
        assert_eq!(Ok(vec!["some_ident".into()]), func_name(&mut ctx));
        assert_eq!(Ok(vec!["another_ident".into(), "something".into()]), func_name(&mut ctx));
    }
}

use pg_basics::Located;
use pg_basics::QualifiedName;
use pg_basics::Str;
use pg_combinators::alt;
use pg_combinators::identifier;
use pg_combinators::located;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::KeywordCategory::ColumnName;
use pg_lexer::KeywordCategory::TypeFuncName;
use pg_lexer::KeywordCategory::Unreserved;
use pg_parser_core::scan;
use pg_parser_core::syntax;
use pg_sink_combinators::attrs;
