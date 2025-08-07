/// Alias: `CreateAmStmt`
pub(super) fn create_access_method_stmt(stream: &mut TokenStream) -> scan::Result<CreateAccessMethodStmt> {

    /*
        ACCESS METHOD ColId TYPE_P am_type HANDLER any_name
    */

    let (_, _, name, _, kind, _, handler) = seq!(Access, Method, col_id, Type, am_type, Handler, any_name)
        .parse(stream)?;

    let stmt = CreateAccessMethodStmt::new(name, kind, handler);
    Ok(stmt)
}

fn am_type(stream: &mut TokenStream) -> scan::Result<AccessMethodKind> {

    alt!(
        Kw::Index.map(|_| Index),
        Kw::Table.map(|_| Table)
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test]
    fn test_create_access_method_stmt() {
        test_parser!(
            source = "access method foo type index handler bar",
            parser = create_access_method_stmt,
            expected = CreateAccessMethodStmt::new("foo", Index, vec!["bar".into()])
        )
    }

    #[test_case("index", Index)]
    #[test_case("table", Table)]
    fn test_am_type(source: &str, expected: AccessMethodKind) {
        test_parser!(source, am_type, expected);
    }
}

use pg_ast::AccessMethodKind;
use pg_ast::AccessMethodKind::Index;
use pg_ast::AccessMethodKind::Table;
use pg_ast::CreateAccessMethodStmt;
use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Access;
use pg_lexer::Keyword::Handler;
use pg_lexer::Keyword::Method;
use pg_lexer::Keyword::Type;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
use pg_sink_combinators::any_name;
use pg_sink_combinators::col_id;
