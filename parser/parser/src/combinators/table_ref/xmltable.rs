pub(super) fn xmltable(ctx: &mut ParserContext) -> scan::Result<XmlTable> {

    /*
        XMLTABLE '('
            ( XMLNAMESPACES '(' xml_namespace_list ')' ',' )?
            c_expr
            xmlexists_argument
            COLUMNS
            xmltable_column_list
        ')'
    */

    let (_, (namespaces, row_spec, doc, _, columns)) = seq!(
        Xmltable,
        paren!(seq!(
            xml_namespaces.optional(),
            expr_primary,
            xmlexists_argument,
            Columns,
            xmltable_column_list
        ))
    ).parse(ctx)?;

    let mut xml_table = XmlTable::new(doc, row_spec, columns);
    xml_table.set_namespaces(namespaces);

    Ok(xml_table)
}

fn xml_namespaces(ctx: &mut ParserContext) -> scan::Result<Vec<NamedValue>> {

    /*
        XMLNAMESPACES '(' xml_namespace_list ')' ','
    */

    let (_, namespaces, _) = seq!(Xmlnamespaces, paren!(xml_namespace_list), Comma)
        .parse(ctx)?;

    Ok(namespaces)
}

fn xmltable_column_list(ctx: &mut ParserContext) -> scan::Result<Vec<XmlTableColumn>> {

    /*
        xmltable_column_el ( ',' xmltable_column_el )*
    */

    many!(sep = Comma, xmltable_column_el).parse(ctx)
}

