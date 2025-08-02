fn xmltable_column_option_el(stream: &mut TokenStream) -> scan::Result<XmltableColumnOption> {

    /*
          NULL
        | NOT NULL
        | DEFAULT b_expr
        | PATH b_expr
        | IDENT b_expr
    */

    or((
        Kw::Null
            .map(|_| Null),
        (Not, Kw::Null)
            .map(|_| NotNull),
        (DefaultKw, b_expr)
            .map(|(_, value)| DefaultOption(value)),
        (Kw::Path, b_expr)
            .map(|(_, value)| Path(value)),
        xmltable_column_ident_option,
    )).parse(stream)
}

fn xmltable_column_ident_option(stream: &mut TokenStream) -> scan::Result<XmltableColumnOption> {

    let ((option, loc), value) = (located(identifier), b_expr).parse(stream)?;

    if option.as_ref() == "__pg__is_not_null" {
        let err = InvalidXmlTableOptionName(option).at(loc);
        return Err(err.into())
    }

    Ok(Generic { option, value })
}

fn xml_namespace_list(stream: &mut TokenStream) -> scan::Result<Vec<NamedValue>> {

    /*
        xml_namespace_el ( ',' xml_namespace_el )*
    */

    many_sep(Comma, xml_namespace_el).parse(stream)
}

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
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::StringConst,
        pg_basics::Location,
    };

    #[test_case("null" => Ok(Null))]
    #[test_case("not null" => Ok(NotNull))]
    #[test_case("default 'foo'" => Ok(DefaultOption(StringConst("foo".into()))))]
    #[test_case("path 'foo'" => Ok(Path(StringConst("foo".into()))))]
    #[test_case("foo 'bar'" => Ok(Generic {
        option: "foo".into(),
        value: StringConst("bar".into())
    }))]
    #[test_case("__pg__is_not_null 'foo'" => Err(
        InvalidXmlTableOptionName("__pg__is_not_null".into())
            .at(Location::new(0..17, 1, 1))
            .into()
    ))]
    fn test_xmltable_column_option_el(source: &str) -> scan::Result<XmltableColumnOption> {
        test_parser!(source, xmltable_column_option_el)
    }

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

use crate::combinators::col_label::col_label;
use crate::combinators::expr::b_expr;
use crate::combinators::foundation::{identifier, located, many_sep, or, Combinator};
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::XmltableColumnOption::Default as DefaultOption;
use pg_ast::XmltableColumnOption::{Generic, NotNull, Null, Path};
use pg_ast::{NamedValue, XmltableColumnOption};
use pg_elog::parser::Error::InvalidXmlTableOptionName;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::{As, DefaultKw, Not};
use pg_lexer::OperatorKind::Comma;
