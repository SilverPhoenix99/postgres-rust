/// Alias: `CreatedbStmt`
pub(super) fn create_database_stmt(stream: &mut TokenStream) -> scan::Result<CreateDatabaseStmt> {

    let (_, name, _, options) = (
        Database,
        col_id,
        With.optional(),
        createdb_opt_list
    ).parse(stream)?;

    let stmt = CreateDatabaseStmt::new(name, options);
    Ok(stmt)
}

fn createdb_opt_list(stream: &mut TokenStream) -> scan::Result<Vec<CreatedbOption>> {

    many(createdb_opt_item).parse(stream)
}

fn createdb_opt_item(stream: &mut TokenStream) -> scan::Result<CreatedbOption> {

    /*
          createdb_opt_name ( '=' )? DEFAULT
        | createdb_opt_name ( '=' )? var_value
    */

    let (kind, _, value) = (
        createdb_opt_name,
        Equals.optional(),
        createdb_opt_value
    ).parse(stream)?;

    let option = CreatedbOption::new(kind, value);
    Ok(option)
}

fn createdb_opt_name(stream: &mut TokenStream) -> scan::Result<CreatedbOptionKind> {
    // Broken down into smaller combinators, due to large Rust type names.
    or((
        createdb_opt_name_1,
        createdb_opt_name_2,
    )).parse(stream)
}

fn createdb_opt_name_1(stream: &mut TokenStream) -> scan::Result<CreatedbOptionKind> {
    or((
        (Connection, Limit).map(|_| ConnectionLimit),
        Kw::Encoding.map(|_| Encoding),
        LocationKw.map(|_| Location),
        Kw::Owner.map(|_| Owner),
    )).parse(stream)
}

fn createdb_opt_name_2(stream: &mut TokenStream) -> scan::Result<CreatedbOptionKind> {
    or((
        Kw::Tablespace.map(|_| Tablespace),
        Kw::Template.map(|_| Template),
        // Unless quoted, identifiers are lower case
        identifier.map(|ident| match ident.as_ref() {
            "allow_connections" => AllowConnections,
            "builtin_locale" => BuiltinLocale,
            "collation_version" => CollationVersion,
            "icu_locale" => IcuLocale,
            "icu_rules" => IcuRules,
            "is_template" => IsTemplate,
            "lc_collate" => LcCollate,
            "lc_ctype" => LcCtype,
            "locale" => Locale,
            "locale_provider" => LocaleProvider,
            "oid" => Oid,
            "strategy" => Strategy,
            _ => Unknown(ident)
        })
    )).parse(stream)
}

pub(in crate::combinators::stmt) fn createdb_opt_value(stream: &mut TokenStream) -> scan::Result<CreatedbOptionValue> {

    /*
          DEFAULT
        | var_value
    */

    or((
        DefaultKw.map(|_| CreatedbOptionValue::Default),
        var_value.map(From::from)
    )).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test]
    fn test_create_database_stmt() {
        test_parser!(
            source = "database db_name with connection limit = 753 allow_connections 'on'",
            parser = create_database_stmt,
            expected = CreateDatabaseStmt::new(
                "db_name",
                vec![
                    CreatedbOption::new(ConnectionLimit, 753),
                    CreatedbOption::new(AllowConnections, "on"),
                ]
            )
        )
    }

    #[test]
    fn test_createdb_opt_list() {
        test_parser!(
            source = "connection limit = 753 allow_connections 'on'",
            parser = createdb_opt_list,
            expected = vec![
                CreatedbOption::new(ConnectionLimit, 753),
                CreatedbOption::new(AllowConnections, "on"),
            ]
        )
    }

    #[test_case("allow_connections DEFAULT", CreatedbOption::new(AllowConnections, CreatedbOptionValue::Default))]
    #[test_case("oid = 54321", CreatedbOption::new(Oid, 54321))]
    fn test_createdb_opt_item(source: &str, expected: CreatedbOption) {
        test_parser!(source, createdb_opt_item, expected)
    }

    #[test_case("allow_connections", AllowConnections)]
    #[test_case("builtin_locale", BuiltinLocale)]
    #[test_case("collation_version", CollationVersion)]
    #[test_case("icu_locale", IcuLocale)]
    #[test_case("icu_rules", IcuRules)]
    #[test_case("is_template", IsTemplate)]
    #[test_case("lc_collate", LcCollate)]
    #[test_case("lc_ctype", LcCtype)]
    #[test_case("locale", Locale)]
    #[test_case("locale_provider", LocaleProvider)]
    #[test_case("oid", Oid)]
    #[test_case("strategy", Strategy)]
    #[test_case("connection limit", ConnectionLimit)]
    #[test_case("encoding", Encoding)]
    #[test_case("location", Location)]
    #[test_case("owner", Owner)]
    #[test_case("tablespace", Tablespace)]
    #[test_case("template", Template)]
    #[test_case("foo", Unknown("foo".into()))]
    fn test_createdb_opt_name(source: &str, expected: CreatedbOptionKind) {
        test_parser!(source, createdb_opt_name, expected)
    }

    #[test_case("default", CreatedbOptionValue::Default)]
    #[test_case("true", true.into())]
    #[test_case("false", false.into())]
    #[test_case("on", "on".into())]
    #[test_case("off", "off".into())]
    #[test_case("'value'", "value".into())]
    #[test_case("+123", 123.into())]
    fn test_createdb_opt_value(source: &str, expected: CreatedbOptionValue) {
        test_parser!(source, createdb_opt_value, expected)
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::identifier;
use crate::combinators::foundation::many;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::var_value;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::CreateDatabaseStmt;
use pg_ast::CreatedbOption;
use pg_ast::CreatedbOptionKind;
use pg_ast::CreatedbOptionKind::AllowConnections;
use pg_ast::CreatedbOptionKind::BuiltinLocale;
use pg_ast::CreatedbOptionKind::CollationVersion;
use pg_ast::CreatedbOptionKind::ConnectionLimit;
use pg_ast::CreatedbOptionKind::Encoding;
use pg_ast::CreatedbOptionKind::IcuLocale;
use pg_ast::CreatedbOptionKind::IcuRules;
use pg_ast::CreatedbOptionKind::IsTemplate;
use pg_ast::CreatedbOptionKind::LcCollate;
use pg_ast::CreatedbOptionKind::LcCtype;
use pg_ast::CreatedbOptionKind::Locale;
use pg_ast::CreatedbOptionKind::LocaleProvider;
use pg_ast::CreatedbOptionKind::Location;
use pg_ast::CreatedbOptionKind::Oid;
use pg_ast::CreatedbOptionKind::Owner;
use pg_ast::CreatedbOptionKind::Strategy;
use pg_ast::CreatedbOptionKind::Tablespace;
use pg_ast::CreatedbOptionKind::Template;
use pg_ast::CreatedbOptionKind::Unknown;
use pg_ast::CreatedbOptionValue;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Connection;
use pg_lexer::Keyword::Database;
use pg_lexer::Keyword::DefaultKw;
use pg_lexer::Keyword::Limit;
use pg_lexer::Keyword::LocationKw;
use pg_lexer::Keyword::With;
use pg_lexer::OperatorKind::Equals;
