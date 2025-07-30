pub(super) fn normalize(stream: &mut TokenStream) -> scan::Result<NormalizeFunc> {

    /*
        NORMALIZE '(' a_expr ( ',' unicode_normal_form )? ')'
    */

    if ! matches!(stream.peek2(), Ok((K(Normalize), Op(OpenParenthesis)))) {
        return no_match(stream)
    }

    let (expr, normal_form) = skip_prefix(1,
        between_paren((
            a_expr,
            (Comma, unicode_normal_form).optional()
        ))
    ).parse(stream)?;

    let normal_form = normal_form
        .map(|(_, normal_form)| normal_form);

    let expr = NormalizeFunc::new(expr, normal_form);
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::StringConst,
        scan::Error::NoMatch,
        pg_ast::UnicodeNormalForm::CanonicalComposition,
    };

    #[test_case("normalize('foo')" => Ok(
        NormalizeFunc::new(
            StringConst("foo".into()),
            None
        )
    ))]
    #[test_case("normalize('foo', nfc)" => Ok(
        NormalizeFunc::new(
            StringConst("foo".into()),
            Some(CanonicalComposition)
        )
    ))]
    #[test_case("normalize" => matches Err(NoMatch(_)))]
    #[test_case("normalize 1" => matches Err(NoMatch(_)))]
    fn test_normalize(source: &str) -> scan::Result<NormalizeFunc> {
        test_parser!(source, normalize)
    }
}

use crate::stream::TokenValue::Keyword as K;
use crate::stream::TokenValue::Operator as Op;
use pg_ast::NormalizeFunc;
use pg_lexer::Keyword::Normalize;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::OpenParenthesis;
use crate::combinators::expr::a_expr;
use crate::combinators::expr::unicode_normal_form::unicode_normal_form;
use crate::combinators::foundation::skip_prefix;
use crate::combinators::foundation::between_paren;
use crate::combinators::foundation::Combinator;
use crate::no_match;
use crate::scan;
use crate::stream::TokenStream;
