impl Parser<'_> {
    /// Alias: `AexprConst`
    pub(in crate::parser) fn expr_const(&mut self) -> ScanResult<ExprNode> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::expr_const";

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

        if let Some(num) = self.unsigned_number().no_match_to_option()? {
            return Ok(num.into())
        }

        if let Some(string) = self.string().optional()? {
            return Ok(StringConst(string))
        }

        let bit_string_const = self.bit_string()
            .optional()?
            .map(|(kind, value)| match kind {
                Binary => BinaryStringConst(value),
                Hex => HexStringConst(value),
            });

        if let Some(bit_string_const) = bit_string_const {
            return Ok(bit_string_const)
        }

        if let Some(mut type_name) = self.const_typename().optional()? {
            let value = self.string().required(fn_info!(FN_NAME))?;

            if let Interval(IntervalRange::Full { precision: None }) = type_name {
                // NB: `const_typename()` doesn't make this specific match,
                // because `SCONST` is between `INTERVAL` and `opt_interval` (i.e., `INTERVAL SCONST opt_interval`),
                // so that match is done here, if `INTERVAL` wasn't followed by `'(' ICONST ')'`
                let range = self.opt_interval()?;
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
    use super::*;
    use crate::parser::ast_node::ExprNode;
    use crate::parser::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("123", ExprNode::IntegerConst(123))]
    #[test_case("123.45", ExprNode::NumericConst { radix: 10, value: "123.45".into() })]
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

use crate::{
    lexer::{
        BitStringKind::*,
        Keyword::{False, Null, True},
        RawTokenKind::Keyword as Kw
    },
    parser::{
        ast_node::{
            ExprNode::{self, BinaryStringConst, BooleanConst, HexStringConst, NullConst, StringConst},
            IntervalRange,
            TypeName::Interval,
            TypecastExpr
        },
        consume_macro::consume,
        result::{
            Optional,
            Required,
            ScanErrorKind::NoMatch,
            ScanResult,
            ScanResultTrait
        },
        Parser,
    }
};
use postgres_basics::fn_info;
