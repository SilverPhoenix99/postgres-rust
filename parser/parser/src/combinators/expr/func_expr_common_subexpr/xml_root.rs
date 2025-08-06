pub(super) fn xml_root(stream: &mut TokenStream) -> scan::Result<XmlRoot> {

    /*
        XMLROOT '('
            a_expr
            ','
            xml_root_version
            ( ',' xml_root_standalone )?
        ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, (content, _, version, standalone)) = seq!(
        skip(1),
        paren!(seq!(
            a_expr,
            Comma,
            xml_root_version,
            seq!(Comma, xml_root_standalone).optional()
        ))
    ).parse(stream)?;

    let standalone = standalone.map(|(_, standalone)| standalone);

    let mut expr = XmlRoot::new(content, version);
    expr.set_standalone(standalone);

    Ok(expr)
}

fn xml_root_version(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        VERSION (
            | NO VALUE // => ExprNode::NullConst
            | a_expr
        )
    */

    let (_, version) = seq!(
        Version,
        alt!(
            version_no_value,
            a_expr
        )
    ).parse(stream)?;

    Ok(version)
}

fn version_no_value(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        NO VALUE
    */

    if ! matches!(stream.peek2(), Ok((K(Kw::No), K(Value)))) {
        return no_match(stream)
    }

    stream.skip(2);
    Ok(ExprNode::NullConst)
}

/// Alias: `opt_xml_root_standalone`
fn xml_root_standalone(stream: &mut TokenStream) -> scan::Result<XmlStandalone> {

    /*
        STANDALONE (
              YES
            | NO ( VALUE )?
        )
    */

    let (_, standalone) = seq!(
        Standalone,
        alt!(
            Kw::Yes.map(|_| Yes),
            seq!(Kw::No, Value.optional())
                .map(|(_, val)|
                    if val.is_some() { NoValue } else { No }
                )
        )
    ).parse(stream)?;

    Ok(standalone)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::{NullConst, StringConst};
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("xmlroot('foo', version '1.0', standalone yes)" => Ok(
        XmlRoot::new(
            StringConst("foo".into()),
            StringConst("1.0".into())
        )
            .with_standalone(XmlStandalone::Yes)
    ))]
    #[test_case("xmlroot('foo', version no value)" => Ok(
        XmlRoot::new(
            StringConst("foo".into()),
            NullConst
        )
    ))]
    fn test_xml_root(source: &str) -> scan::Result<XmlRoot> {
        test_parser!(source, xml_root)
    }

    #[test_case("version '1.0'" => Ok(StringConst("1.0".into())))]
    #[test_case("version no value" => Ok(NullConst))]
    fn test_xml_root_version(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, xml_root_version)
    }

    #[test_case("standalone yes" => Ok(XmlStandalone::Yes))]
    #[test_case("standalone no" => Ok(XmlStandalone::No))]
    #[test_case("standalone no value" => Ok(XmlStandalone::NoValue))]
    fn test_xml_root_standalone(source: &str) -> scan::Result<XmlStandalone> {
        test_parser!(source, xml_root_standalone)
    }
}

use crate::combinators::expr::a_expr;
use crate::no_match;
use pg_ast::ExprNode;
use pg_ast::XmlRoot;
use pg_ast::XmlStandalone;
use pg_ast::XmlStandalone::No;
use pg_ast::XmlStandalone::NoValue;
use pg_ast::XmlStandalone::Yes;
use pg_combinators::alt;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::skip;
use pg_combinators::Combinator;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Standalone;
use pg_lexer::Keyword::Value;
use pg_lexer::Keyword::Version;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
use pg_parser_core::stream::TokenValue::Keyword as K;
