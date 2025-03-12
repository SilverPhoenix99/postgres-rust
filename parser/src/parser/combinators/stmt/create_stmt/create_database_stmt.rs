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

pub(in crate::parser::combinators::stmt) fn createdb_opt_value() -> impl Combinator<Output = CreatedbOptionValue> {
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
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test]
    fn test_create_database_stmt() {
        let source = "database db_name with connection limit = 753 allow_connections 'on'";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = create_database_stmt().parse(&mut stream);

        let expected = CreateDatabaseStmt::new(
            "db_name".into(),
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

use crate::lexer::Keyword as Kw;
use crate::lexer::Keyword::Connection;
use crate::lexer::Keyword::Database;
use crate::lexer::Keyword::DefaultKw;
use crate::lexer::Keyword::Limit;
use crate::lexer::Keyword::LocationKw;
use crate::lexer::Keyword::With;
use crate::lexer::OperatorKind::Equals;
use crate::parser::ast_node::CreateDatabaseStmt;
use crate::parser::ast_node::CreatedbOption;
use crate::parser::ast_node::CreatedbOptionKind;
use crate::parser::ast_node::CreatedbOptionKind::AllowConnections;
use crate::parser::ast_node::CreatedbOptionKind::BuiltinLocale;
use crate::parser::ast_node::CreatedbOptionKind::CollationVersion;
use crate::parser::ast_node::CreatedbOptionKind::ConnectionLimit;
use crate::parser::ast_node::CreatedbOptionKind::Encoding;
use crate::parser::ast_node::CreatedbOptionKind::IcuLocale;
use crate::parser::ast_node::CreatedbOptionKind::IcuRules;
use crate::parser::ast_node::CreatedbOptionKind::IsTemplate;
use crate::parser::ast_node::CreatedbOptionKind::LcCollate;
use crate::parser::ast_node::CreatedbOptionKind::LcCtype;
use crate::parser::ast_node::CreatedbOptionKind::Locale;
use crate::parser::ast_node::CreatedbOptionKind::LocaleProvider;
use crate::parser::ast_node::CreatedbOptionKind::Location;
use crate::parser::ast_node::CreatedbOptionKind::Oid;
use crate::parser::ast_node::CreatedbOptionKind::Owner;
use crate::parser::ast_node::CreatedbOptionKind::Strategy;
use crate::parser::ast_node::CreatedbOptionKind::Tablespace;
use crate::parser::ast_node::CreatedbOptionKind::Template;
use crate::parser::ast_node::CreatedbOptionKind::Unknown;
use crate::parser::ast_node::CreatedbOptionValue;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::identifier;
use crate::parser::combinators::foundation::many;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::var_value;
