/// Alias: `CreatedbStmt`
pub(super) fn create_database_stmt() -> impl Combinator<Output = CreateDatabaseStmt> {

    sequence!(
        Database.skip(),
        col_id(),
        With.optional().skip(),
        createdb_opt_list()
    ).map(|(_, name, _, options)|
        CreateDatabaseStmt::new(name, options)
    )
}

fn createdb_opt_list() -> impl Combinator<Output = Vec<CreatedbOption>> {

    many(createdb_opt_item())
}

fn createdb_opt_item() -> impl Combinator<Output = CreatedbOption> {

    /*
          createdb_opt_name ( '=' )? DEFAULT
        | createdb_opt_name ( '=' )? var_value
    */

    sequence!(
        createdb_opt_name(),
        Equals.optional().skip(),
        createdb_opt_value()
    ).map(|(kind, _, value)|
        CreatedbOption::new(kind, value)
    )
}

fn createdb_opt_name() -> impl Combinator<Output = CreatedbOptionKind> {

    match_first! {
        Connection.and(Limit).map(|_| ConnectionLimit),
        Kw::Encoding.map(|_| Encoding),
        LocationKw.map(|_| Location),
        Kw::Owner.map(|_| Owner),
        Kw::Tablespace.map(|_| Tablespace),
        Kw::Template.map(|_| Template),
        // Unless quoted, identifiers are lower case
        identifier().map(|ident| match ident.as_ref() {
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
    }
}

pub(in crate::combinators::stmt) fn createdb_opt_value() -> impl Combinator<Output = CreatedbOptionValue> {
    use CreatedbOptionValue::*;

    /*
          DEFAULT
        | var_value
    */

    match_first! {
        DefaultKw.map(|_| Default),
        var_value().map(From::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test]
    fn test_create_database_stmt() {
        let source = "database db_name with connection limit = 753 allow_connections 'on'";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = create_database_stmt().parse(&mut stream);

        let expected = CreateDatabaseStmt::new(
            "db_name",
            vec![
                CreatedbOption::new(ConnectionLimit, 753),
                CreatedbOption::new(AllowConnections, "on"),
            ]
        );

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_createdb_opt_list() {
        let source = "connection limit = 753 allow_connections 'on'";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = createdb_opt_list().parse(&mut stream);

        let expected = vec![
            CreatedbOption::new(ConnectionLimit, 753),
            CreatedbOption::new(AllowConnections, "on"),
        ];

        assert_eq!(Ok(expected), actual);
    }

    #[test_case("allow_connections DEFAULT", CreatedbOption::new(AllowConnections, CreatedbOptionValue::Default))]
    #[test_case("oid = 54321", CreatedbOption::new(Oid, 54321))]
    fn test_createdb_opt_item(source: &str, expected: CreatedbOption) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = createdb_opt_item().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
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
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = createdb_opt_name().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }

    #[test_case("default", CreatedbOptionValue::Default)]
    #[test_case("true", true.into())]
    #[test_case("false", false.into())]
    #[test_case("on", "on".into())]
    #[test_case("off", "off".into())]
    #[test_case("'value'", "value".into())]
    #[test_case("+123", 123.into())]
    fn test_createdb_opt_value(source: &str, expected: CreatedbOptionValue) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = createdb_opt_value().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::identifier;
use crate::combinators::foundation::many;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::sequence;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::var_value;
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
