/// Alias: `opt_for_locking_strength`
pub(super) fn for_locking_strength(ctx: &mut ParserContext) -> scan::Result<LockClauseStrength> {

    /*
          FOR KEY SHARE
        | FOR SHARE
        | FOR NO KEY UPDATE
        | FOR UPDATE
    */

    let (_, lock_strength) = seq!(
        For,
        alt!(
            seq!(Key, Share).map(|_| ForKeyShare),
            Share.map(|_| ForShare),
            seq!(No, Key, Update).map(|_| ForNoKeyUpdate),
            Update.map(|_| ForUpdate)
        )
    ).parse(ctx)?;

    Ok(lock_strength)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_matrix;

    #[test_matrix("for key share" => Ok(ForKeyShare))]
    #[test_matrix("for share" => Ok(ForShare))]
    #[test_matrix("for no key update" => Ok(ForNoKeyUpdate))]
    #[test_matrix("for update" => Ok(ForUpdate))]
    fn test_for_locking_strength(source: &str) -> scan::Result<LockClauseStrength> {
        test_parser!(source, for_locking_strength)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::LockClauseStrength;
use pg_ast::LockClauseStrength::ForKeyShare;
use pg_ast::LockClauseStrength::ForNoKeyUpdate;
use pg_ast::LockClauseStrength::ForShare;
use pg_ast::LockClauseStrength::ForUpdate;
use pg_lexer::Keyword::For;
use pg_lexer::Keyword::Key;
use pg_lexer::Keyword::No;
use pg_lexer::Keyword::Share;
use pg_lexer::Keyword::Update;
use pg_parser_core::scan;
