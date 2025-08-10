pub(super) fn xml_forest(ctx: &mut ParserContext) -> scan::Result<SqlFunction> {

    /*
        XMLFOREST '(' xml_attribute_list ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, content) = seq!(skip(1), paren!(xml_attribute_list))
        .parse(ctx)?;

    Ok(XmlForest(content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
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
use pg_ast::SqlFunction;
use pg_ast::SqlFunction::XmlForest;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::skip;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_parser_core::scan;
