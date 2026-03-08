pub(super) fn xml_element(ctx: &mut ParserContext) -> scan::Result<XmlElement> {

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
    ).parse(ctx)?;

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

fn xml_element_extra_args(ctx: &mut ParserContext) -> scan::Result<ExtraArgs>
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
    ).parse(ctx)?;

    Ok((args, content))
}

fn xml_attributes(ctx: &mut ParserContext) -> scan::Result<Vec<NamedValue>> {

    /*
        XMLATTRIBUTES '(' xml_attribute_list ')'
    */

    let (_, attrs) = seq!(Xmlattributes, paren!(xml_attribute_list))
        .parse(ctx)?;

    Ok(attrs)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::{IntegerConst, StringConst};
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
        let mut ctx = ParserContext::new(source);
        xml_element(&mut ctx)
    }

    #[test_case("xmlattributes(1, 2 as x)" => Ok(vec![
        NamedValue::unnamed(IntegerConst(1)),
        NamedValue::new(Some("x".into()), IntegerConst(2)),
    ]))]
    fn test_xml_attributes(source: &str) -> scan::Result<Vec<NamedValue>> {
        let mut ctx = ParserContext::new(source);
        xml_attributes(&mut ctx)
    }
}

use super::xml_attribute_list;
use crate::alt;
use crate::combinators::col_label;
use crate::combinators::core::skip;
use crate::combinators::core::Combinator;
use crate::combinators::expr_list;
use crate::paren;
use crate::seq;
use crate::ParserContext;
use pg_ast::ExprNode;
use pg_ast::NamedValue;
use pg_ast::XmlElement;
use pg_lexer::Keyword::Name;
use pg_lexer::Keyword::Xmlattributes;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
