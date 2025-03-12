/// Post-condition: Vec is **Not** empty
pub(super) fn qualified_name_list() -> impl Combinator<Output = Vec<RangeVar>> {

    /*
        qualified_name ( ',' qualified_name )*
    */

    many_sep(Comma, qualified_name())
}

pub(super) fn qualified_name() -> impl Combinator<Output = RangeVar> {

    /*
        col_id attrs{1,3}
    */

    located(any_name::any_name())
        .map_result(|result| {
            let (mut qn, loc) = result?;

            match qn.as_mut_slice() {
                [relation] => {
                    let relation = mem::take(relation);
                    Ok(RangeVar::new(relation))
                },
                [schema, relation] => {
                    let schema = mem::take(schema);
                    let relation = mem::take(relation);
                    Ok(
                        RangeVar::new(relation)
                            .with_schema(schema)
                    )
                },
                [catalog, schema, relation] => {
                    let catalog = mem::take(catalog);
                    let schema = mem::take(schema);
                    let relation = mem::take(relation);
                    Ok(
                        RangeVar::new(relation)
                            .with_schema(schema)
                            .with_catalog(catalog)
                    )
                },
                _ => {
                    let err = ParserError::new(ImproperQualifiedName(NameList(qn)), loc);
                    Err(err.into())
                }
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_qualified_name_list() {
        let source = "relation_,schema_.relation_, catalog_.schema_.relation_";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = vec![
            RangeVar::new("relation_".into()),
            RangeVar::new("relation_".into())
                .with_schema("schema_".into()),
            RangeVar::new("relation_".into())
                .with_schema("schema_".into())
                .with_catalog("catalog_".into())
        ];

        assert_eq!(Ok(expected), qualified_name_list().parse(&mut stream));
    }

    #[test]
    fn test_qualified_name() {
        let source = "some_catalog.some_schema.some_relation";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = RangeVar::new("some_relation".into())
            .with_schema("some_schema".into())
            .with_catalog("some_catalog".into());

        assert_eq!(Ok(expected), qualified_name().parse(&mut stream));
    }
}

use crate::lexer::OperatorKind::Comma;
use crate::parser::ast_node::RangeVar;
use crate::parser::combinators::any_name;
use crate::parser::combinators::foundation::located;
use crate::parser::combinators::foundation::many_sep;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::error::NameList;
use crate::parser::ParserError;
use crate::parser::ParserErrorKind::ImproperQualifiedName;
use std::mem;
