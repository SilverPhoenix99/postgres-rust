pub(super) fn xml_processing_instruction(stream: &mut TokenStream) -> scan::Result<XmlProcessingInstruction> {

    /*
        XMLPI '(' NAME col_label ( ',' a_expr )? ')'
    */

    if ! matches!(stream.peek2(), Ok((K(Xmlpi), Op(OpenParenthesis)))) {
        return no_match(stream)
    }

    let (_, name, value) = skip_prefix(1, between_paren((
        Name,
        col_label,
        (Comma, a_expr).optional()
    ))).parse(stream)?;

    let value = value.map(|(_, val)| val);

    let mut expr = XmlProcessingInstruction::new(name);
    expr.set_value(value);

    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::StringConst,
        scan::Error::NoMatch,
    };

    #[test_case("xmlpi(name foo)" => Ok(
        XmlProcessingInstruction::new("foo")
    ))]
    #[test_case("xmlpi(name bar, 'baz')" => Ok(
        XmlProcessingInstruction::new("bar")
            .with_value(StringConst("baz".into()))
    ))]
    #[test_case("xmlpi" => matches Err(NoMatch(_)))]
    #[test_case("xmlpi 1" => matches Err(NoMatch(_)))]
    fn test_xml_processing_instruction(source: &str) -> scan::Result<XmlProcessingInstruction> {
        test_parser!(source, xml_processing_instruction)
    }
}

use crate::combinators::col_label::col_label;
use crate::combinators::expr::a_expr;
use crate::combinators::foundation::between_paren;
use crate::combinators::foundation::skip_prefix;
use crate::combinators::foundation::Combinator;
use crate::no_match;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword as K;
use crate::stream::TokenValue::Operator as Op;
use pg_ast::XmlProcessingInstruction;
use pg_lexer::Keyword::Name;
use pg_lexer::Keyword::Xmlpi;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::OpenParenthesis;
