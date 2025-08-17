pub(super) fn xml_serialize(ctx: &mut ParserContext) -> scan::Result<XmlSerialize> {

    /*
        XMLSERIALIZE '('
            document_or_content
            a_expr
            AS
            SimpleTypename
            ( xml_indent_option )?
        ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, (kind, content, _, type_name, indent)) = seq!(
        skip(1),
        paren!(seq!(
            document_or_content,
            a_expr,
            As,
            simple_typename,
            xml_indent_option.optional()
        ))
    ).parse(ctx)?;

    let indent = indent.unwrap_or_default();

    let expr = XmlSerialize::new(kind, content, type_name)
        .with_indent(indent);

    Ok(expr)
}

fn xml_indent_option(ctx: &mut ParserContext) -> scan::Result<bool> {

    /*
        ( NO )? INDENT
    */

    alt!(
        Indent.map(|_| true),
        seq!(No, Indent).map(|_| false),
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::StringConst,
        pg_ast::TypeName::{Int4, Int8},
        pg_xml_ast::XmlNodeKind,
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
    fn test_xml_serialize(source: &str) -> scan::Result<XmlSerialize> {
        test_parser!(source, xml_serialize)
    }

    #[test_case("indent" => Ok(true))]
    #[test_case("no indent" => Ok(false))]
    fn test_xml_indent_option(source: &str) -> scan::Result<bool> {
        test_parser!(source, xml_indent_option)
    }
}

use crate::combinators::expr::a_expr;
use pg_ast::XmlSerialize;
use pg_combinators::alt;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::skip;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::As;
use pg_lexer::Keyword::Indent;
use pg_lexer::Keyword::No;
use pg_parser_core::scan;
use pg_type_combinators::simple_typename;
use pg_xml_combinators::document_or_content;
