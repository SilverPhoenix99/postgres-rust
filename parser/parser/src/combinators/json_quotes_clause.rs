/// Alias: `json_quotes_clause_opt`
pub(super) fn json_quotes_clause(ctx: &mut ParserContext) -> scan::Result<JsonQuotes> {

    /*
        ( KEEP | OMIT ) QUOTES ( ON SCALAR STRING )?
    */

    let (quotes, ..) = seq!(
        alt!(
            Kw::Keep.map(|_| Keep),
            Kw::Omit.map(|_| Omit),
        ),
        Quotes,
        seq!(On, Scalar, StringKw).optional()
    ).parse(ctx)?;

    Ok(quotes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_matrix;

    #[test_matrix("keep quotes" => Ok(Keep))]
    #[test_matrix("keep quotes on scalar string" => Ok(Keep))]
    #[test_matrix("omit quotes" => Ok(Omit))]
    #[test_matrix("omit quotes on scalar string" => Ok(Omit))]
    fn test_json_quotes_clause(source: &str) -> scan::Result<JsonQuotes> {
        test_parser!(source, json_quotes_clause)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_ast::JsonQuotes;
use pg_ast::JsonQuotes::Keep;
use pg_ast::JsonQuotes::Omit;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::On;
use pg_lexer::Keyword::Quotes;
use pg_lexer::Keyword::Scalar;
use pg_lexer::Keyword::StringKw;
use pg_parser_core::scan;
