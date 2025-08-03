#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) enum Operator {
    WithArgs(OperatorWithArgs),
    Class { name: QualifiedName, index_method: Str },
    Family { name: QualifiedName, index_method: Str },
}

pub(super) fn operator(stream: &mut TokenStream) -> scan::Result<Operator> {

    let (_, op) = seq!(
        Kw::Operator,
        alt!(
            seq!(Class, any_name, Using, col_id)
                .map(|(_, name, _, index_method)|
                    Operator::Class { name, index_method }
                ),
            seq!(Family, any_name, Using, col_id)
                .map(|(_, name, _, index_method)|
                    Operator::Family { name, index_method }
                ),
            operator_with_argtypes
                .map(Operator::WithArgs)
        )
    ).parse(stream)?;

    Ok(op)
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
        test_parser!(source, operator, expected)
    }
}

use crate::combinators::any_name;
use crate::combinators::col_id;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::stmt::operator_with_argtypes;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::OperatorWithArgs;
use pg_basics::QualifiedName;
use pg_basics::Str;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Class;
use pg_lexer::Keyword::Family;
use pg_lexer::Keyword::Using;
