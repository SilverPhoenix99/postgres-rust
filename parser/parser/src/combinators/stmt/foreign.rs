#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) enum Foreign {
    DataWrapper(Str),
    Table(QualifiedName),
}

pub(super) fn foreign(stream: &mut TokenStream) -> scan::Result<Foreign> {
    /*
        FOREIGN any_name
    */

    let (_, foreign) = seq!(
        Kw::Foreign,
        alt!(
            seq!(Data, Wrapper, col_id)
                .map(|(.., name)| Foreign::DataWrapper(name)),
            seq!(Kw::Table, any_name)
                .map(|(_, name)| Foreign::Table(name))
        )
    ).parse(stream)?;

    Ok(foreign)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

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

use pg_basics::QualifiedName;
use pg_basics::Str;
use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Data;
use pg_lexer::Keyword::Wrapper;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
use pg_sink_combinators::any_name;
use pg_sink_combinators::col_id;
