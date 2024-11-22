impl Parser<'_> {
    /// Alias: `AexprConst`
    pub(in crate::parser) fn expr_const(&mut self) -> ScanResult<ExprNode> {

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

        if let Some(num) = number().maybe_match().parse(&mut self.buffer)? {
            return Ok(num.into())
        }

        if let Some(string) = string().optional().parse(&mut self.buffer)? {
            return Ok(StringConst(string))
        }

        let bit_string_const = bit_string()
            .optional()
            .parse(&mut self.buffer)?
            .map(|(kind, value)| match kind {
                Binary => BinaryStringConst(value),
                Hex => HexStringConst(value),
            });

        if let Some(bit_string_const) = bit_string_const {
            return Ok(bit_string_const)
        }

        if let Some(mut type_name) = self.const_typename().optional()? {
            let value = string().required().parse(&mut self.buffer)?;

            if let Interval(IntervalRange::Full { precision: None }) = type_name {
                // NB: `const_typename()` doesn't make this specific match,
                // because `SCONST` is between `INTERVAL` and `opt_interval` (i.e., `INTERVAL SCONST opt_interval`),
                // so that match is done here, if `INTERVAL` wasn't followed by `'(' ICONST ')'`
                let range = opt_interval().parse(&mut self.buffer)?;
                type_name = Interval(range)
            }

            let typecast = TypecastExpr::new(type_name.into(), StringConst(value));
            return Ok(typecast.into())
        }

        consume!{self
            Ok {
                Kw(True) => Ok(BooleanConst(true)),
                Kw(False) => Ok(BooleanConst(false)),
                Kw(Null) => Ok(NullConst),
            }
            Err {
                Err(err) => err.into(),
                _ => {
                    let loc = self.buffer.current_location();
                    NoMatch(loc)
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::ast_node::ExprNode::{self, *};
    use crate::parser::ast_node::{TypeName::*, *};
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::Parser;
    use test_case::test_case;

    #[test_case("123", IntegerConst(123))]
    #[test_case("123.45", NumericConst { radix: crate::NumberRadix::Decimal, value: "123.45".into() })]
    #[test_case("true", BooleanConst(true))]
    #[test_case("false", BooleanConst(false))]
    #[test_case("null", NullConst)]
    #[test_case("b'0101'", BinaryStringConst("0101".into()))]
    #[test_case("x'19af'", HexStringConst("19af".into()))]
    #[test_case("'string literal'", StringConst("string literal".into()))]
    #[test_case("interval '1' day", TypecastExpr::new(Interval(IntervalRange::Day).into(), StringConst("1".into())).into())]
    fn test_expr_const(source: &str, expected: ExprNode) {
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.expr_const();
        assert_eq!(actual, Ok(expected))
    }
}

use crate::lexer::BitStringKind::*;
use crate::lexer::Keyword::False;
use crate::lexer::Keyword::Null;
use crate::lexer::Keyword::True;
use crate::lexer::RawTokenKind::Keyword as Kw;
use crate::parser::ast_node::ExprNode;
use crate::parser::ast_node::ExprNode::BinaryStringConst;
use crate::parser::ast_node::ExprNode::BooleanConst;
use crate::parser::ast_node::ExprNode::HexStringConst;
use crate::parser::ast_node::ExprNode::NullConst;
use crate::parser::ast_node::ExprNode::StringConst;
use crate::parser::ast_node::IntervalRange;
use crate::parser::ast_node::TypeName::Interval;
use crate::parser::ast_node::TypecastExpr;
use crate::parser::combinators::bit_string;
use crate::parser::combinators::number;
use crate::parser::combinators::string;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::consume_macro::consume;
use crate::parser::opt_interval::opt_interval;
use crate::parser::result::Optional;
use crate::parser::result::ScanErrorKind::NoMatch;
use crate::parser::result::ScanResult;
use crate::parser::Parser;
