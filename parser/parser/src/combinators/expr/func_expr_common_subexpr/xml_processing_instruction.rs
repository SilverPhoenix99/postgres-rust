pub(super) fn xml_processing_instruction(stream: &mut TokenStream) -> scan::Result<XmlProcessingInstruction> {

    /*
        XMLPI '(' NAME col_label ( ',' a_expr )? ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, (_, name, value)) = seq!(
        skip(1),
        paren!(seq!(
            Name,
            col_label,
            seq!(Comma, a_expr).optional()
        ))
    ).parse(stream)?;

    let value = value.map(|(_, val)| val);

    let mut expr = XmlProcessingInstruction::new(name);
    expr.set_value(value);

    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::StringConst;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("xmlpi(name foo)" => Ok(
        XmlProcessingInstruction::new("foo")
    ))]
    #[test_case("xmlpi(name bar, 'baz')" => Ok(
        XmlProcessingInstruction::new("bar")
            .with_value(StringConst("baz".into()))
    ))]
    fn test_xml_processing_instruction(source: &str) -> scan::Result<XmlProcessingInstruction> {
        test_parser!(source, xml_processing_instruction)
    }
}

use crate::combinators::col_label;
use crate::combinators::expr::a_expr;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::skip;
use pg_ast::XmlProcessingInstruction;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Name;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
