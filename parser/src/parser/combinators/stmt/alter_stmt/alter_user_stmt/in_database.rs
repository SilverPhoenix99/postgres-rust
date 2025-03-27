/// Alias: `opt_in_database`
pub(super) fn in_database() -> impl Combinator<Output = Str> {

    /*
        IN DATABASE col_id
    */

    sequence!(In, Database)
        .and_right(col_id())
        .map(|dbname| dbname)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_in_database() {
        let source = "in database db_name";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = in_database().parse(&mut stream);
        assert_eq!(Ok("db_name".into()), actual);
    }
}

use crate::lexer::Keyword::Database;
use crate::lexer::Keyword::In;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
