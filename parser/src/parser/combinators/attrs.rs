/// Post-condition: Vec is **Not** empty
pub(super) fn attrs<F>(prefix: F) -> impl Combinator<Output = QualifiedName>
where
    F: Combinator<Output = Str>
{
    /*
        prefix ( '.' col_label )*
    */

    many_pre(
        prefix,
        Dot.and_right(col_label::col_label())
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::combinators;
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_attrs() {
        let source = ".qualified_.name_";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let parser = combinators::foundation::parser(|_| Ok("*some*".into()));
        let actual = attrs(parser).parse(&mut stream);

        let expected: QualifiedName = vec![
            "*some*".into(),
            "qualified_".into(),
            "name_".into()
        ];

        assert_eq!(Ok(expected), actual);
    }
}

use crate::lexer::OperatorKind::Dot;
use crate::parser::ast_node::QualifiedName;
use crate::parser::combinators::col_label;
use crate::parser::combinators::foundation::many_pre;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
