pub(super) fn time(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
          CURRENT_DATE
        | CURRENT_TIME ( '(' ICONST ')' )?
        | CURRENT_TIMESTAMP ( '(' ICONST ')' )?
        | LOCALTIME ( '(' ICONST ')' )?
        | LOCALTIMESTAMP ( '(' ICONST ')' )?
    */

    alt!(
        Kw::CurrentDate.map(|_| CurrentDate),

        seq!(Kw::CurrentTime, precision.optional())
            .map(|(_, precision)| CurrentTime { precision }),

        seq!(Kw::CurrentTimestamp, precision.optional())
            .map(|(_, precision)| CurrentTimestamp { precision }),

        seq!(Kw::Localtime, precision.optional())
            .map(|(_, precision)| LocalTime { precision }),

        seq!(Kw::Localtimestamp, precision.optional())
            .map(|(_, precision)| LocalTimestamp { precision }),

    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::ExprNode;
    use test_case::test_case;

    #[test_case("current_date" => Ok(CurrentDate))]
    #[test_case("current_time" => Ok(CurrentTime { precision: None }))]
    #[test_case("current_time(3)" => Ok(CurrentTime { precision: Some(3) }))]
    #[test_case("current_timestamp" => Ok(CurrentTimestamp { precision: None }))]
    #[test_case("current_timestamp(7)" => Ok(CurrentTimestamp { precision: Some(7) }))]
    #[test_case("localtime" => Ok(LocalTime { precision: None }))]
    #[test_case("localtime(6)" => Ok(LocalTime { precision: Some(6) }))]
    #[test_case("localtimestamp" => Ok(LocalTimestamp { precision: None }))]
    #[test_case("localtimestamp(4)" => Ok(LocalTimestamp { precision: Some(4) }))]
    fn test_time(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, time)
    }
}

use crate::combinators::foundation::alt;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::precision::precision;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
use pg_ast::ExprNode::CurrentDate;
use pg_ast::ExprNode::CurrentTime;
use pg_ast::ExprNode::CurrentTimestamp;
use pg_ast::ExprNode::LocalTime;
use pg_ast::ExprNode::LocalTimestamp;
use pg_lexer::Keyword as Kw;
