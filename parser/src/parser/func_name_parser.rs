pub(in crate::parser) fn func_name() -> impl Combinator<Output = QualifiedName> {

    /*
          type_func_name_keyword
        | col_name_keyword attrs
        | unreserved_keyword ( attrs )?
        | IDENT ( attrs )?
    */

    match_first!{
        TypeFuncName
            .map(|kw| vec![kw.details().text().into()]),
        attrs(
            or(
                Unreserved
                    .map(|kw| kw.details().text().into()),
                identifier()
                    .map(From::from)
            )
        ),
        located(attrs(
            ColumnName
                .map(|kw| kw.details().text().into())
            ))
            .map_result(|result| {
                let (name, loc) = result?;
                if name.len() == 1 {
                    return Err(syntax_err(loc).into())
                }
                Ok(name)
            }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

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

use crate::lexer::KeywordCategory::ColumnName;
use crate::lexer::KeywordCategory::TypeFuncName;
use crate::lexer::KeywordCategory::Unreserved;
use crate::parser::attrs;
use crate::parser::combinators::identifier;
use crate::parser::combinators::match_first;
use crate::parser::combinators::or;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::error::syntax_err;
use crate::parser::located_combinator::located;
use crate::parser::QualifiedName;
