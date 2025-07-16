pub(super) fn ambiguous_prefix_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    match stream.peek2() {
        Ok((Keyword(Double), Keyword(Precision))) => {
            // Due to the condition, this will never return `NoMatch`.
            double_precision(stream)
        },
        Ok((Keyword(CurrentSchema), second))
        // `current_schema()` is valid syntax, so exclude that case.
        if ! matches!(second, Operator(OpenParenthesis))
        => {
            stream.next(); // Consume the `current_schema` keyword.
            Ok(ExprNode::CurrentSchema)
        },
        Ok((Keyword(Collation), Keyword(For))) => {
            collation_for(stream)
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

fn collation_for(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        COLLATION FOR '(' a_expr ')'
    */

    let (.., expr) = (Collation, For, between_paren(a_expr))
        .parse(stream)?;

    let expr = Box::new(expr);
    let expr = ExprNode::CollationFor(expr);
    Ok(expr)
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::between_paren;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::scan::Error::NoMatch;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword;
use crate::stream::TokenValue::Operator;
use pg_ast::ExprNode;
use pg_ast::ExprNode::StringConst;
use pg_ast::TypeName;
use pg_ast::TypecastExpr;
use pg_lexer::Keyword::Collation;
use pg_lexer::Keyword::CurrentSchema;
use pg_lexer::Keyword::Double;
use pg_lexer::Keyword::For;
use pg_lexer::Keyword::Precision;
use pg_lexer::OperatorKind::OpenParenthesis;
