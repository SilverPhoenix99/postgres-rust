fn xmltable_column_list(stream: &mut TokenStream) -> scan::Result<Vec<XmltableColumn>> {

    /*
        xmltable_column_el ( ',' xmltable_column_el )*
    */

    many_sep(Comma, xmltable_column_el).parse(stream)
}

fn xmltable_column_el(stream: &mut TokenStream) -> scan::Result<XmltableColumn> {

    /*
          col_id FOR ORDINALITY
        | col_id Typename ( xmltable_column_option_el )*
    */

    let (column_name, kind) = (
        col_id,
        or((
            (For, Ordinality).map(|_| None),
            (typename, many(located(xmltable_column_option_el)).optional()).map(Some)
        ))
    ).parse(stream)?;

    let Some((type_name, options)) = kind else {
        return Ok(XmltableColumn::new(column_name, ForOrdinality))
    };

    let mut nullability_seen = false;
    let mut column_def = XmltableColumnDefinition::from(type_name);

    let options = options.into_iter().flatten();
    for (option, loc) in options {
        match option {
            Null => {
                if nullability_seen {
                    return Err(ConflictingNullability(column_name).at(loc).into())
                }
                column_def.set_not_null(false);
                nullability_seen = true;
            }
            NotNull => {
                if nullability_seen {
                    return Err(ConflictingNullability(column_name).at(loc).into())
                }
                column_def.set_not_null(true);
                nullability_seen = true;
            }
            DefaultOption(value) => {
                if column_def.default_value().is_some() {
                    return Err(DefaultValueAlreadyDeclared.at(loc).into())
                }
                column_def.set_default_value(Some(value));
            }
            Path(value) => {
                if column_def.path_spec().is_some() {
                    return Err(PathValueAlreadyDeclared.at(loc).into())
                }
                column_def.set_path_spec(Some(value));
            }
        }
    }

    Ok(XmltableColumn::new(column_name, column_def))
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum XmltableColumnOption {
    Null,
    NotNull,
    Default(ExprNode),
    Path(ExprNode),
}

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

    let ((option, loc), _) = (located(identifier), b_expr).parse(stream)?;

    let err = if option.as_ref() == "__pg__is_not_null" {
        InvalidXmlTableOptionName(option)
    }
    else {
        UnrecognizedColumnOption(option)
    };

    Err(err.at(loc).into())
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
        pg_ast::ExprNode::{IntegerConst, StringConst},
        pg_ast::TypeName::Int4,
        pg_basics::Location,
    };

    #[test_case("foo for ordinality" => Ok(
        XmltableColumn::new("foo", ForOrdinality)
    ))]
    #[test_case("bar int" => Ok(
        XmltableColumn::new(
            "bar",
            XmltableColumnDefinition::from(Int4)
        )
    ))]
    #[test_case("baz int not null default 1" => Ok(
        XmltableColumn::new(
            "baz",
            XmltableColumnDefinition::from(Int4)
                .with_not_null(true)
                .with_default_value(IntegerConst(1))
        )
    ))]
    #[test_case("qux int default 1 default 2" => Err(
        DefaultValueAlreadyDeclared
            .at(Location::new(18..25, 1, 19))
            .into()
    ))]
    #[test_case("lorem int path 'x' path 'y'" => Err(
        PathValueAlreadyDeclared
            .at(Location::new(19..23, 1, 20))
            .into()
    ))]
    #[test_case("yumyum int not null null" => Err(
        ConflictingNullability("yumyum".into())
            .at(Location::new(20..24, 1, 21))
            .into()
    ))]
    #[test_case("narslog int null not null" => Err(
        ConflictingNullability("narslog".into())
            .at(Location::new(17..20, 1, 18))
            .into()
    ))]
    #[test_case("umpus int null null" => Err(
        ConflictingNullability("umpus".into())
            .at(Location::new(15..19, 1, 16))
            .into()
    ))]
    #[test_case("wawas int not null not null" => Err(
        ConflictingNullability("wawas".into())
            .at(Location::new(19..22, 1, 20))
            .into()
    ))]
    fn test_xmltable_column_el(source: &str) -> scan::Result<XmltableColumn> {
        test_parser!(source, xmltable_column_el)
    }

    #[test_case("null" => Ok(Null))]
    #[test_case("not null" => Ok(NotNull))]
    #[test_case("default 'foo'" => Ok(DefaultOption(StringConst("foo".into()))))]
    #[test_case("path 'foo'" => Ok(Path(StringConst("foo".into()))))]
    #[test_case("foo 'bar'" => Err(
        UnrecognizedColumnOption("foo".into())
            .at(Location::new(0..3, 1, 1))
            .into()
    ))]
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

use crate::combinators::col_id::col_id;
use crate::combinators::col_label::col_label;
use crate::combinators::expr::b_expr;
use crate::combinators::foundation::{identifier, located, many, many_sep, or, Combinator};
use crate::combinators::typename::typename;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::XmltableColumnKind::ForOrdinality;
use pg_ast::{ExprNode, NamedValue, XmltableColumn, XmltableColumnDefinition};
use pg_elog::parser::Error::InvalidXmlTableOptionName;
use pg_elog::parser::Error::UnrecognizedColumnOption;
use pg_elog::parser::Error::{ConflictingNullability, DefaultValueAlreadyDeclared, PathValueAlreadyDeclared};
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::{As, DefaultKw, For, Not, Ordinality};
use pg_lexer::OperatorKind::Comma;
use XmltableColumnOption::Default as DefaultOption;
use XmltableColumnOption::{NotNull, Null, Path};
