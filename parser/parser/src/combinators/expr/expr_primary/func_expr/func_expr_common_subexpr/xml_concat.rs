pub(super) fn xml_concat(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        XMLCONCAT '(' expr_list ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, args) = seq!(skip(1), paren!(expr_list))
        .parse(stream)?;

    Ok(XmlConcat(args))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::StringConst;
    use test_case::test_case;

    #[test_case("xmlconcat('foo', 'bar')" => Ok(
        XmlConcat(vec![
            StringConst("foo".into()),
            StringConst("bar".into()),
        ])
    ))]
    fn test_xml_concat(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, xml_concat)
    }
}

use crate::combinators::expr_list::expr_list;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
use pg_ast::ExprNode::XmlConcat;
