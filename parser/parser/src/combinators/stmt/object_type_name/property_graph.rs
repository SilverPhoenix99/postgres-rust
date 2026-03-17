pub(in crate::combinators::stmt) fn property_graph(ctx: &mut ParserContext) -> scan::Result<QualifiedName> {

    /*
        PROPERTY GRAPH any_name
    */

    let (_, _, name) = seq!(Property, Graph, any_name)
        .parse(ctx)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_property_graph() {
        test_parser!(
            source = "property graph foo",
            parser = property_graph,
            expected = vec!["foo".into()]
        );
    }
}

use crate::combinators::any_name::any_name;
use crate::combinators::core::Combinator;
use crate::context::ParserContext;
use crate::seq;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Graph;
use pg_lexer::Keyword::Property;
use pg_parser_core::scan;
