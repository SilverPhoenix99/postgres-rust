pub(super) fn json_predicate_type_constraint(ctx: &mut ParserContext) -> scan::Result<JsonValueKind> {

    /*
          VALUE
        | ARRAY
        | OBJECT
        | SCALAR
    */

    alt!(
        Kw::Value.map(|_| Value),
        Kw::Array.map(|_| Array),
        Kw::Object.map(|_| Object),
        Kw::Scalar.map(|_| Scalar),
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_matrix;

    #[test_matrix("value" => Ok(Value))]
    #[test_matrix("array" => Ok(Array))]
    #[test_matrix("object" => Ok(Object))]
    #[test_matrix("scalar" => Ok(Scalar))]
    fn test_json_predicate_type_constraint(source: &str) -> scan::Result<JsonValueKind> {
        test_parser!(source, json_predicate_type_constraint)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::context::ParserContext;
use pg_ast::JsonValueKind;
use pg_ast::JsonValueKind::Array;
use pg_ast::JsonValueKind::Object;
use pg_ast::JsonValueKind::Scalar;
use pg_ast::JsonValueKind::Value;
use pg_lexer::Keyword as Kw;
use pg_parser_core::scan;
