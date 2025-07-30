pub(super) fn json_quotes_clause(stream: &mut TokenStream) -> scan::Result<JsonQuotes> {

    /*
        ( KEEP | OMIT ) QUOTES ( ON SCALAR STRING )?
    */

    let (quotes, ..) = (
        or((
            Kw::Keep.map(|_| Keep),
            Kw::Omit.map(|_| Omit),
        )),
        Quotes,
        (On, Scalar, StringKw).optional()
    ).parse(stream)?;

    Ok(quotes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("keep quotes" => Ok(JsonQuotes::Keep))]
    #[test_case("keep quotes on scalar string" => Ok(JsonQuotes::Keep))]
    #[test_case("omit quotes" => Ok(JsonQuotes::Omit))]
    #[test_case("omit quotes on scalar string" => Ok(JsonQuotes::Omit))]
    fn test_json_quotes_clause(source: &str) -> scan::Result<JsonQuotes> {
        test_parser!(source, json_quotes_clause)
    }
}

use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::JsonQuotes;
use pg_ast::JsonQuotes::Keep;
use pg_ast::JsonQuotes::Omit;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::On;
use pg_lexer::Keyword::Quotes;
use pg_lexer::Keyword::Scalar;
use pg_lexer::Keyword::StringKw;
