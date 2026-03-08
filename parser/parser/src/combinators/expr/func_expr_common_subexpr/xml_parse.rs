pub(super) fn xml_parse(ctx: &mut ParserContext) -> scan::Result<XmlParse> {

    /*
        XMLPARSE '(' document_or_content a_expr ( xml_whitespace_option )? ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, (kind, content, whitespace)) = seq!(
        skip(1),
        paren!(seq!(
            document_or_content,
            a_expr,
            xml_whitespace_option.optional()
        ))
    ).parse(ctx)?;

    let expr = XmlParse::new(
        kind,
        content,
        whitespace.unwrap_or_default()
    );

    Ok(expr)
}

fn xml_whitespace_option(ctx: &mut ParserContext) -> scan::Result<XmlWhitespaceOption> {

    /*
        ( PRESERVE | STRIP ) WHITESPACE
    */

    let (option, _) = seq!(
        alt!(
            Kw::Preserve.map(|_| Preserve),
            Kw::Strip.map(|_| Strip),
        ),
        Whitespace
    ).parse(ctx)?;

    Ok(option)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::StringConst,
        pg_ast::XmlNodeKind,
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
    fn test_xml_parse(source: &str) -> scan::Result<XmlParse> {
        test_parser!(source, xml_parse)
    }

    #[test_case("preserve whitespace" => Ok(Preserve))]
    #[test_case("strip whitespace" => Ok(Strip))]
    fn test_xml_whitespace_option(source: &str) -> scan::Result<XmlWhitespaceOption> {
        test_parser!(source, xml_whitespace_option)
    }
}

use crate::alt;
use crate::combinators::core::skip;
use crate::combinators::core::Combinator;
use crate::combinators::document_or_content;
use crate::combinators::expr::a_expr;
use crate::paren;
use crate::seq;
use crate::ParserContext;
use pg_ast::XmlParse;
use pg_ast::XmlWhitespaceOption;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Whitespace;
use pg_parser_core::scan;
use XmlWhitespaceOption::Preserve;
use XmlWhitespaceOption::Strip;
