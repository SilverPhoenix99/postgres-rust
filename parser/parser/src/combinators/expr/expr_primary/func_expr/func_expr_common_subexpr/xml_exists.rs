pub(super) fn xml_exists(stream: &mut TokenStream) -> scan::Result<XmlExists> {

    /*
        XMLEXISTS '(' c_expr xmlexists_argument ')'
    */

    if ! matches!(stream.peek2(), Ok((K(Xmlexists), Op(OpenParenthesis)))) {
        return no_match(stream);
    }

    let (_, (path_spec, content)) = seq!(skip(1), paren!(seq!(
        expr_primary,
        xmlexists_argument
    ))).parse(stream)?;

    let expr = XmlExists::new(path_spec, content);
    Ok(expr)
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

    #[test_case("xmlexists('foo' passing 'bar')" => Ok(
        XmlExists::new(
            StringConst("foo".into()),
            StringConst("bar".into())
        )
    ))]
    #[test_case("xmlexists" => matches Err(NoMatch(_)))]
    #[test_case("xmlexists 1" => matches Err(NoMatch(_)))]
    fn test_xml_exists(source: &str) -> scan::Result<XmlExists> {
        test_parser!(source, xml_exists)
    }
}

use crate::combinators::expr::expr_primary;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use crate::combinators::foundation::Combinator;
use crate::combinators::xmlexists_argument;
use crate::no_match;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword as K;
use crate::stream::TokenValue::Operator as Op;
use pg_ast::XmlExists;
use pg_lexer::Keyword::Xmlexists;
use pg_lexer::OperatorKind::OpenParenthesis;
