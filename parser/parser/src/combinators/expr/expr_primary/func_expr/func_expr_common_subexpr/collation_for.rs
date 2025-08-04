pub(super) fn collation_for(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        COLLATION FOR '(' a_expr ')'
    */

    if ! matches!(stream.peek2(), Ok((K(Collation), K(For)))) {
        return no_match(stream)
    }

    let (_, expr) = seq!(skip(2), paren!(a_expr))
        .parse(stream)?;

    let expr = Box::new(expr);
    Ok(CollationForFunc(expr))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::StringConst,
        scan::Error::NoMatch,
    };

    #[test_case("collation for ('foo')" => Ok(
        CollationForFunc(
            Box::new(StringConst("foo".into()))
        )
    ))]
    #[test_case("collation()" => matches Err(NoMatch(_)) ; "collation_empty_paren")]
    #[test_case("collation 1" => matches Err(NoMatch(_)))]
    #[test_case("collation" => matches Err(NoMatch(_)))]
    fn test_collation_for(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, collation_for)
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use crate::combinators::foundation::Combinator;
use crate::no_match;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword as K;
use pg_ast::ExprNode;
use pg_ast::ExprNode::CollationForFunc;
use pg_lexer::Keyword::Collation;
use pg_lexer::Keyword::For;
