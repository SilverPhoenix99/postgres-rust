#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) enum Foreign {
    DataWrapper(Str),
    Table(QualifiedName),
}

pub(super) fn foreign(stream: &mut TokenStream) -> Result<Foreign> {
    /*
        FOREIGN any_name
    */

    let (_, foreign) = seq!(=>
        Kw::Foreign.parse(stream),
        choice!(stream =>
            seq!(stream => Data, Wrapper, col_id)
                .map(|(.., name)|
                    Foreign::DataWrapper(name)
                ),
            seq!(stream => Kw::Table, any_name)
                .map(|(_, name)|
                    Foreign::Table(name)
                )
        )
    )?;

    Ok(foreign)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_foreign_data_wrapper() {
        test_parser!(
            source = "foreign data wrapper foo",
            parser = foreign,
            expected = Foreign::DataWrapper("foo".into())
        );
    }

    #[test]
    fn test_foreign_table() {
        test_parser!(
            source = "foreign table foo",
            parser = foreign,
            expected = Foreign::Table(vec!["foo".into()])
        );
    }
}

use crate::combinators::any_name;
use crate::combinators::col_id;
use crate::combinators::foundation::choice;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_basics::QualifiedName;
use pg_basics::Str;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Data;
use pg_lexer::Keyword::Wrapper;
