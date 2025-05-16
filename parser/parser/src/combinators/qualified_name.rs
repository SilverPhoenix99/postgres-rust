/// Post-condition: Vec is **Not** empty
pub(super) fn qualified_name_list() -> impl Combinator<Output = Vec<RelationName>> {

    /*
        qualified_name ( ',' qualified_name )*
    */

    many_sep(Comma, qualified_name())
}

pub(super) fn qualified_name() -> impl Combinator<Output = RelationName> {

    /*
        (col_id attrs){1,3}
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
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

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

use crate::combinators::any_name;
use crate::combinators::foundation::located;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use elog::parser::NameList;
use elog::parser::ParserError;
use elog::parser::ParserErrorKind::ImproperQualifiedName;
use postgres_parser_ast::RelationName;
use postgres_parser_ast::SchemaName;
use postgres_parser_lexer::OperatorKind::Comma;
use std::mem;
