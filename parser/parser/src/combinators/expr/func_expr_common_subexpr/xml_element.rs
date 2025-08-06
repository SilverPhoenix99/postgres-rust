pub(super) fn xml_element(stream: &mut TokenStream) -> scan::Result<XmlElement> {

    /*
        XMLELEMENT '('
            NAME
            col_label
            ( ',' xml_attributes )?
            ( ',' expr_list )?
        ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, (_, name, extra_args)) = seq!(
        skip(1),
        paren!(seq!(
            Name,
            col_label,
            xml_element_extra_args.optional()
        ))
    ).parse(stream)?;

    let (attrs, content) = extra_args.unwrap_or_default();

    let mut el = XmlElement::new(name);
    el.set_attributes(attrs)
        .set_content(content);

    Ok(el)
}

type ExtraArgs = (
    Option<Vec<NamedValue>>,
    Option<Vec<ExprNode>>
);

fn xml_element_extra_args(stream: &mut TokenStream) -> scan::Result<ExtraArgs>
{
    /*
        ',' (
              xml_attributes ( ',' expr_list )?
            | expr_list
        )
    */

    let (_, (args, content)) = seq!(
        Comma,
        alt!(
            seq!(
                xml_attributes.map(Some),
                seq!(Comma, expr_list)
                    .map(|(_, content)| content)
                    .optional()
            ),
            expr_list.map(|content| {
                (None, Some(content))
            })
        )
    ).parse(stream)?;

    Ok((args, content))
}

fn xml_attributes(stream: &mut TokenStream) -> scan::Result<Vec<NamedValue>> {

    /*
        XMLATTRIBUTES '(' xml_attribute_list ')'
    */

    let (_, attrs) = seq!(Xmlattributes, paren!(xml_attribute_list))
        .parse(stream)?;

    Ok(attrs)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::{IntegerConst, StringConst};
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("xmlelement(name foo)" => Ok(
        XmlElement::new("foo".into())
    ))]
    #[test_case("xmlelement(name foo, xmlattributes(1 as a), 'foo')" => Ok(
        XmlElement::new("foo".into())
            .with_attributes(vec![
                NamedValue::new(Some("a".into()), IntegerConst(1))
            ])
            .with_content(vec![
                StringConst("foo".into())
            ])
    ))]
    #[test_case("xmlelement(name foo, 'bar')" => Ok(
        XmlElement::new("foo".into())
            .with_content(vec![
                StringConst("bar".into())
            ])
    ))]
    fn test_xml_element(source: &str) -> scan::Result<XmlElement> {
        test_parser!(source, xml_element)
    }

    #[test_case("xmlattributes(1, 2 as x)" => Ok(vec![
        NamedValue::unnamed(IntegerConst(1)),
        NamedValue::new(Some("x".into()), IntegerConst(2)),
    ]))]
    fn test_xml_attributes(source: &str) -> scan::Result<Vec<NamedValue>> {
        test_parser!(source, xml_attributes)
    }
}

use super::xml_attribute_list;
use crate::combinators::col_label;
use crate::combinators::expr_list;
use pg_ast::ExprNode;
use pg_ast::NamedValue;
use pg_ast::XmlElement;
use pg_combinators::alt;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::skip;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Name;
use pg_lexer::Keyword::Xmlattributes;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
