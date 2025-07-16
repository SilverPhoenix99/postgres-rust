pub(super) fn ambiguous_prefix_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    match stream.peek2() {
        Ok((Keyword(Double), Keyword(Precision))) => {
            // Due to the condition, this will never return `NoMatch`.
            double_precision(stream)
        },
        _ => Err(NoMatch(stream.current_location()))
    }
}

fn double_precision(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        DOUBLE PRECISION SCONST
    */

    let (.., value) = (Double, Precision, string).parse(stream)?;
    let expr = TypecastExpr::new(StringConst(value), TypeName::Float8);
    Ok(expr.into())
}

use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::scan::Error::NoMatch;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword;
use pg_ast::ExprNode;
use pg_ast::ExprNode::StringConst;
use pg_ast::TypeName;
use pg_ast::TypecastExpr;
use pg_lexer::Keyword::Double;
use pg_lexer::Keyword::Precision;
