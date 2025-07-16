pub(super) fn type_func_name_prefixed_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        type_func_name_keyword
        (
              SCONST                                             => AexprConst
            | '(' func_arg_list ')' SCONST                       => AexprConst
            | '(' ( func_application_args )? ')' func_args_tail  => func_expr
        )
    */

    let (kw, tail) = (TypeFuncName, attr_tail).parse(stream)?;
    let name = vec![Str::from(kw)];

    let expr = tailed_expr::tailed_expr(name, tail);
    Ok(expr)
}

use super::attr_tail;
use super::tailed_expr;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
use pg_basics::Str;
use pg_lexer::KeywordCategory::TypeFuncName;
