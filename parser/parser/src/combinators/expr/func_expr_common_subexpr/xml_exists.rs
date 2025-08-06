pub(super) fn xml_exists(stream: &mut TokenStream) -> scan::Result<XmlExists> {

    /*
        XMLEXISTS '(' c_expr xmlexists_argument ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, (path_spec, content)) = seq!(skip(1), paren!(seq!(
        expr_primary,
        xmlexists_argument
    ))).parse(stream)?;

    let expr = XmlExists::new(path_spec, content);
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::StringConst;
    use test_case::test_case;

    #[test_case("xmlexists('foo' passing 'bar')" => Ok(
        XmlExists::new(
            StringConst("foo".into()),
            StringConst("bar".into())
        )
    ))]
    fn test_xml_exists(source: &str) -> scan::Result<XmlExists> {
        test_parser!(source, xml_exists)
    }
}

use crate::combinators::expr::expr_primary;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use crate::combinators::foundation::Combinator;
use crate::combinators::xmlexists_argument;
use pg_ast::XmlExists;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
