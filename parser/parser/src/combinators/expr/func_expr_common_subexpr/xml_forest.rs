pub(super) fn xml_forest(ctx: &mut ParserContext) -> scan::Result<SqlFunction> {

    /*
        XMLFOREST '(' labeled_expr_list ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, content) = seq!(skip(1), paren!(labeled_expr_list))
        .parse(ctx)?;

    Ok(XmlForest(content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
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

use super::labeled_expr_list;
use crate::combinators::core::skip;
use crate::combinators::core::Combinator;
use crate::paren;
use crate::seq;
use crate::ParserContext;
use pg_ast::SqlFunction;
use pg_ast::SqlFunction::XmlForest;
use pg_parser_core::scan;
