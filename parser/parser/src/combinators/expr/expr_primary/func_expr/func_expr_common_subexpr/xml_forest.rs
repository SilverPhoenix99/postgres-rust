pub(super) fn xml_forest(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        XMLFOREST '(' xml_attribute_list ')'
    */

    if !matches!(stream.peek2(), Ok((K(Xmlforest), Op(OpenParenthesis)))) {
        return no_match(stream)
    }

    let (_, content) = seq!(skip(1), paren!(xml_attribute_list))
        .parse(stream)?;

    Ok(XmlForest(content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::StringConst,
        pg_ast::NamedValue,
        scan::Error::NoMatch,
    };

    #[test_case("xmlforest('foo', 'bar' as baz)" => Ok(
        XmlForest(vec![
            NamedValue::unnamed(StringConst("foo".into())),
            NamedValue::new(Some("baz".into()), StringConst("bar".into())),
        ])
    ))]
    #[test_case("xmlforest" => matches Err(NoMatch(_)))]
    #[test_case("xmlforest 1" => matches Err(NoMatch(_)))]
    fn test_xml_forest(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, xml_forest)
    }
}

use super::xml_attribute_list;
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
use pg_ast::ExprNode::XmlForest;
use pg_lexer::Keyword::Xmlforest;
use pg_lexer::OperatorKind::OpenParenthesis;
