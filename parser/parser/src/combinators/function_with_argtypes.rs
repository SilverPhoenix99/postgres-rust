pub(super) fn function_with_argtypes_list() -> impl Combinator<Output = Vec<FunctionWithArgs>> {

    /*
        function_with_argtypes ( ',' function_with_argtypes )*
    */

    many!(sep = Comma, function_with_argtypes())
}

pub(super) fn function_with_argtypes() -> impl Combinator<Output = FunctionWithArgs> {

    /*
        Original production:
              func_name func_args
            | type_func_name_keyword
            | ColId ( attrs )?

        The original production used `indirection` instead of `attrs`,
        but the only valid rule from `indirection` is: `'.' ColLabel`.
        This is why there's no need to call [check_func_name](https://github.com/postgres/postgres/blob/e974f1c2164bc677d55f98edaf99f80c0b6b89d9/src/backend/parser/gram.y#L18976).
        See [function_with_argtypes](https://github.com/postgres/postgres/blob/e974f1c2164bc677d55f98edaf99f80c0b6b89d9/src/backend/parser/gram.y#L8471).

        Refactored production to remove conflicts:
              type_func_name_keyword ( func_args )?
            | unreserved_keyword ( attrs )? ( func_args )?
            | IDENT ( attrs )? ( func_args )?
            | col_name_keyword ( attrs ( func_args )? )?
    */

    match_first! {
        TypeFuncName.map(|kw| vec![From::from(kw)])
            .and_then(func_args(), FunctionWithArgs::new),

        attrs!(or(
            Unreserved.map(From::from),
            identifier.map(From::from)
        ))
            .and_then(func_args(), FunctionWithArgs::new),

        attrs!(ColumnName.map(From::from))
            .chain(|name, stream| {
                if name.len() == 1 {
                    return Ok(FunctionWithArgs::new(name, None))
                }
                // arguments are only allowed when the function name is qualified
                let args = func_args().parse(stream)?;
                Ok(FunctionWithArgs::new(name, args))
            })
    }
}

/// # Return
/// The combinator returns `Option<_>` over a possibly missing arguments list:
/// * `None` if there's no arguments specified, i.e., `(` didn't match;
/// * `Some(_)` if there are parenthesis, but the arguments list might still be empty. E.g.s:
///     * `"()"`: An empty list returns `Some(None)`;
///     * `"(arg1, arg2)"`: If arguments exist, then it returns them `Some(Some([arg1, arg2]))`.
fn func_args() -> impl Combinator<Output = Option<Option<Vec<FunctionParameter>>>> {

    /*
        ( '(' ( func_args_list )? ')' )?
    */

    between_paren(func_args_list.optional())
        .optional()
}

fn func_args_list(stream: &mut TokenStream) -> Result<Vec<FunctionParameter>> {

    /*
        func_arg ( ',' func_arg )*
    */

    many!(sep = Comma, func_arg()).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    #[allow(unused_imports)]
    use pg_ast::{
        FuncType,
        TypeName,
    };
    use test_case::test_case;

    // type_func_name_keyword ( func_args )?
    #[test_case("collation", FunctionWithArgs::new(vec!["collation".into()], None))]
    #[test_case("current_schema()", FunctionWithArgs::new(vec!["current_schema".into()], Some(None)))]
    // unreserved_keyword ( attrs )? ( func_args )?
    #[test_case("double.trouble()", FunctionWithArgs::new(vec!["double".into(), "trouble".into()], Some(None)))]
    #[test_case("double.double", FunctionWithArgs::new(vec!["double".into(), "double".into()], None))]
    #[test_case("double()", FunctionWithArgs::new(vec!["double".into()], Some(None)))]
    #[test_case("double", FunctionWithArgs::new(vec!["double".into()], None))]
    // IDENT ( attrs )? ( func_args )?
    #[test_case("ident.qualified_()", FunctionWithArgs::new(vec!["ident".into(), "qualified_".into()], Some(None)))]
    #[test_case("qualif.ident", FunctionWithArgs::new(vec!["qualif".into(), "ident".into()], None))]
    #[test_case("ident()", FunctionWithArgs::new(vec!["ident".into()], Some(None)))]
    #[test_case("ident", FunctionWithArgs::new(vec!["ident".into()], None))]
    // col_name_keyword ( attrs ( func_args )? )?
    #[test_case("float.point()", FunctionWithArgs::new(vec!["float".into(), "point".into()], Some(None)))]
    #[test_case("float.boat", FunctionWithArgs::new(vec!["float".into(), "boat".into()], None))]
    #[test_case("float", FunctionWithArgs::new(vec!["float".into()], None))]
    fn test_function_with_argtypes(source: &str, expected: FunctionWithArgs) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = function_with_argtypes().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }

    #[test_case("", None)]
    #[test_case("()", Some(None))]
    #[test_case("(json, int)", Some(Some(vec![
        FuncType::Type(TypeName::Json.into()).into(),
        FuncType::Type(TypeName::Int4.into()).into()
    ])))]
    fn test_func_args(source: &str, expected: Option<Option<Vec<FunctionParameter>>>) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = func_args().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::combinators::attrs;
use crate::combinators::between_paren;
use crate::combinators::foundation::identifier;
use crate::combinators::foundation::many;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::func_arg;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_ast::FunctionParameter;
use pg_ast::FunctionWithArgs;
use pg_lexer::KeywordCategory::ColumnName;
use pg_lexer::KeywordCategory::TypeFuncName;
use pg_lexer::KeywordCategory::Unreserved;
use pg_lexer::OperatorKind::Comma;
