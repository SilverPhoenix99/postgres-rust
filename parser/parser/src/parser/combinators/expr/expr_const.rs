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
        number().map(From::from),
        string().map(StringConst),
        bit_string()
            .map(|(kind, value)| match kind {
                Binary => BinaryStringConst(value),
                Hex => HexStringConst(value),
            }),
        True.map(|_| BooleanConst(true)),
        False.map(|_| BooleanConst(false)),
        Null.map(|_| NullConst),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::ExprNode::*;
    #[allow(unused_imports)]
    use crate::parser::ast_node::TypeName::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    #[allow(unused_imports)]
    use postgres_parser_lexer::NumberRadix::Decimal;
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

use crate::parser::ast_node::ExprNode;
use crate::parser::ast_node::ExprNode::BinaryStringConst;
use crate::parser::ast_node::ExprNode::BooleanConst;
use crate::parser::ast_node::ExprNode::HexStringConst;
use crate::parser::ast_node::ExprNode::NullConst;
use crate::parser::ast_node::ExprNode::StringConst;
use crate::parser::combinators::foundation::bit_string;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::number;
use crate::parser::combinators::foundation::string;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_parser_lexer::BitStringKind::*;
use postgres_parser_lexer::Keyword::False;
use postgres_parser_lexer::Keyword::Null;
use postgres_parser_lexer::Keyword::True;
