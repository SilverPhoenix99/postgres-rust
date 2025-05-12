pub(super) fn func_type() -> impl Combinator<Output = FuncType> {

    /*
          Typename
        | ( SETOF )? type_function_name attrs '%' TYPE_P
    */

    typename().chain(|typ, stream| {

        // In `Typename`, only generic types goes to `type_function_name`.
        let Generic { name, type_modifiers } = typ.name() else {
            return Ok(FuncType::Type(typ))
        };

        // Also, Type references (`%TYPE`):
        // 1. don't have modifiers;
        let ref_allowed = type_modifiers.is_empty()
            // 2. don't have array bounds;
            && typ.array_bounds().is_empty()
            // 3. must have a qualified name.
            && name.len() > 1;

        if !ref_allowed {
            return Ok(FuncType::Type(typ))
        }

        // `%TYPE`
        if Percent.and(Type).optional().parse(stream)?.is_none() {
            // If it isn't a type reference, just return the type
            return Ok(FuncType::Type(typ))
        }

        let (Generic { name, .. }, _, mult) = typ.deconstruct() else {
            // SAFETY: already checked that it's `Generic` above.
            unsafe { unreachable_unchecked() }
        };

        let typeref = TypeReference::new(name, mult);
        Ok(Reference(typeref))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node;
    use crate::parser::ast_node::SetOf;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_ref_func_type() {
        let mut stream = TokenStream::new("setof some_.qualified_name %type", DEFAULT_CONFIG);
        let actual = func_type().parse(&mut stream);

        let expected = TypeReference::new(
            vec!["some_".into(), "qualified_name".into()],
            SetOf::Table
        );

        assert_eq!(Ok(Reference(expected)), actual);
    }

    #[test]
    fn test_type_func_type() {
        let mut stream = TokenStream::new("setof some_.qualified_name[]", DEFAULT_CONFIG);
        let actual = func_type().parse(&mut stream);

        let expected = ast_node::Type::new(
            Generic {
                name: vec!["some_".into(), "qualified_name".into()],
                type_modifiers: vec![]
            },
            vec![None],
            SetOf::Table
        );

        assert_eq!(Ok(FuncType::Type(expected)), actual);
    }
}

use crate::parser::ast_node::FuncType;
use crate::parser::ast_node::FuncType::Reference;
use crate::parser::ast_node::TypeName::Generic;
use crate::parser::ast_node::TypeReference;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::typename;
use postgres_parser_lexer::Keyword::Type;
use postgres_parser_lexer::OperatorKind::Percent;
use std::hint::unreachable_unchecked;
