pub(super) fn xml_element(stream: &mut TokenStream) -> scan::Result<XmlElement> {

    /*
        XMLELEMENT '('
            NAME
            col_label
            ( ',' xml_attributes )?
            ( ',' expr_list )?
        ')'
    */

    if ! matches!(stream.peek2(), Ok((K(Xmlelement), Op(OpenParenthesis)))) {
        return no_match(stream)
    }

    let (_, name, extra_args) = skip_prefix(1, between_paren((
        Name,
        col_label,
        xml_element_extra_args.optional()
    ))).parse(stream)?;

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

    let (_, (args, content)) = (
        Comma,
        or((
            (
                xml_attributes.map(Some),
                (Comma, expr_list)
                    .map(|(_, content)| content)
                    .optional()
            ),
            expr_list.map(|content| {
                (None, Some(content))
            })
        ))
    ).parse(stream)?;

    Ok((args, content))
}

fn xml_attributes(stream: &mut TokenStream) -> scan::Result<Vec<NamedValue>> {

    /*
        XMLATTRIBUTES '(' xml_attribute_list ')'
    */

    let (_, attrs) = (Xmlattributes, between_paren(xml_attribute_list))
        .parse(stream)?;

    Ok(attrs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::{IntegerConst, StringConst},
        scan::Error::NoMatch,
    };

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
    #[test_case("xmlelement" => matches Err(NoMatch(_)))]
    #[test_case("xmlelement 1" => matches Err(NoMatch(_)))]
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
use crate::combinators::foundation::between_paren;
use crate::combinators::foundation::or;
use crate::combinators::foundation::skip_prefix;
use crate::combinators::foundation::Combinator;
use crate::no_match;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword as K;
use crate::stream::TokenValue::Operator as Op;
use pg_ast::ExprNode;
use pg_ast::NamedValue;
use pg_ast::XmlElement;
use pg_lexer::Keyword::Name;
use pg_lexer::Keyword::Xmlattributes;
use pg_lexer::Keyword::Xmlelement;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::OpenParenthesis;
