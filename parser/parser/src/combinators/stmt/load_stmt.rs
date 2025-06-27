/// Alias: `LoadStmt`
pub(super) fn load_stmt(stream: &mut TokenStream) -> scan::Result<Box<str>> {

    /*
        LOAD SCONST
    */

    let (_, lib_name) = seq!(stream => Load, string)?;

    Ok(lib_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_load_stmt() {
        test_parser!(
            source = "load 'test string'",
            parser = load_stmt,
            expected = "test string"
        )
    }
}

use crate::combinators::foundation::seq;
use crate::combinators::foundation::string;
use crate::scan;
use crate::stream::TokenStream;
use pg_lexer::Keyword::Load;
