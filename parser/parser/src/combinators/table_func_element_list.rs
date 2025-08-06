/// Aliases:
/// * `OptTableFuncElementList`
/// * `TableFuncElementList`
pub(super) fn table_func_element_list(stream: &mut TokenStream) -> scan::Result<Vec<SimpleColumnDefinition>> {

    /*
        table_func_element ( ',' table_func_element )*
    */

    many!(sep = Comma, table_func_element).parse(stream)
}

/// Alias: `TableFuncElement ( ',' TableFuncElement )*`
fn table_func_element(stream: &mut TokenStream) -> scan::Result<SimpleColumnDefinition> {

    /*
        col_id typename ( collate_clause )?
    */

    let (name, type_name, collation) = seq!(
        col_id,
        simple_typename,
        collate_clause.optional()
    ).parse(stream)?;

    let mut col = SimpleColumnDefinition::new(name, type_name);
    col.set_collation(collation);

    Ok(col)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::SimpleColumnDefinition;
    #[allow(unused_imports)]
    use pg_ast::TypeName;
    use test_case::test_case;

    #[test_case("json json" => Ok(
        SimpleColumnDefinition::new("json", TypeName::Json)
    ))]
    #[test_case("int int collate foo" => Ok(
        SimpleColumnDefinition::new("int", TypeName::Int4)
            .with_collation(vec!["foo".into()])
    ))]
    fn test_table_func_element(source: &str) -> scan::Result<SimpleColumnDefinition> {
        test_parser!(source, table_func_element)
    }
}

use crate::combinators::col_id;
use crate::combinators::collate_clause;
use crate::combinators::foundation::many;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::simple_typename;
use crate::stream::TokenStream;
use pg_ast::SimpleColumnDefinition;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
