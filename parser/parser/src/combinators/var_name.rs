/// Post-condition: Vec is **Not** empty
pub(super) fn var_name() -> impl Combinator<Output = QualifiedName> {

    /*
        col_id ( '.' col_id )*
    */

    many_sep(Dot, col_id())
}

use crate::combinators::col_id;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::Combinator;
use postgres_basics::QualifiedName;
use postgres_parser_lexer::OperatorKind::Dot;
