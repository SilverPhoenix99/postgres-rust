pub(super) fn xml_parse(stream: &mut TokenStream) -> scan::Result<XmlParse> {

    /*
        XMLPARSE '(' document_or_content a_expr ( xml_whitespace_option )? ')'
    */

    if ! matches!(stream.peek2(), Ok((K(Xmlparse), Op(OpenParenthesis)))) {
        return no_match(stream)
    }

    let (kind, content, whitespace) = skip_prefix(1, paren((
        document_or_content,
        a_expr,
        xml_whitespace_option.optional()
    ))).parse(stream)?;

    let expr = XmlParse::new(
        kind,
        content,
        whitespace.unwrap_or_default()
    );

    Ok(expr)
}

fn xml_whitespace_option(stream: &mut TokenStream) -> scan::Result<XmlWhitespaceOption> {

    /*
        ( PRESERVE | STRIP ) WHITESPACE
    */

    let (option, _) = (
        or((
            Kw::Preserve.map(|_| Preserve),
            Kw::Strip.map(|_| Strip),
        )),
        Whitespace
    ).parse(stream)?;

    Ok(option)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::StringConst,
        pg_ast::XmlNodeKind,
        scan::Error::NoMatch,
    };

    #[test_case("xmlparse(content 'foo')" => Ok(
        XmlParse::new(
            XmlNodeKind::Content,
            StringConst("foo".into()),
            Strip
        )
    ))]
    #[test_case("xmlparse(document 'foo' preserve whitespace)" => Ok(
        XmlParse::new(
            XmlNodeKind::Document,
            StringConst("foo".into()),
            Preserve
        )
    ))]
    #[test_case("xmlparse" => matches Err(NoMatch(_)))]
    #[test_case("xmlparse 1" => matches Err(NoMatch(_)))]
    fn test_xml_parse(source: &str) -> scan::Result<XmlParse> {
        test_parser!(source, xml_parse)
    }

    #[test_case("preserve whitespace" => Ok(Preserve))]
    #[test_case("strip whitespace" => Ok(Strip))]
    fn test_xml_whitespace_option(source: &str) -> scan::Result<XmlWhitespaceOption> {
        test_parser!(source, xml_whitespace_option)
    }
}

use crate::combinators::document_or_content;
use crate::combinators::expr::a_expr;
use crate::combinators::foundation::or;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::skip_prefix;
use crate::combinators::foundation::Combinator;
use crate::no_match;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword as K;
use crate::stream::TokenValue::Operator as Op;
use pg_ast::XmlParse;
use pg_ast::XmlWhitespaceOption;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Whitespace;
use pg_lexer::Keyword::Xmlparse;
use pg_lexer::OperatorKind::OpenParenthesis;
use XmlWhitespaceOption::Preserve;
use XmlWhitespaceOption::Strip;
