pub(super) fn func_type(stream: &mut TokenStream) -> scan::Result<FuncType> {

    /*
          Typename
        | ( SETOF )? type_function_name attrs '%' TYPE_P
    */

    let typ = typename(stream)?;

    // In `Typename`, only generic types goes to `type_function_name`.
    let Generic { name, type_modifiers } = typ.name() else {
        return Ok(FuncType::Type(typ))
    };

    // Also, Type references (`%TYPE`):
    // 1. don't have modifiers;
    let ref_allowed = type_modifiers.is_none()
        // 2. don't have array bounds;
        && typ.array_bounds().is_none()
        // 3. must have a qualified name.
        && name.len() > 1;

    if !ref_allowed {
        return Ok(FuncType::Type(typ))
    }

    // `%TYPE`
    if seq!(Percent, Type).parse(stream).optional()?.is_none() {
        // If it isn't a type reference, just return the type
        return Ok(FuncType::Type(typ))
    }

    let (Generic { name, .. }, _, mult) = typ.into() else {
        // SAFETY: already checked that it's `Generic` above.
        unsafe { unreachable_unchecked() }
    };

    let typeref = TypeReference::new(name, mult);
    Ok(Reference(typeref))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::SetOf;

    #[test]
    fn test_ref_func_type() {
        test_parser!(
            source = "setof some_.qualified_name %type",
            parser = func_type,
            expected = Reference(
                TypeReference::new(
                    vec!["some_".into(), "qualified_name".into()],
                    SetOf::Table
                )
            )
        )
    }

    #[test]
    fn test_type_func_type() {
        test_parser!(
            source = "setof some_.qualified_name[]",
            parser = func_type,
            expected = FuncType::Type(
                pg_ast::Type::new(
            Generic {
                name: vec!["some_".into(), "qualified_name".into()],
                type_modifiers: None
            },
            Some(vec![None]),
            SetOf::Table
        )
            )
        )
    }
}

use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::typename;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
use core::hint::unreachable_unchecked;
use pg_ast::FuncType;
use pg_ast::FuncType::Reference;
use pg_ast::TypeName::Generic;
use pg_ast::TypeReference;
use pg_lexer::Keyword::Type;
use pg_lexer::OperatorKind::Percent;
