pub(in crate::combinators::stmt) fn transform(ctx: &mut ParserContext) -> scan::Result<Transform> {

    /*
        TRANSFORM FOR Typename LANGUAGE ColId
    */

    let (_, _, for_type, _, language) = seq!(Kw::Transform, For, typename, Language, col_id)
        .parse(ctx)?;

    Ok(Transform::new(for_type, language))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_ast::TypeName::Int4;
    use pg_combinators::test_parser;

    #[test]
    fn test_transform() {
        test_parser!(
            source = "transform for int language plpgsql",
            parser = transform,
            expected = Transform::new(Int4, "plpgsql")
        )
    }
}

use pg_ast::Transform;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::For;
use pg_lexer::Keyword::Language;
use pg_parser_core::scan;
use pg_sink_combinators::col_id;
use pg_type_combinators::typename;
