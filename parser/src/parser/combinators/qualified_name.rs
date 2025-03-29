/// Post-condition: Vec is **Not** empty
pub(super) fn qualified_name_list() -> impl Combinator<Output = Vec<RelationName>> {

    /*
        qualified_name ( ',' qualified_name )*
    */

    many_sep(Comma, qualified_name())
}

pub(super) fn qualified_name() -> impl Combinator<Output = RelationName> {

    /*
        col_id attrs{1,3}
    */

    located(any_name())
        .map_result(|result| {
            let (mut qn, loc) = result?;

            match qn.as_mut_slice() {
                [relation] => {
                    let relation = mem::take(relation);
                    Ok(RelationName::new(relation, None))
                },
                [schema, relation] => {
                    let schema = mem::take(schema);
                    let relation = mem::take(relation);
                    Ok(RelationName::new(
                        relation,
                        Some(SchemaName::new(schema, None))
                    ))
                },
                [catalog, schema, relation] => {
                    let catalog = mem::take(catalog);
                    let schema = mem::take(schema);
                    let relation = mem::take(relation);
                    Ok(RelationName::new(
                        relation,
                        Some(SchemaName::new(
                            schema,
                            Some(catalog)
                        ))
                    ))
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
    use crate::parser::ast_node::SchemaName;
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_qualified_name_list() {
        let source = "relation_,schema_.relation_, catalog_.schema_.relation_";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = vec![
            RelationName::new("relation_", None),
            RelationName::new(
                "relation_",
                Some(SchemaName::new("schema_", None))
            ),
            RelationName::new(
                "relation_",
                Some(SchemaName::new(
                    "schema_",
                    Some("catalog_".into())
                ))
            )
        ];

        assert_eq!(Ok(expected), qualified_name_list().parse(&mut stream));
    }

    #[test]
    fn test_qualified_name() {
        let source = "some_catalog.some_schema.some_relation";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = RelationName::new(
            "some_relation",
            Some(SchemaName::new(
                "some_schema",
                Some("some_catalog".into())
            ))
        );

        assert_eq!(Ok(expected), qualified_name().parse(&mut stream));
    }
}

use crate::lexer::OperatorKind::Comma;
use crate::parser::ast_node::RelationName;
use crate::parser::ast_node::SchemaName;
use crate::parser::combinators::any_name;
use crate::parser::combinators::foundation::located;
use crate::parser::combinators::foundation::many_sep;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::error::NameList;
use crate::parser::ParserError;
use crate::parser::ParserErrorKind::ImproperQualifiedName;
use std::mem;
