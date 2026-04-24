pub(super) fn xmlexists_argument(ctx: &mut ParserContext) -> scan::Result<ExprNode> {

    /*
        Several variants are allowed for SQL and other compatibility.

        PASSING ( xml_passing_mech )? c_expr ( xml_passing_mech )?
    */

    let (_, _, expr, _) = seq!(
        Passing,
        xml_passing_mech.optional(),
        expr_primary,
        xml_passing_mech.optional()
    ).parse(ctx)?;

    Ok(expr)
}

fn xml_passing_mech(ctx: &mut ParserContext) -> scan::Result<()> {

    /*
        BY ( REF | VALUE )
    */

    if ! matches!(ctx.stream_mut().peek_n::<2>(), Ok([K(By), K(RefKw | Value)])) {
        return no_match(ctx)
    }

    ctx.stream_mut().skip(2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode::StringConst;
    use scan::Error::NoMatch;
    use test_case::test_matrix;

    #[test_matrix([
        "passing by ref 'foo' by value",
        "passing 'foo' by ref",
        "passing by value 'foo'",
        "passing 'foo'",
    ])]
    fn test_xmlexists_argument(source: &str) {
        test_parser!(source, xmlexists_argument, StringConst("foo".into()))
    }

    #[test_matrix("by ref" => Ok(()))]
    #[test_matrix("by value" => Ok(()))]
    #[test_matrix("by" => matches Err(NoMatch(_)))]
    fn test_xml_passing_mech(source: &str) -> scan::Result<()> {
        test_parser!(source, xml_passing_mech)
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::expr::expr_primary;
use crate::no_match;
use crate::seq;
use crate::ParserContext;
use pg_ast::ExprNode;
use pg_lexer::Keyword::By;
use pg_lexer::Keyword::Passing;
use pg_lexer::Keyword::RefKw;
use pg_lexer::Keyword::Value;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenValue::Keyword as K;
