pub(super) fn xml_serialize(stream: &mut TokenStream) -> scan::Result<XmlSerialize> {

    /*
        XMLSERIALIZE '('
            document_or_content
            a_expr
            AS
            SimpleTypename
            ( xml_indent_option )?
        ')'
    */

    if ! matches!(stream.peek2(), Ok((K(Xmlserialize), Op(OpenParenthesis)))) {
        return no_match(stream)
    }

    let (_, (kind, content, _, type_name, indent)) = seq!(
        skip(1),
        paren(seq!(
            document_or_content,
            a_expr,
            As,
            simple_typename,
            xml_indent_option.optional()
        ))
    ).parse(stream)?;

    let indent = indent.unwrap_or_default();

    let expr = XmlSerialize::new(kind, content, type_name)
        .with_indent(indent);

    Ok(expr)
}

fn xml_indent_option(stream: &mut TokenStream) -> scan::Result<bool> {

    /*
        ( NO )? INDENT
    */

    alt!(
        Indent.map(|_| true),
        seq!(No, Indent).map(|_| false),
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::StringConst,
        pg_ast::TypeName::{Int4, Int8},
        pg_ast::XmlNodeKind,
        scan::Error::NoMatch,
    };

    #[test_case("xmlserialize(content 'foo' as int)" => Ok(
        XmlSerialize::new(
            XmlNodeKind::Content,
            StringConst("foo".into()),
            Int4
        )
    ))]
    #[test_case("xmlserialize(document 'bar' as bigint indent)" => Ok(
        XmlSerialize::new(
            XmlNodeKind::Document,
            StringConst("bar".into()),
            Int8
        )
        .with_indent(true)
    ))]
    #[test_case("xmlserialize" => matches Err(NoMatch(_)))]
    #[test_case("xmlserialize 1" => matches Err(NoMatch(_)))]
    fn test_xml_serialize(source: &str) -> scan::Result<XmlSerialize> {
        test_parser!(source, xml_serialize)
    }

    #[test_case("indent" => Ok(true))]
    #[test_case("no indent" => Ok(false))]
    fn test_xml_indent_option(source: &str) -> scan::Result<bool> {
        test_parser!(source, xml_indent_option)
    }
}

use crate::combinators::document_or_content;
use crate::combinators::expr::a_expr;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use crate::combinators::foundation::Combinator;
use crate::combinators::simple_typename;
use crate::no_match;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword as K;
use crate::stream::TokenValue::Operator as Op;
use pg_ast::XmlSerialize;
use pg_lexer::Keyword::As;
use pg_lexer::Keyword::Indent;
use pg_lexer::Keyword::No;
use pg_lexer::Keyword::Xmlserialize;
use pg_lexer::OperatorKind::OpenParenthesis;
