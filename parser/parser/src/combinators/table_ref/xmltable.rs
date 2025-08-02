fn xml_namespace_el(stream: &mut TokenStream) -> scan::Result<NamedValue> {

    /*
          DEFAULT b_expr
        | b_expr AS ColLabel
    */

    or((
        (DefaultKw, b_expr)
            .map(|(_, value)| NamedValue::unnamed(value)),
        (b_expr, As, col_label)
            .map(|(value, _, name)| NamedValue::new(Some(name), value)),
    )).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::StringConst;

    #[test_case("default 'foo'" => Ok(
        NamedValue::unnamed(
            StringConst("foo".into())
        )
    ))]
    #[test_case("'foo' as bar" => Ok(
        NamedValue::new(
            Some("bar".into()),
            StringConst("foo".into())
        )
    ))]
    fn test_xml_namespace_el(source: &str) -> scan::Result<NamedValue> {
        test_parser!(source, xml_namespace_el)
    }
}

use crate::scan;
use crate::stream::TokenStream;
use pg_ast::NamedValue;
use pg_lexer::Keyword::{As, DefaultKw};
use crate::combinators::col_label::col_label;
use crate::combinators::expr::b_expr;
use crate::combinators::foundation::{or, Combinator};
