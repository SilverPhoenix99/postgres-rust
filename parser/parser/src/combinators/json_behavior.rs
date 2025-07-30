/// Inlined: `json_behavior_type`
pub(super) fn json_behavior(stream: &mut TokenStream) -> scan::Result<JsonBehavior> {

    /*
          ERROR
        | NULL
        | TRUE
        | FALSE
        | UNKNOWN
        | EMPTY (ARRAY | OBJECT)?
        | DEFAULT a_expr
    */

    or((
        ErrorKw.map(|_| Error),
        Kw::Null.map(|_| Null),
        Kw::True.map(|_| True),
        Kw::False.map(|_| False),
        Kw::Unknown.map(|_| Unknown),
        empty_behavior,
        default_behavior
    )).parse(stream)
}

fn empty_behavior(stream: &mut TokenStream) -> scan::Result<JsonBehavior> {

    /*
        EMPTY (ARRAY | OBJECT)?
    */

    let (_, behavior) = (
        Empty,
        or((
            Array.map(|_| EmptyArray),
            Object.map(|_| EmptyObject)
        )).optional(/* non-standard, for Oracle compatibility only */)
    ).parse(stream)?;

    let behavior = behavior.unwrap_or(EmptyArray);

    Ok(behavior)
}

fn default_behavior(stream: &mut TokenStream) -> scan::Result<JsonBehavior> {

    /*
        DEFAULT a_expr
    */

    let (_, expr) = (DefaultKw, a_expr).parse(stream)?;
    Ok(JsonBehavior::Default(expr))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::JsonBehavior;
    use test_case::test_case;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::{IntegerConst, StringConst};

    #[test_case("error" => Ok(Error))]
    #[test_case("null" => Ok(Null))]
    #[test_case("true" => Ok(True))]
    #[test_case("false" => Ok(False))]
    #[test_case("unknown" => Ok(Unknown))]
    #[test_case("empty" => Ok(EmptyArray))]
    #[test_case("empty array" => Ok(EmptyArray))]
    #[test_case("empty object" => Ok(EmptyObject))]
    #[test_case("default 1" => Ok(JsonBehavior::Default(IntegerConst(1))))]
    fn test_json_behavior(source: &str) -> scan::Result<JsonBehavior> {
        test_parser!(source, json_behavior)
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::JsonBehavior;
use pg_ast::JsonBehavior::EmptyArray;
use pg_ast::JsonBehavior::EmptyObject;
use pg_ast::JsonBehavior::Error;
use pg_ast::JsonBehavior::False;
use pg_ast::JsonBehavior::Null;
use pg_ast::JsonBehavior::True;
use pg_ast::JsonBehavior::Unknown;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Array;
use pg_lexer::Keyword::DefaultKw;
use pg_lexer::Keyword::Empty;
use pg_lexer::Keyword::ErrorKw;
use pg_lexer::Keyword::Object;
