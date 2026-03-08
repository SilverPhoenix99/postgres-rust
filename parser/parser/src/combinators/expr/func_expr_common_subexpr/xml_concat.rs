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
    use test_case::test_case;

    #[test_case("xmlconcat('foo', 'bar')" => Ok(
        XmlConcat(vec![
            StringConst("foo".into()),
            StringConst("bar".into()),
        ])
    ))]
    fn test_xml_concat(source: &str) -> scan::Result<SqlFunction> {
        let mut ctx = ParserContext::new(source);
        xml_concat(&mut ctx)
    }
}

use crate::combinators::core::skip;
use crate::combinators::core::Combinator;
use crate::combinators::expr_list;
use crate::paren;
use crate::seq;
use crate::ParserContext;
use pg_ast::SqlFunction;
use pg_ast::SqlFunction::XmlConcat;
use pg_parser_core::scan;