fn xmltable_column_el(ctx: &mut ParserContext) -> scan::Result<XmlTableColumn> {

    /*
          col_id FOR ORDINALITY
        | col_id Typename ( xmltable_column_option_el )*
    */

    let (column_name, kind) = seq!(
        col_id,
        alt!(
            seq!(For, Ordinality).map(|_| None),
            seq!(typename,
                many!(
                    located!(xmltable_column_option_el)
                ).optional()
            ).map(Some)
        )
    ).parse(ctx)?;

    let Some((type_name, options)) = kind else {
        return Ok(XmlTableColumn::new(column_name, ForOrdinality))
    };

    let mut nullability_seen = false;
    let mut column_def = XmlTableColumnDefinition::from(type_name);

    let options = options.into_iter().flatten();
    for Located(option, loc) in options {
        match option {
            Null => {
                if nullability_seen {
                    return Err(ConflictingNullability(column_name).at_location(loc).into())
                }
                column_def.set_not_null(false);
                nullability_seen = true;
            }
            NotNull => {
                if nullability_seen {
                    return Err(ConflictingNullability(column_name).at_location(loc).into())
                }
                column_def.set_not_null(true);
                nullability_seen = true;
            }
            DefaultOption(value) => {
                if column_def.default_value().is_some() {
                    return Err(DefaultValueAlreadyDeclared.at_location(loc).into())
                }
                column_def.set_default_value(Some(value));
            }
            Path(value) => {
                if column_def.path_spec().is_some() {
                    return Err(PathValueAlreadyDeclared.at_location(loc).into())
                }
                column_def.set_path_spec(Some(value));
            }
        }
    }

    Ok(XmlTableColumn::new(column_name, column_def))
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum XmlTableColumnOption {
    Null,
    NotNull,
    Default(ExprNode),
    Path(ExprNode),
}

fn xmltable_column_option_el(ctx: &mut ParserContext) -> scan::Result<XmlTableColumnOption> {

    /*
          NULL
        | NOT NULL
        | DEFAULT b_expr
        | PATH b_expr
        | IDENT b_expr
    */

    alt!(
        Kw::Null
            .map(|_| Null),
        seq!(Not, Kw::Null)
            .map(|_| NotNull),
        seq!(DefaultKw, b_expr)
            .map(|(_, value)| DefaultOption(value)),
        seq!(Kw::Path, b_expr)
            .map(|(_, value)| Path(value)),
        xmltable_column_ident_option,
    ).parse(ctx)
}

fn xmltable_column_ident_option(ctx: &mut ParserContext) -> scan::Result<XmlTableColumnOption> {

    let (Located(option, loc), _) = seq!(located!(identifier), b_expr).parse(ctx)?;

    let err = if option.as_ref() == "__pg__is_not_null" {
        InvalidXmlTableOptionName(option)
    }
    else {
        UnrecognizedColumnOption(option)
    };

    Err(err.at_location(loc).into())
}

fn xml_namespace_list(ctx: &mut ParserContext) -> scan::Result<Vec<NamedValue>> {

    /*
        xml_namespace_el ( ',' xml_namespace_el )*
    */

    many!(sep = Comma, xml_namespace_el).parse(ctx)
}

fn xml_namespace_el(ctx: &mut ParserContext) -> scan::Result<NamedValue> {

    /*
          DEFAULT b_expr
        | b_expr AS ColLabel
    */

    alt!(
        seq!(DefaultKw, b_expr)
            .map(|(_, value)| NamedValue::unnamed(value)),
        seq!(b_expr, As, col_label)
            .map(|(value, _, name)| NamedValue::new(Some(name), value)),
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::{IntegerConst, StringConst},
        pg_ast::TypeName::Int4,
        pg_elog::Error::Parser,
        scan::Error::ScanErr,
    };

    #[test_case(
        "xmltable(\
            'path' \
            passing by ref 'doc' \
            columns \
                a for ordinality, \
                b int\
        )"
        => Ok(
            XmlTable::new(
                StringConst("doc".into()),
                StringConst("path".into()),
                vec![
                    XmlTableColumn::new("a", ForOrdinality),
                    XmlTableColumn::new("b",
                        XmlTableColumnDefinition::from(Int4)
                    ),
                ]
            )
        )
    )]
    #[test_case(
        "xmltable(\
            xmlnamespaces(default 'foo', 'bar' as x), \
            'path' \
            passing 'doc' by value \
            columns \
                a int, \
                b for ordinality\
        )"
        => Ok(
            XmlTable::new(
                StringConst("doc".into()),
                StringConst("path".into()),
                vec![
                    XmlTableColumn::new("a",
                        XmlTableColumnDefinition::from(Int4)
                    ),
                    XmlTableColumn::new("b", ForOrdinality),
                ]
            )
            .with_namespaces(vec![
                NamedValue::unnamed(StringConst("foo".into())),
                NamedValue::new(Some("x".into()), StringConst("bar".into())),
            ])
        )
    )]
    fn test_xmltable(source: &str) -> scan::Result<XmlTable> {
        test_parser!(source, xmltable)
    }

    #[test_case("foo for ordinality" => Ok(
        XmlTableColumn::new("foo", ForOrdinality)
    ))]
    #[test_case("bar int" => Ok(
        XmlTableColumn::new(
            "bar",
            XmlTableColumnDefinition::from(Int4)
        )
    ))]
    #[test_case("baz int not null default 1" => Ok(
        XmlTableColumn::new(
            "baz",
            XmlTableColumnDefinition::from(Int4)
                .with_not_null(true)
                .with_default_value(IntegerConst(1))
        )
    ))]
    #[test_case("qux int default 1 default 2" => matches Err(ScanErr(
        Located(Parser(DefaultValueAlreadyDeclared), _)
    )))]
    #[test_case("lorem int path 'x' path 'y'" => matches Err(ScanErr(
        Located(Parser(PathValueAlreadyDeclared), _)
    )))]
    #[test_case("yumyum int not null null" => matches Err(ScanErr(
        Located(Parser(ConflictingNullability(_)), _)
    )))]
    #[test_case("narslog int null not null" => matches Err(ScanErr(
        Located(Parser(ConflictingNullability(_)), _)
    )))]
    #[test_case("umpus int null null" => matches Err(ScanErr(
        Located(Parser(ConflictingNullability(_)), _)
    )))]
    #[test_case("wawas int not null not null" => matches Err(ScanErr(
        Located(Parser(ConflictingNullability(_)), _)
    )))]
    fn test_xmltable_column_el(source: &str) -> scan::Result<XmlTableColumn> {
        test_parser!(source, xmltable_column_el)
    }

    #[test_case("null" => Ok(Null))]
    #[test_case("not null" => Ok(NotNull))]
    #[test_case("default 'foo'" => Ok(DefaultOption(StringConst("foo".into()))))]
    #[test_case("path 'foo'" => Ok(Path(StringConst("foo".into()))))]
    #[test_case("foo 'bar'" => matches Err(ScanErr(
        Located(Parser(UnrecognizedColumnOption(_)), _)
    )))]
    #[test_case("__pg__is_not_null 'foo'" => matches Err(ScanErr(
        Located(Parser(InvalidXmlTableOptionName(_)), _)
    )))]
    fn test_xmltable_column_option_el(source: &str) -> scan::Result<XmlTableColumnOption> {
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

use crate::combinators::expr::b_expr;
use crate::combinators::expr::expr_primary;
use crate::combinators::typename;
use crate::combinators::xmlexists_argument;
use pg_ast::ExprNode;
use pg_ast::NamedValue;
use pg_ast::XmlTable;
use pg_ast::XmlTableColumn;
use pg_ast::XmlTableColumnDefinition;
use pg_ast::XmlTableColumnKind::ForOrdinality;
use pg_basics::IntoLocated;
use pg_basics::Located;
use pg_combinators::alt;
use pg_combinators::identifier;
use pg_combinators::located;
use pg_combinators::many;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_elog::parser::Error::ConflictingNullability;
use pg_elog::parser::Error::DefaultValueAlreadyDeclared;
use pg_elog::parser::Error::InvalidXmlTableOptionName;
use pg_elog::parser::Error::PathValueAlreadyDeclared;
use pg_elog::parser::Error::UnrecognizedColumnOption;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::As;
use pg_lexer::Keyword::Columns;
use pg_lexer::Keyword::DefaultKw;
use pg_lexer::Keyword::For;
use pg_lexer::Keyword::Not;
use pg_lexer::Keyword::Ordinality;
use pg_lexer::Keyword::Xmlnamespaces;
use pg_lexer::Keyword::Xmltable;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_sink_combinators::col_id;
use pg_sink_combinators::col_label;
use XmlTableColumnOption::Default as DefaultOption;
use XmlTableColumnOption::NotNull;
use XmlTableColumnOption::Null;
use XmlTableColumnOption::Path;
