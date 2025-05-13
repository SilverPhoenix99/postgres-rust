/// Post-condition: Vec is **Not** empty
pub(super) fn function_with_argtypes_list() -> impl Combinator<Output = Vec<FunctionWithArgs>> {

    /*
        function_with_argtypes ( ',' function_with_argtypes )*
    */

    many_sep(Comma, function_with_argtypes())
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

        attrs(or(
            Unreserved.map(From::from),
            identifier().map(From::from)
        ))
            .and_then(func_args(), FunctionWithArgs::new),

        attrs(ColumnName.map(From::from))
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

/// Post-condition: Vec **May** be empty.
///
/// # Return
/// The combinator returns `Option<_>` over a possibly missing arguments list:
/// * `None` if there's no arguments specified, i.e., `(` didn't match;
/// * `Some(_)` if there are parenthesis, but the arguments list might still be empty. E.g.s:
///     * `"()"`: An empty list returns `Some([])`;
///     * `"(arg1, arg2)"`: If arguments exist, then it returns them `Some([arg1, arg2])`.
fn func_args() -> impl Combinator<Output = Option<Vec<FunctionParameter>>> {

    /*
        ( '(' ( func_args_list )? ')' )?
    */

    between_paren(
        func_args_list()
            .optional()
            .map(Option::unwrap_or_default)
    )
        .optional()
}

/// Post-condition: Vec is **Not** empty
fn func_args_list() -> impl Combinator<Output = Vec<FunctionParameter>> {

    /*
        func_arg ( ',' func_arg )*
    */

    many_sep(Comma, func_arg())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    #[allow(unused_imports)]
    use postgres_parser_ast::{
        FuncType,
        TypeName,
    };
    use test_case::test_case;

    // type_func_name_keyword ( func_args )?
    #[test_case("collation", FunctionWithArgs::new(vec!["collation".into()], None))]
    #[test_case("current_schema()", FunctionWithArgs::new(vec!["current_schema".into()], Some(vec![])))]
    // unreserved_keyword ( attrs )? ( func_args )?
    #[test_case("double.trouble()", FunctionWithArgs::new(vec!["double".into(), "trouble".into()], Some(vec![])))]
    #[test_case("double.double", FunctionWithArgs::new(vec!["double".into(), "double".into()], None))]
    #[test_case("double()", FunctionWithArgs::new(vec!["double".into()], Some(vec![])))]
    #[test_case("double", FunctionWithArgs::new(vec!["double".into()], None))]
    // IDENT ( attrs )? ( func_args )?
    #[test_case("ident.qualified_()", FunctionWithArgs::new(vec!["ident".into(), "qualified_".into()], Some(vec![])))]
    #[test_case("qualif.ident", FunctionWithArgs::new(vec!["qualif".into(), "ident".into()], None))]
    #[test_case("ident()", FunctionWithArgs::new(vec!["ident".into()], Some(vec![])))]
    #[test_case("ident", FunctionWithArgs::new(vec!["ident".into()], None))]
    // col_name_keyword ( attrs ( func_args )? )?
    #[test_case("float.point()", FunctionWithArgs::new(vec!["float".into(), "point".into()], Some(vec![])))]
    #[test_case("float.boat", FunctionWithArgs::new(vec!["float".into(), "boat".into()], None))]
    #[test_case("float", FunctionWithArgs::new(vec!["float".into()], None))]
    fn test_function_with_argtypes(source: &str, expected: FunctionWithArgs) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = function_with_argtypes().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }

    #[test_case("", None)]
    #[test_case("()", Some(vec![]))]
    #[test_case("(json, int)", Some(vec![FuncType::Type(TypeName::Json.into()).into(), FuncType::Type(TypeName::Int4.into()).into()]))]
    fn test_func_args(source: &str, expected: Option<Vec<FunctionParameter>>) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = func_args().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::parser::combinators::attrs;
use crate::parser::combinators::between_paren;
use crate::parser::combinators::foundation::identifier;
use crate::parser::combinators::foundation::many_sep;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::func_arg;
use postgres_parser_ast::FunctionParameter;
use postgres_parser_ast::FunctionWithArgs;
use postgres_parser_lexer::KeywordCategory::ColumnName;
use postgres_parser_lexer::KeywordCategory::TypeFuncName;
use postgres_parser_lexer::KeywordCategory::Unreserved;
use postgres_parser_lexer::OperatorKind::Comma;
