pub fn identifier_prefixed_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        (IDENT | unreserved_keyword) (
              indirection => columnref
            | ( attrs )? (
                  SCONST                                             => AexprConst
                | '(' func_arg_list ')' SCONST                       => AexprConst
                | '(' ( func_application_args )? ')' func_args_tail  => func_expr
                | ε                                                  => columnref
            )
        )
    */

    let (name, indirection) = (
        or((
            identifier.map(Str::from),
            Unreserved.map(Str::from)
        )),
        located(indirection).optional()
    ).parse(stream)?;

    let column_ref = make_column_ref(name, indirection)?;

    let name = match QualifiedName::try_from(column_ref) {
        Ok(name) => name,
        Err(column_ref) => {
            // columnref
            return Ok(column_ref.into())
        },
    };

    let Some(tail) = attr_tail(stream).optional()? else {
        // columnref
        let mut name = name;
        let expr = match name.as_mut_slice() {
            [name] => ColumnRef::SingleName(mem::take(name)),
            _ => ColumnRef::Name(name)
        };
        return Ok(expr.into())
    };

    let expr = tailed_expr::tailed_expr(name, tail);
    Ok(expr)
}

use super::attr_tail;
use super::tailed_expr;
use crate::combinators::expr::indirection;
use crate::combinators::foundation::identifier;
use crate::combinators::foundation::located;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::make_column_ref;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
use core::mem;
use pg_ast::ColumnRef;
use pg_ast::ExprNode;
use pg_basics::QualifiedName;
use pg_basics::Str;
use pg_lexer::KeywordCategory::Unreserved;
