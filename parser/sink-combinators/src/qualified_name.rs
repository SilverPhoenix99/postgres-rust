pub fn qualified_name_list(stream: &mut TokenStream) -> scan::Result<Vec<RelationName>> {

    /*
        qualified_name ( ',' qualified_name )*
    */

    many!(sep = Comma, qualified_name).parse(stream)
}

pub fn qualified_name(stream: &mut TokenStream) -> scan::Result<RelationName> {

    /*
        (col_id attrs){1,3}
    */

    let (mut qn, loc) = located!(any_name).parse(stream)?;

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
            let err = ImproperQualifiedName(NameList(qn)).at(loc);
            Err(err.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_qualified_name_list() {
        test_parser!(
            source = "relation_,schema_.relation_, catalog_.schema_.relation_",
            parser = qualified_name_list,
            expected = vec![
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
            ]
        )
    }

    #[test]
    fn test_qualified_name() {
        test_parser!(
            source = "some_catalog.some_schema.some_relation",
            parser = qualified_name,
            expected = RelationName::new(
                "some_relation",
                Some(SchemaName::new(
                    "some_schema",
                    Some("some_catalog".into())
                ))
            )
        )
    }
}

use crate::any_name;
use core::mem;
use pg_combinators::located;
use pg_combinators::many;
use pg_combinators::Combinator;
use pg_elog::parser::Error::ImproperQualifiedName;
use pg_elog::parser::NameList;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
use pg_sink_ast::RelationName;
use pg_sink_ast::SchemaName;
