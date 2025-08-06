pub(super) fn xml_forest(stream: &mut TokenStream) -> scan::Result<SqlFunction> {

    /*
        XMLFOREST '(' xml_attribute_list ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, content) = seq!(skip(1), paren!(xml_attribute_list))
        .parse(stream)?;

    Ok(XmlForest(content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::StringConst,
        pg_ast::NamedValue,
    };

    #[test_case("xmlforest('foo', 'bar' as baz)" => Ok(
        XmlForest(vec![
            NamedValue::unnamed(StringConst("foo".into())),
            NamedValue::new(Some("baz".into()), StringConst("bar".into())),
        ])
    ))]
    fn test_xml_forest(source: &str) -> scan::Result<SqlFunction> {
        test_parser!(source, xml_forest)
    }
}

use super::xml_attribute_list;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use crate::combinators::foundation::Combinator;
use pg_ast::SqlFunction;
use pg_ast::SqlFunction::XmlForest;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
