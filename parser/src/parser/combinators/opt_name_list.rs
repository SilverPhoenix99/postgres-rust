/// Post-condition: Vec is **Not** empty
///
/// Alias: `opt_column_list`
pub(super) fn opt_name_list() -> impl Combinator<Output = Vec<Str>> {

    /*
        '(' name_list ')'
    */

    between(OpenParenthesis, name_list::name_list(), CloseParenthesis)
}

use crate::lexer::OperatorKind::CloseParenthesis;
use crate::lexer::OperatorKind::OpenParenthesis;
use crate::parser::combinators::foundation::{between, Combinator};
use crate::parser::combinators::name_list;
use postgres_basics::Str;
