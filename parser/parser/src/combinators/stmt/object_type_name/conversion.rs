pub(in crate::combinators::stmt) fn conversion(ctx: &mut ParserContext) -> scan::Result<QualifiedName> {

    /*
        CONVERSION any_name
    */

    let (_, name) = seq!(Conversion, any_name)
        .parse(ctx)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_conversion() {
        test_parser!(
            source = "conversion foo",
            parser = conversion,
            expected = vec!["foo".into()]
        )
    }
}

use crate::combinators::any_name;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Conversion;
use pg_parser_core::scan;
