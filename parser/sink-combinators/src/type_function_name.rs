pub fn type_function_name(ctx: &mut ParserContext) -> scan::Result<Str> {
    alt!(
        identifier.map(Str::from),
        Unreserved.map(Str::from),
        TypeFuncName.map(Str::from),
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_function_name() {
        let source = "before xxyyzz collation";
        let mut ctx = ParserContext::from(source);

        assert_eq!(Ok("before".into()), type_function_name(&mut ctx));
        assert_eq!(Ok("xxyyzz".into()), type_function_name(&mut ctx));
        assert_eq!(Ok("collation".into()), type_function_name(&mut ctx));
    }
}

use pg_basics::Str;
use pg_combinators::alt;
use pg_combinators::identifier;
use pg_combinators::Combinator;
use pg_lexer::KeywordCategory::TypeFuncName;
use pg_lexer::KeywordCategory::Unreserved;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
