pub(in crate::combinators::expr::expr_primary)
fn null_treatment(ctx: &mut ParserContext) -> scan::Result<NullTreatment> {

    /*
          IGNORE NULLS
        | RESPECT NULLS
    */

    let (null_treatment, _)=  seq!(
        alt!(
            Kw::Ignore.map(|_| Ignore),
            Kw::Respect.map(|_| Respect),
        ),
        Nulls
    ).parse(ctx)?;

    Ok(null_treatment)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;

    #[test_case("ignore nulls" => Ok(NullTreatment::Ignore))]
    #[test_case("respect nulls" => Ok(NullTreatment::Respect))]
    fn test_null_treatment(source: &str) -> scan::Result<NullTreatment> {
        test_parser!(source, null_treatment)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_ast::NullTreatment;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Nulls;
use pg_parser_core::scan;
use NullTreatment::Ignore;
use NullTreatment::Respect;
