pub(super) fn xml_attribute_list(stream: &mut TokenStream) -> scan::Result<Vec<NamedValue>> {

    /*
        xml_attribute ( ',' xml_attribute )*
    */

    many!(sep = Comma, xml_attribute).parse(stream)
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
    #[allow(unused_imports)]
    use pg_ast::ExprNode::IntegerConst;
    use pg_combinators::test_parser;
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

use crate::combinators::expr::a_expr;
use pg_ast::NamedValue;
use pg_combinators::many;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::As;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
use pg_sink_combinators::col_label;
