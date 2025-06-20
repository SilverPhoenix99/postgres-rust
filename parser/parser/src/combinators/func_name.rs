pub(super) fn func_name() -> impl Combinator<Output = QualifiedName> {

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

    parser(|stream|
        choice!(stream,
            {
                TypeFuncName.parse(stream)
                    .map(|kw| vec![kw.into()])
            },
            {
                attrs!(stream,
                    choice!(stream,
                        Unreserved.parse(stream).map(Str::from),
                        identifier(stream).map(Str::from)
                    )
                )
            },
            {
                let (name, loc) = located!(stream,
                    attrs(ColumnName.map(From::from)).parse(stream)
                )?;

                if name.len() == 1 {
                    return Err(syntax(loc))
                }
                Ok(name)
            },
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_type_func_name_keyword() {
        let source = "authorization";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(vec!["authorization".into()]), func_name().parse(&mut stream));
    }

    #[test]
    fn test_col_name_keyword() {
    let source = "trim.something";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(vec!["trim".into(), "something".into()]), func_name().parse(&mut stream));
    }

    #[test]
    fn test_unreserved_keyword() {
        let source = "attribute inline.some_thing";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(vec!["attribute".into()]), func_name().parse(&mut stream));
        assert_eq!(Ok(vec!["inline".into(), "some_thing".into()]), func_name().parse(&mut stream));
    }

    #[test]
    fn test_identifier() {
        let source = "some_ident another_ident.something";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(vec!["some_ident".into()]), func_name().parse(&mut stream));
        assert_eq!(Ok(vec!["another_ident".into(), "something".into()]), func_name().parse(&mut stream));
    }
}

use crate::combinators::attrs;
use crate::combinators::foundation::choice;
use crate::combinators::foundation::identifier;
use crate::combinators::foundation::located;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::v2::attrs;
use pg_basics::QualifiedName;
use pg_basics::Str;
use pg_elog::syntax;
use pg_lexer::KeywordCategory::ColumnName;
use pg_lexer::KeywordCategory::TypeFuncName;
use pg_lexer::KeywordCategory::Unreserved;
