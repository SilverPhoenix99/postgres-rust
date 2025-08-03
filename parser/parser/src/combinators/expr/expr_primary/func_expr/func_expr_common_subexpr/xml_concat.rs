pub(super) fn xml_concat(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        XMLCONCAT '(' expr_list ')'
    */

    if ! matches!(stream.peek2(), Ok((K(Xmlconcat), Op(OpenParenthesis)))) {
        return no_match(stream)
    }

    let (_, args) = seq!(skip(1), paren(expr_list))
        .parse(stream)?;

    Ok(XmlConcat(args))
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

    #[test_case("xmlconcat('foo', 'bar')" => Ok(
        XmlConcat(vec![
            StringConst("foo".into()),
            StringConst("bar".into()),
        ])
    ))]
    #[test_case("xmlconcat" => matches Err(NoMatch(_)))]
    #[test_case("xmlconcat 1" => matches Err(NoMatch(_)))]
    fn test_xml_concat(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, xml_concat)
    }
}

use crate::combinators::expr_list::expr_list;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use crate::combinators::foundation::Combinator;
use crate::no_match;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword as K;
use crate::stream::TokenValue::Operator as Op;
use pg_ast::ExprNode;
use pg_ast::ExprNode::XmlConcat;
use pg_lexer::Keyword::Xmlconcat;
use pg_lexer::OperatorKind::OpenParenthesis;
