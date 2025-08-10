/// Alias: `NonReservedWord`
pub fn non_reserved_word(ctx: &mut ParserContext) -> scan::Result<Str> {
    alt!(
        identifier.map(Str::from),
        Unreserved.map(Str::from),
        ColumnName.map(Str::from),
        TypeFuncName.map(Str::from),
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_reserved_word() {
        let source = "breadth xxyyzz boolean authorization";
        let mut ctx = ParserContext::from(source);

        assert_eq!(Ok("breadth".into()), non_reserved_word(&mut ctx));
        assert_eq!(Ok("xxyyzz".into()), non_reserved_word(&mut ctx));
        assert_eq!(Ok("boolean".into()), non_reserved_word(&mut ctx));
        assert_eq!(Ok("authorization".into()), non_reserved_word(&mut ctx));
    }
}

use pg_basics::Str;
use pg_combinators::alt;
use pg_combinators::identifier;
use pg_combinators::Combinator;
use pg_lexer::KeywordCategory::ColumnName;
use pg_lexer::KeywordCategory::TypeFuncName;
use pg_lexer::KeywordCategory::Unreserved;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
