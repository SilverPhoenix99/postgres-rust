/// Alias: `Character`
///
/// Inlined:
/// * `CharacterWithLength`
/// * `CharacterWithoutLength`
/// * `character` (lowercase rule)
pub(super) fn character(default_len: Option<i32>) -> impl Combinator<Output = TypeName> {

    /*
          VARCHAR ( precision )?
        | (CHAR | CHARACTER | NCHAR) ( VARYING )? ( precision )?
        | NATIONAL (CHAR | CHARACTER) ( VARYING )? ( precision )?
    */

    parser(move |ctx| {
        let (varying, mut length) = seq!(
            alt!(
                Kw::Varchar.map(|_| true),
                seq!(
                    alt!(
                        Char.skip(),
                        Character.skip(),
                        Nchar.skip(),
                        seq!(
                            National,
                            alt!(Char, Character)
                        )
                            .skip()
                    ),
                    Varying.optional()
                        .map(|varying| varying.is_some())
                )
                    .map(|(_, varying)| varying),
            ),
            precision.optional()
        ).parse(ctx)?;

        if varying {
            return Ok(Varchar { max_length: length })
        }

        length = length.or(default_len);

        Ok(Bpchar { length })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_parser_core::scan;
    use test_case::test_matrix;

    #[test_matrix("varchar"                       => Ok(Varchar { max_length: None }))]
    #[test_matrix("char varying"                  => Ok(Varchar { max_length: None }))]
    #[test_matrix("varchar(3)"                    => Ok(Varchar { max_length: Some(3) }))]
    #[test_matrix("char varying(5)"               => Ok(Varchar { max_length: Some(5) }))]
    #[test_matrix("character varying"             => Ok(Varchar { max_length: None }))]
    #[test_matrix("character varying(2)"          => Ok(Varchar { max_length: Some(2) }))]
    #[test_matrix("nchar varying"                 => Ok(Varchar { max_length: None }))]
    #[test_matrix("nchar varying(7)"              => Ok(Varchar { max_length: Some(7) }))]
    #[test_matrix("national char varying"         => Ok(Varchar { max_length: None }))]
    #[test_matrix("national char varying(5)"      => Ok(Varchar { max_length: Some(5) }))]
    #[test_matrix("national character varying"    => Ok(Varchar { max_length: None }))]
    #[test_matrix("national character varying(3)" => Ok(Varchar { max_length: Some(3) }))]
    #[test_matrix("char"                          => Ok(Bpchar { length: Some(66) }))]
    #[test_matrix("char(4)"                       => Ok(Bpchar { length: Some(4) }))]
    #[test_matrix("character"                     => Ok(Bpchar { length: Some(66) }))]
    #[test_matrix("character(2)"                  => Ok(Bpchar { length: Some(2) }))]
    #[test_matrix("nchar"                         => Ok(Bpchar { length: Some(66) }))]
    #[test_matrix("nchar(9)"                      => Ok(Bpchar { length: Some(9) }))]
    #[test_matrix("national char"                 => Ok(Bpchar { length: Some(66) }))]
    #[test_matrix("national char(7)"              => Ok(Bpchar { length: Some(7) }))]
    #[test_matrix("national character"            => Ok(Bpchar { length: Some(66) }))]
    #[test_matrix("national character(8)"         => Ok(Bpchar { length: Some(8) }))]
    fn test_character(source: &str) -> scan::Result<TypeName> {
        test_parser!(source, character(Some(66)))
    }
}

use crate::alt;
use crate::combinators::core::parser;
use crate::combinators::core::Combinator;
use crate::combinators::precision;
use crate::seq;
use pg_ast::TypeName;
use pg_ast::TypeName::Bpchar;
use pg_ast::TypeName::Varchar;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Char;
use pg_lexer::Keyword::Character;
use pg_lexer::Keyword::National;
use pg_lexer::Keyword::Nchar;
use pg_lexer::Keyword::Varying;
