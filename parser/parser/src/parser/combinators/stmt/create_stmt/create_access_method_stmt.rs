/// Alias: `CreateAmStmt`
pub(super) fn create_access_method_stmt() -> impl Combinator<Output = CreateAccessMethodStmt> {

    /*
        ACCESS METHOD ColId TYPE_P am_type HANDLER any_name
    */

    sequence!(
        and(Access, Method).and_right(col_id()),
        Type.and_right(am_type()),
        Handler.and_right(any_name())
    )
        .map(|(name, kind, handler)|
            CreateAccessMethodStmt::new(name, kind, handler)
        )
}

fn am_type() -> impl Combinator<Output = AccessMethodKind> {

    or(
        Kw::Index.map(|_| Index),
        Kw::Table.map(|_| Table)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;
    use test_case::test_case;

    #[test]
    fn test_create_access_method_stmt() {
        test_parser!(
            source = "access method foo type index handler bar",
            parser = create_access_method_stmt(),
            expected = CreateAccessMethodStmt::new("foo", Index, vec!["bar".into()])
        )
    }

    #[test_case("index", Index)]
    #[test_case("table", Table)]
    fn test_am_type(source: &str, expected: AccessMethodKind) {
        test_parser!(source, am_type(), expected);
    }
}

use crate::parser::ast_node::AccessMethodKind;
use crate::parser::ast_node::AccessMethodKind::Index;
use crate::parser::ast_node::AccessMethodKind::Table;
use crate::parser::ast_node::CreateAccessMethodStmt;
use crate::parser::combinators::any_name;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::and;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_parser_lexer::Keyword as Kw;
use postgres_parser_lexer::Keyword::Access;
use postgres_parser_lexer::Keyword::Handler;
use postgres_parser_lexer::Keyword::Method;
use postgres_parser_lexer::Keyword::Type;
