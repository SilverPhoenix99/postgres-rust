#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) enum Foreign {
    DataWrapper(Str),
    Table(QualifiedName),
}

pub(super) fn foreign() -> impl Combinator<Output = Foreign> {
    /*
        FOREIGN any_name
    */

    Kw::Foreign.and_right(match_first!(
        (Data, Wrapper, col_id)
            .map(|(.., name)|
                Foreign::DataWrapper(name)
            ),
        Kw::Table
            .and_then(any_name, |_, name|
                Foreign::Table(name)
            )
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

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

use crate::combinators::any_name;
use crate::combinators::col_id;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::Combinator;
use pg_basics::QualifiedName;
use pg_basics::Str;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Data;
use pg_lexer::Keyword::Wrapper;
