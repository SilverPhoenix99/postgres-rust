/// Alias: `opt_in_database`
pub(super) fn in_database() -> impl Combinator<Output = Str> {

    /*
        IN DATABASE col_id
    */

    (In, Database)
        .and_right(col_id)
        .map(|dbname| dbname)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_in_database() {
        let source = "in database db_name";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = in_database().parse(&mut stream);
        assert_eq!(Ok("db_name".into()), actual);
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::Combinator;
use pg_basics::Str;
use pg_lexer::Keyword::Database;
use pg_lexer::Keyword::In;
