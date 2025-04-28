#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) enum Operator {
    WithArgs(OperatorWithArgs),
    Class { name: QualifiedName, index_method: Str },
    Family { name: QualifiedName, index_method: Str },
}

pub(super) fn operator() -> impl Combinator<Output = Operator> {

    Kw::Operator.and_right(match_first! {
        and(
            Class.and_right(any_name()),
            Using.and_right(col_id())
        ).map(|(name, index_method)| Operator::Class { name, index_method }),
        and(
            Family.and_right(any_name()),
            Using.and_right(col_id())
        ).map(|(name, index_method)| Operator::Family { name, index_method }),
        operator_with_argtypes().map(Operator::WithArgs)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use crate::parser::ast_node::{
        OneOrBoth,
        Operator::Addition,
        QualifiedOperator,
        TypeName::Int4,
    };
    use crate::parser::tests::test_parser;
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

use crate::lexer::Keyword as Kw;
use crate::lexer::Keyword::Class;
use crate::lexer::Keyword::Family;
use crate::lexer::Keyword::Using;
use crate::parser::ast_node::OperatorWithArgs;
use crate::parser::ast_node::QualifiedName;
use crate::parser::combinators::any_name;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::and;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::stmt::operator_with_argtypes;
use postgres_basics::Str;
