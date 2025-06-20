pub(super) fn any_name_list() -> impl Combinator<Output = Vec<QualifiedName>> {
    parser(v2::any_name_list)
}

pub(super) fn any_name() -> impl Combinator<Output = QualifiedName> {
    parser(v2::any_name)
}

use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::parser;
use crate::combinators::v2;
use pg_basics::QualifiedName;
