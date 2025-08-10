pub fn conversion(ctx: &mut ParserContext) -> scan::Result<QualifiedName> {

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
    use pg_combinators::test_parser;

    #[test]
    fn test_conversion() {
        test_parser!(
            source = "conversion foo",
            parser = conversion,
            expected = vec!["foo".into()]
        )
    }
}

use crate::any_name;
use pg_basics::QualifiedName;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Conversion;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
