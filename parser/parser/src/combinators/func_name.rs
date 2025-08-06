pub(super) fn func_name(stream: &mut TokenStream) -> scan::Result<QualifiedName> {

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
    ).parse(stream)
}

fn column_name(stream: &mut TokenStream) -> scan::Result<QualifiedName> {

    let loc = stream.current_location();
    let name = attrs!(ColumnName.map(Str::from)).parse(stream)?;

    if name.len() == 1 {
        return Err(syntax(loc))
    }

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::test_parser;
    use crate::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("authorization", vec!["authorization".into()])]
    #[test_case("trim.something", vec!["trim".into(), "something".into()])]
    fn test_type_func_name(source: &str, expected: QualifiedName) {
        test_parser!(source, func_name, expected)
    }

    #[test]
    fn test_unreserved_keyword() {
        let source = "attribute inline.some_thing";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(vec!["attribute".into()]), func_name(&mut stream));
        assert_eq!(Ok(vec!["inline".into(), "some_thing".into()]), func_name(&mut stream));
    }

    #[test]
    fn test_identifier() {
        let source = "some_ident another_ident.something";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(vec!["some_ident".into()]), func_name(&mut stream));
        assert_eq!(Ok(vec!["another_ident".into(), "something".into()]), func_name(&mut stream));
    }
}

use crate::combinators::attrs::attrs;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::identifier;
use crate::combinators::foundation::Combinator;
use crate::stream::TokenStream;
use pg_basics::QualifiedName;
use pg_basics::Str;
use pg_lexer::KeywordCategory::ColumnName;
use pg_lexer::KeywordCategory::TypeFuncName;
use pg_lexer::KeywordCategory::Unreserved;
use pg_parser_core::scan;
use pg_parser_core::syntax;
