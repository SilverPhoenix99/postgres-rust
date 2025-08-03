pub(super) fn xml_attribute_list(stream: &mut TokenStream) -> scan::Result<Vec<NamedValue>> {

    /*
        xml_attribute ( ',' xml_attribute )*
    */

    many_sep(Comma, xml_attribute).parse(stream)
}

/// Alias: `xml_attribute_el`
fn xml_attribute(stream: &mut TokenStream) -> scan::Result<NamedValue> {

    /*
        a_expr ( AS col_label )?
    */

    let (value, name) = seq!(
        a_expr,
        seq!(As, col_label).optional()
    ).parse(stream)?;

    let name = name.map(|(_, name)| name);
    let value = NamedValue::new(name, value);
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::IntegerConst;
    use test_case::test_case;

    #[test_case("1" => Ok(
        NamedValue::unnamed(IntegerConst(1))
    ))]
    #[test_case("2 as x" => Ok(
        NamedValue::new(Some("x".into()), IntegerConst(2))
    ))]
    fn test_xml_attribute(source: &str) -> scan::Result<NamedValue> {
        test_parser!(source, xml_attribute)
    }
}

use crate::combinators::col_label;
use crate::combinators::expr::a_expr;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::NamedValue;
use pg_lexer::Keyword::As;
use pg_lexer::OperatorKind::Comma;
