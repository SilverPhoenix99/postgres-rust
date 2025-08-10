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
    use pg_combinators::test_parser;
    use pg_parser_core::scan;
    use test_case::test_case;

    #[test_case("varchar"                       => Ok(Varchar { max_length: None }))]
    #[test_case("char varying"                  => Ok(Varchar { max_length: None }))]
    #[test_case("varchar(3)"                    => Ok(Varchar { max_length: Some(3) }))]
    #[test_case("char varying(5)"               => Ok(Varchar { max_length: Some(5) }))]
    #[test_case("character varying"             => Ok(Varchar { max_length: None }))]
    #[test_case("character varying(2)"          => Ok(Varchar { max_length: Some(2) }))]
    #[test_case("nchar varying"                 => Ok(Varchar { max_length: None }))]
    #[test_case("nchar varying(7)"              => Ok(Varchar { max_length: Some(7) }))]
    #[test_case("national char varying"         => Ok(Varchar { max_length: None }))]
    #[test_case("national char varying(5)"      => Ok(Varchar { max_length: Some(5) }))]
    #[test_case("national character varying"    => Ok(Varchar { max_length: None }))]
    #[test_case("national character varying(3)" => Ok(Varchar { max_length: Some(3) }))]
    #[test_case("char"                          => Ok(Bpchar { length: Some(66) }))]
    #[test_case("char(4)"                       => Ok(Bpchar { length: Some(4) }))]
    #[test_case("character"                     => Ok(Bpchar { length: Some(66) }))]
    #[test_case("character(2)"                  => Ok(Bpchar { length: Some(2) }))]
    #[test_case("nchar"                         => Ok(Bpchar { length: Some(66) }))]
    #[test_case("nchar(9)"                      => Ok(Bpchar { length: Some(9) }))]
    #[test_case("national char"                 => Ok(Bpchar { length: Some(66) }))]
    #[test_case("national char(7)"              => Ok(Bpchar { length: Some(7) }))]
    #[test_case("national character"            => Ok(Bpchar { length: Some(66) }))]
    #[test_case("national character(8)"         => Ok(Bpchar { length: Some(8) }))]
    fn test_character(source: &str) -> scan::Result<TypeName> {
        test_parser!(source, character(Some(66)))
    }
}

use pg_ast::TypeName;
use pg_ast::TypeName::Bpchar;
use pg_ast::TypeName::Varchar;
use pg_combinators::alt;
use pg_combinators::parser;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Char;
use pg_lexer::Keyword::Character;
use pg_lexer::Keyword::National;
use pg_lexer::Keyword::Nchar;
use pg_lexer::Keyword::Varying;
use pg_sink_combinators::precision;
