#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) enum Operator {
    WithArgs(OperatorWithArgs),
    Class { name: QualifiedName, index_method: Str },
    Family { name: QualifiedName, index_method: Str },
}

pub(super) fn operator() -> impl Combinator<Output = Operator> {

    Kw::Operator.and_right(match_first! {
        and(
            Class.and_right(any_name),
            Using.and_right(col_id)
        ).map(|(name, index_method)| Operator::Class { name, index_method }),
        and(
            Family.and_right(any_name),
            Using.and_right(col_id)
        ).map(|(name, index_method)| Operator::Family { name, index_method }),
        operator_with_argtypes().map(Operator::WithArgs)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{
        OneOrBoth,
        Operator::Addition,
        QualifiedOperator,
        TypeName::Int4,
    };
    use test_case::test_case;

    #[test_case("operator +(int, int)", Operator::WithArgs(
        OperatorWithArgs::new(
            QualifiedOperator(vec![], Addition),
            OneOrBoth::Both(Int4.into(), Int4.into())
        )
    ))]
    #[test_case("operator family some_family using some_method",
        Operator::Family {
            name: vec!["some_family".into()],
            index_method: "some_method".into()
            }
    )]
    #[test_case("operator class some_class using some_method",
        Operator::Class {
            name: vec!["some_class".into()],
            index_method: "some_method".into()
            }
    )]
    fn test_operator(source: &str, expected: Operator) {
        test_parser!(source, operator(), expected)
    }
}

use crate::combinators::any_name;
use crate::combinators::col_id;
use crate::combinators::foundation::and;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::Combinator;
use crate::combinators::stmt::operator_with_argtypes;
use pg_ast::OperatorWithArgs;
use pg_basics::QualifiedName;
use pg_basics::Str;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Class;
use pg_lexer::Keyword::Family;
use pg_lexer::Keyword::Using;
