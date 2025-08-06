pub(super) fn xmlexists_argument(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        Several variants are allowed for SQL and other compatibility.

        PASSING ( xml_passing_mech )? c_expr ( xml_passing_mech )?
    */

    let (_, _, expr, _) = seq!(
        Passing,
        xml_passing_mech.optional(),
        expr_primary,
        xml_passing_mech.optional()
    ).parse(stream)?;

    Ok(expr)
}

fn xml_passing_mech(stream: &mut TokenStream) -> scan::Result<()> {

    /*
        BY ( REF | VALUE )
    */

    if ! matches!(stream.peek2(), Ok((K(By), K(RefKw | Value)))) {
        return no_match(stream)
    }

    stream.skip(2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::StringConst,
        scan::Error::NoMatch,
    };

    #[test_case("passing by ref 'foo' by value")]
    #[test_case("passing 'foo' by ref")]
    #[test_case("passing by value 'foo'")]
    #[test_case("passing 'foo'")]
    fn test_xmlexists_argument(source: &str) {
        test_parser!(source, xmlexists_argument, StringConst("foo".into()))
    }

    #[test_case("by ref" => Ok(()))]
    #[test_case("by value" => Ok(()))]
    #[test_case("by" => matches Err(NoMatch(_)))]
    fn test_xml_passing_mech(source: &str) -> scan::Result<()> {
        test_parser!(source, xml_passing_mech)
    }
}

use crate::combinators::expr::expr_primary;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::no_match;
use pg_ast::ExprNode;
use pg_lexer::Keyword::By;
use pg_lexer::Keyword::Passing;
use pg_lexer::Keyword::RefKw;
use pg_lexer::Keyword::Value;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
use pg_parser_core::stream::TokenValue::Keyword as K;
