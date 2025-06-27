/// Alias: `AexprConst`
pub(super) fn expr_const() -> impl Combinator<Output = ExprNode> {

    /*
          ICONST
        | FCONST
        | SCONST
        | BCONST
        | XCONST
        | TRUE
        | FALSE
        | NULL
        | ConstTypename SCONST
    */

    match_first! {
        number.map(From::from),
        string.map(StringConst),
        bit_string
            .map(|(kind, value)| match kind {
                BitStringKind::Binary => BinaryStringConst(value),
                BitStringKind::Hex => HexStringConst(value),
            }),
        True.map(|_| BooleanConst(true)),
        False.map(|_| BooleanConst(false)),
        Null.map(|_| NullConst),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use pg_ast::ExprNode::*;
    #[allow(unused_imports)]
    use pg_ast::TypeName::*;
    #[allow(unused_imports)]
    use pg_basics::NumberRadix::Decimal;
    use test_case::test_case;

    #[test_case("123", IntegerConst(123))]
    #[test_case("123.45", NumericConst { radix: Decimal, value: "123.45".into() })]
    #[test_case("true", BooleanConst(true))]
    #[test_case("false", BooleanConst(false))]
    #[test_case("null", NullConst)]
    #[test_case("b'0101'", BinaryStringConst("0101".into()))]
    #[test_case("x'19af'", HexStringConst("19af".into()))]
    #[test_case("'string literal'", StringConst("string literal".into()))]
    fn test_expr_const(source: &str, expected: ExprNode) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = expr_const().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::combinators::foundation::bit_string;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::number;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use pg_ast::ExprNode;
use pg_ast::ExprNode::BinaryStringConst;
use pg_ast::ExprNode::BooleanConst;
use pg_ast::ExprNode::HexStringConst;
use pg_ast::ExprNode::NullConst;
use pg_ast::ExprNode::StringConst;
use pg_lexer::BitStringKind;
use pg_lexer::Keyword::False;
use pg_lexer::Keyword::Null;
use pg_lexer::Keyword::True;
