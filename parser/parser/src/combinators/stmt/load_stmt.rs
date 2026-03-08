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
    use crate::test_parser;

    #[test]
    fn test_load_stmt() {
        test_parser!(
            source = "load 'test string'",
            parser = load_stmt,
            expected = "test string"
        )
    }
}

use crate::combinators::core::string;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_lexer::Keyword::Load;
use pg_parser_core::scan;
