/// Alias: `LoadStmt`
pub(super) fn load_stmt(ctx: &mut ParserContext) -> scan::Result<Box<str>> {

    /*
        LOAD SCONST
    */

    let (_, lib_name) = seq!(Load, string)
        .parse(ctx)?;

    Ok(lib_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_load_stmt() {
        test_parser!(
            source = "load 'test string'",
            parser = load_stmt,
            expected = "test string"
        )
    }
}

use pg_combinators::seq;
use pg_combinators::string;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Load;
use pg_parser_core::scan;
