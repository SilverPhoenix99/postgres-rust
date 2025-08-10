pub(super) fn xml_concat(ctx: &mut ParserContext) -> scan::Result<SqlFunction> {

    /*
        XMLCONCAT '(' expr_list ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, args) = seq!(skip(1), paren!(expr_list))
        .parse(ctx)?;

    Ok(XmlConcat(args))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::StringConst;
    use pg_ast::SqlFunction;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("xmlconcat('foo', 'bar')" => Ok(
        XmlConcat(vec![
            StringConst("foo".into()),
            StringConst("bar".into()),
        ])
    ))]
    fn test_xml_concat(source: &str) -> scan::Result<SqlFunction> {
        test_parser!(source, xml_concat)
    }
}

use crate::combinators::expr_list::expr_list;
use pg_ast::SqlFunction;
use pg_ast::SqlFunction::XmlConcat;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::skip;
use pg_combinators::Combinator;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
