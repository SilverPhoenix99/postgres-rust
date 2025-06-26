pub(super) fn func_arg() -> impl Combinator<Output = FunctionParameter> {

    /*
          arg_class ( type_function_name )? func_type
        | type_function_name ( arg_class )? func_type
        |                                   func_type
    */

    // The 1st token of `func_type` might be a `type_function_name`, so this production is LL(2),
    // due to the conflict with the optional argument name which is also `type_function_name`.

    arg_class().optional().chain(|mut mode, stream| {

        let (first, second) = stream.peek2();

        let arg_name = if is_arg_name(&first, &second) {
            // It's the argument name.
            // Regardless of `arg_class` matching or not, `is_arg_name()` returned `true`,
            // so this is guaranteed to be `Some` argument name.
            Some(type_function_name.required().parse(stream)?)
        }
        else {
            None
        };

        if mode.is_none() && arg_name.is_some() {
            // `arg_class` didn't match before the argument name, so it might still match after.
            mode = arg_class().optional().parse(stream)?;
        }

        let func_type = if mode.is_none() && arg_name.is_none() {
            // Nothing matched before, so it's still optional
            func_type().parse(stream)?
        }
        else {
            // At least 1 matched
            func_type().required().parse(stream)?
        };

        // In case `arg_class` didn't match, there's still a default that can be applied.
        let mode = mode.unwrap_or_default();

        let func_arg = FunctionParameter::new(arg_name, mode, func_type);
        Ok(func_arg)
    })
}

fn arg_class() -> impl Combinator<Output = FunctionParameterMode> {
    use FunctionParameterMode::*;

    match_first!(
        Kw::In.and_right(
            Kw::Out.optional()
                .map(|out| if out.is_some() { InOut } else { In })
        ),
        Kw::Out.map(|_| Out),
        Kw::Inout.map(|_| InOut),
        Kw::Variadic.map(|_| Variadic),
    )
}

fn is_arg_name(
    first: &eof::Result<&TokenValue>,
    second: &eof::Result<&TokenValue>,
) -> bool {

    match (first, second) {

        (Ok(Identifier(_)), Ok(Identifier(_) | Keyword(_))) => true,

        (Ok(Keyword(_)), Ok(Identifier(_))) => true,

        // Double is an Unreserved keyword that can conflict with the argument name.
        // E.g.:
        // * In `double double precision`, the 1st `double` will be the argument name.
        // * In `double precision`, the argument is anonymous.
        (Ok(Keyword(Double)), Ok(Keyword(kw2))) => kw2 != &Precision,

        (Ok(Keyword(kw)), Ok(Keyword(_)))
            => matches!(kw.category(), Unreserved | TypeFuncName),

        _ => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use pg_ast::FuncType;
    use pg_ast::SetOf;
    use pg_ast::Type;
    use pg_ast::TypeName;
    use pg_basics::Str;
    use test_case::test_case;

    #[test_case("json", None, FunctionParameterMode::Default, TypeName::Json, SetOf::Record)]
    #[test_case("in json", None, FunctionParameterMode::In, TypeName::Json, SetOf::Record)]
    #[test_case("inout double precision", None, FunctionParameterMode::InOut, TypeName::Float8, SetOf::Record)]
    #[test_case("double out double precision", Some("double".into()), FunctionParameterMode::Out, TypeName::Float8, SetOf::Record)]
    #[test_case("double double precision", Some("double".into()), FunctionParameterMode::Default, TypeName::Float8, SetOf::Record)]
    #[test_case("double int", Some("double".into()), FunctionParameterMode::Default, TypeName::Int4, SetOf::Record)]
    #[test_case("setof json", None, FunctionParameterMode::Default, TypeName::Json, SetOf::Table)]
    fn test_func_arg(
        source: &str,
        arg_name: Option<Str>,
        mode: FunctionParameterMode,
        type_name: TypeName,
        set_of: SetOf
    ) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = func_arg().parse(&mut stream);

        let expected = FunctionParameter::new(
            arg_name,
            mode,
            FuncType::Type(Type::new(type_name, None, set_of))
        );

        assert_eq!(Ok(expected), actual);
    }

    #[test_case("in", FunctionParameterMode::In)]
    #[test_case("in out", FunctionParameterMode::InOut)]
    #[test_case("out", FunctionParameterMode::Out)]
    #[test_case("inout", FunctionParameterMode::InOut)]
    #[test_case("variadic", FunctionParameterMode::Variadic)]
    fn test_arg_class(source: &str, expected: FunctionParameterMode) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(expected), arg_class().parse(&mut stream));
    }
}

use crate::combinators::foundation::match_first;
use crate::combinators::foundation::Combinator;
use crate::combinators::func_type;
use crate::combinators::type_function_name;
use crate::eof;
use crate::stream::TokenValue;
use crate::stream::TokenValue::Identifier;
use crate::stream::TokenValue::Keyword;
use pg_ast::FunctionParameter;
use pg_ast::FunctionParameterMode;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Double;
use pg_lexer::Keyword::Precision;
use pg_lexer::KeywordCategory::TypeFuncName;
use pg_lexer::KeywordCategory::Unreserved;
