/// Alias: `CreatedbStmt`
pub(in crate::combinators::stmt) fn create_database_stmt(ctx: &mut ParserContext) -> scan::Result<DatabaseStmt> {

    /*
        CREATE DATABASE ColId ( WITH )? createdb_opt_list
    */

    let (_, name, _, options) = seq!(
        Database,
        col_id,
        With.optional(),
        createdb_opt_list
    ).parse(ctx)?;

    let options = DatabaseStmtOption::Create(options);
    let stmt = DatabaseStmt::new(name, options);
    Ok(stmt)
}

fn createdb_opt_list(ctx: &mut ParserContext) -> scan::Result<Vec<CreatedbOption>> {

    /*
        ( createdb_opt_item )+
    */

    many!(createdb_opt_item).parse(ctx)
}

fn createdb_opt_item(ctx: &mut ParserContext) -> scan::Result<CreatedbOption> {

    /*
          createdb_opt_name ( '=' )? DEFAULT
        | createdb_opt_name ( '=' )? var_value
    */

    let (kind, _, value) = seq!(
        createdb_opt_name,
        Equals.optional(),
        createdb_opt_value
    ).parse(ctx)?;

    let option = CreatedbOption::new(kind, value);
    Ok(option)
}

fn createdb_opt_name(ctx: &mut ParserContext) -> scan::Result<CreatedbOptionKind> {
    alt!(
        seq!(Connection, Limit).map(|_| ConnectionLimit),
        Kw::Encoding.map(|_| Encoding),
        LocationKw.map(|_| Location),
        Kw::Owner.map(|_| Owner),
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
    ).parse(ctx)
}

pub(super) fn createdb_opt_value(ctx: &mut ParserContext) -> scan::Result<CreatedbOptionValue> {

    /*
          DEFAULT
        | var_value
    */

    alt!(
        DefaultKw.map(|_| CreatedbOptionValue::Default),
        var_value.map(From::from)
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_matrix;

    #[test]
    fn test_create_database_stmt() {
        test_parser!(
            source = "database db_name with connection limit = 753 allow_connections 'on'",
            parser = create_database_stmt,
            expected = DatabaseStmt::new(
                "db_name",
                DatabaseStmtOption::Create(
                    vec![
                        CreatedbOption::new(ConnectionLimit, 753),
                        CreatedbOption::new(AllowConnections, "on"),
                    ]
                )
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

    #[test_matrix("allow_connections DEFAULT" => Ok(CreatedbOption::new(AllowConnections, CreatedbOptionValue::Default)))]
    #[test_matrix("oid = 54321" => Ok(CreatedbOption::new(Oid, 54321)))]
    fn test_createdb_opt_item(source: &str) -> scan::Result<CreatedbOption> {
        test_parser!(source, createdb_opt_item)
    }

    #[test_matrix("allow_connections" => Ok(AllowConnections))]
    #[test_matrix("builtin_locale" => Ok(BuiltinLocale))]
    #[test_matrix("collation_version" => Ok(CollationVersion))]
    #[test_matrix("icu_locale" => Ok(IcuLocale))]
    #[test_matrix("icu_rules" => Ok(IcuRules))]
    #[test_matrix("is_template" => Ok(IsTemplate))]
    #[test_matrix("lc_collate" => Ok(LcCollate))]
    #[test_matrix("lc_ctype" => Ok(LcCtype))]
    #[test_matrix("locale" => Ok(Locale))]
    #[test_matrix("locale_provider" => Ok(LocaleProvider))]
    #[test_matrix("oid" => Ok(Oid))]
    #[test_matrix("strategy" => Ok(Strategy))]
    #[test_matrix("connection limit" => Ok(ConnectionLimit))]
    #[test_matrix("encoding" => Ok(Encoding))]
    #[test_matrix("location" => Ok(Location))]
    #[test_matrix("owner" => Ok(Owner))]
    #[test_matrix("tablespace" => Ok(Tablespace))]
    #[test_matrix("template" => Ok(Template))]
    #[test_matrix("foo" => Ok(Unknown("foo".into())))]
    fn test_createdb_opt_name(source: &str) -> scan::Result<CreatedbOptionKind> {
        test_parser!(source, createdb_opt_name)
    }

    #[test_matrix("default" => Ok(CreatedbOptionValue::Default))]
    #[test_matrix("true" => Ok(true.into()))]
    #[test_matrix("false" => Ok(false.into()))]
    #[test_matrix("on" => Ok("on".into()))]
    #[test_matrix("off" => Ok("off".into()))]
    #[test_matrix("'value'" => Ok("value".into()))]
    #[test_matrix("+123" => Ok(123.into()))]
    fn test_createdb_opt_value(source: &str) -> scan::Result<CreatedbOptionValue> {
        test_parser!(source, createdb_opt_value)
    }
}

use crate::alt;
use crate::combinators::col_id;
use crate::combinators::core::identifier;
use crate::combinators::core::Combinator;
use crate::combinators::var_value;
use crate::many;
use crate::seq;
use crate::ParserContext;
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
use pg_ast::DatabaseStmt;
use pg_ast::DatabaseStmtOption;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Connection;
use pg_lexer::Keyword::Database;
use pg_lexer::Keyword::DefaultKw;
use pg_lexer::Keyword::Limit;
use pg_lexer::Keyword::LocationKw;
use pg_lexer::Keyword::With;
use pg_lexer::OperatorKind::Equals;
use pg_parser_core::scan;
