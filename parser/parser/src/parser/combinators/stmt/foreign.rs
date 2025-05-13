#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) enum Foreign {
    DataWrapper(Str),
    Table(QualifiedName),
}

pub(super) fn foreign() -> impl Combinator<Output = Foreign> {
    /*
        FOREIGN any_name
    */

    Kw::Foreign.and_right(or(
        and(Data, Wrapper)
            .and_right(col_id())
            .map(Foreign::DataWrapper),
        Kw::Table
            .and_right(any_name())
            .map(Foreign::Table)
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;

    #[test]
    fn test_foreign_data_wrapper() {
        test_parser!(
            source = "foreign data wrapper foo",
            parser = foreign(),
            expected = Foreign::DataWrapper("foo".into())
        );
    }
    
    #[test]
    fn test_foreign_table() {
        test_parser!(
            source = "foreign table foo",
            parser = foreign(),
            expected = Foreign::Table(vec!["foo".into()])
        );
    }
}

use crate::parser::combinators::any_name;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::and;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
use postgres_parser_ast::QualifiedName;
use postgres_parser_lexer::Keyword as Kw;
use postgres_parser_lexer::Keyword::Data;
use postgres_parser_lexer::Keyword::Wrapper;
