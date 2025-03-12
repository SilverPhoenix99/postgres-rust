/// Post-condition: Vec is **Not** empty
pub(super) fn var_name() -> impl Combinator<Output = QualifiedName> {

    /*
        col_id ( '.' col_id )*
    */

    many_sep(Dot, col_id::col_id())
}

use crate::lexer::OperatorKind::Dot;
use crate::parser::ast_node::QualifiedName;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::many_sep;
use crate::parser::combinators::foundation::Combinator;
