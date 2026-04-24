/// Alias: `json_behavior_clause_opt`
pub(super) fn json_behavior_clause(ctx: &mut ParserContext) -> scan::Result<JsonBehaviorClause> {

    /*
          json_behavior ON ERROR
        | json_behavior ON EMPTY ( json_behavior ON ERROR )?
    */

    let (first, _, second) = seq!(
        json_behavior,
        On,
        alt!(
            ErrorKw.map(|_| None),
            seq!(Empty, json_on_error_clause.optional())
                .map(|(_, behavior)| Some(behavior))
        )
    ).parse(ctx)?;

    let clause = match second {
        Some(None) => {
            JsonBehaviorClause::new()
                .with_on_empty(first)
        },
        Some(Some(on_error)) => {
            JsonBehaviorClause::new()
                .with_on_empty(first)
                .with_on_error(on_error)
        },
        None => {
            JsonBehaviorClause::new()
                .with_on_error(first)
        }
    };

    Ok(clause)
}

/// Alias: `json_on_error_clause_opt`
pub(super) fn json_on_error_clause(ctx: &mut ParserContext) -> scan::Result<JsonBehavior> {

    /*
        json_behavior ON ERROR
    */

    let (behavior, ..) = seq!(json_behavior, On, ErrorKw)
        .parse(ctx)?;

    Ok(behavior)
}

/// Inlined: `json_behavior_type`
pub(super) fn json_behavior(ctx: &mut ParserContext) -> scan::Result<JsonBehavior> {

    /*
          ERROR
        | NULL
        | TRUE
        | FALSE
        | UNKNOWN
        | EMPTY (ARRAY | OBJECT)?
        | DEFAULT a_expr
    */

    alt!(
        ErrorKw.map(|_| Error),
        Kw::Null.map(|_| Null),
        Kw::True.map(|_| True),
        Kw::False.map(|_| False),
        Kw::Unknown.map(|_| Unknown),
        empty_behavior,
        default_behavior
    ).parse(ctx)
}

fn empty_behavior(ctx: &mut ParserContext) -> scan::Result<JsonBehavior> {

    /*
        EMPTY (ARRAY | OBJECT)?
    */

    let (_, behavior) = seq!(
        Empty,
        alt!(
            Array.map(|_| EmptyArray),
            Object.map(|_| EmptyObject)
        ).optional(/* non-standard, for Oracle compatibility only */)
    ).parse(ctx)?;

    let behavior = behavior.unwrap_or(EmptyArray);

    Ok(behavior)
}

fn default_behavior(ctx: &mut ParserContext) -> scan::Result<JsonBehavior> {

    /*
        DEFAULT a_expr
    */

    let (_, expr) = seq!(DefaultKw, a_expr).parse(ctx)?;
    Ok(JsonBehavior::Default(expr))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode::IntegerConst;
    use test_case::test_matrix;

    #[test_matrix("error on error" => Ok(
        JsonBehaviorClause::new()
            .with_on_error(Error)
    ))]
    #[test_matrix("false on empty" => Ok(
        JsonBehaviorClause::new()
            .with_on_empty(False)
    ))]
    #[test_matrix("true on empty false on error" => Ok(
        JsonBehaviorClause::new()
            .with_on_empty(True)
            .with_on_error(False)
    ))]
    fn test_json_behavior_clause(source: &str) -> scan::Result<JsonBehaviorClause> {
        test_parser!(source, json_behavior_clause)
    }

    #[test_matrix("null on error" => Ok(Null))]
    fn test_json_on_error_clause(source: &str) -> scan::Result<JsonBehavior> {
        test_parser!(source, json_on_error_clause)
    }

    #[test_matrix("error" => Ok(Error))]
    #[test_matrix("null" => Ok(Null))]
    #[test_matrix("true" => Ok(True))]
    #[test_matrix("false" => Ok(False))]
    #[test_matrix("unknown" => Ok(Unknown))]
    #[test_matrix("empty" => Ok(EmptyArray))]
    #[test_matrix("empty array" => Ok(EmptyArray))]
    #[test_matrix("empty object" => Ok(EmptyObject))]
    #[test_matrix("default 1" => Ok(JsonBehavior::Default(IntegerConst(1))))]
    fn test_json_behavior(source: &str) -> scan::Result<JsonBehavior> {
        test_parser!(source, json_behavior)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::expr::a_expr;
use crate::seq;
use crate::ParserContext;
use pg_ast::JsonBehavior;
use pg_ast::JsonBehavior::EmptyArray;
use pg_ast::JsonBehavior::EmptyObject;
use pg_ast::JsonBehavior::Error;
use pg_ast::JsonBehavior::False;
use pg_ast::JsonBehavior::Null;
use pg_ast::JsonBehavior::True;
use pg_ast::JsonBehavior::Unknown;
use pg_ast::JsonBehaviorClause;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Array;
use pg_lexer::Keyword::DefaultKw;
use pg_lexer::Keyword::Empty;
use pg_lexer::Keyword::ErrorKw;
use pg_lexer::Keyword::Object;
use pg_lexer::Keyword::On;
use pg_parser_core::scan;
