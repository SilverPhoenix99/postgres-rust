/// Alias: `Bit`
///
/// Inlined:
/// * `BitWithLength`
/// * `BitWithoutLength`
pub(super) fn bit(default_type_modifiers:  Option<i32>) -> impl Combinator<Output = TypeName> {

    /*
        BIT ( VARYING )? ( type_modifiers )?
    */

    parser(move |ctx| {
        let (_, varying, mut modifiers) = seq!(
            Kw::Bit,
            Varying.optional()
                .map(|varying| varying),
            type_modifiers.optional()
        ).parse(ctx)?;

        if varying.is_some() {
            return Ok(Varbit(modifiers))
        }

        modifiers = modifiers.or_else(||
            default_type_modifiers
                .map(|len| vec![IntegerConst(len)])
        );

        Ok(Bit(modifiers))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ParserContext;
    use pg_parser_core::scan;
    use test_case::test_case;

    #[test_case("bit"             => Ok(Bit(Some(vec![IntegerConst(3)]))))]
    #[test_case("bit(77)"         => Ok(Bit(Some(vec![IntegerConst(77)]))))]
    #[test_case("bit varying"     => Ok(Varbit(None)))]
    #[test_case("bit varying(55)" => Ok(Varbit(Some(vec![IntegerConst(55)]))))]
    fn test_bit(source: &str) -> scan::Result<TypeName> {
        let mut ctx = ParserContext::new(source);
        bit(Some(3)).parse(&mut ctx)
    }
}

use super::type_modifiers;
use crate::combinators::core::parser;
use crate::combinators::core::Combinator;
use crate::seq;
use pg_ast::ExprNode::IntegerConst;
use pg_ast::TypeName;
use pg_ast::TypeName::Bit;
use pg_ast::TypeName::Varbit;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Varying;
