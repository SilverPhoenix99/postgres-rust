pg_basics::reexport! {
    a_expr_prec,
    a_expr_prec_0,
    a_expr_prec_1,
    a_expr_prec_10,
    a_expr_prec_12,
    a_expr_prec_13,
    a_expr_prec_14,
    a_expr_prec_3,
    a_expr_prec_4,
    a_expr_prec_7,
    a_expr_prec_8,
    a_expr_prec_9,
    a_expr_primary,
    json_predicate_type_constraint,
}

pub(in crate::combinators) fn a_expr(ctx: &mut ParserContext) -> scan::Result<ExprNode> {
    a_expr_prec(0).parse(ctx)
}

type PrecResult = Result<ExprNode, LocatedResult<ExprNode>>;

/// Wraps [`scan::Result`] into [`PrecResult`].
macro_rules! prec_wrap {
    ($ctx:ident, $lhs:ident, $parser:expr) => {{
        use pg_parser_core::scan::Error::{Eof, NoMatch, ScanErr};

        let p = $parser;

        let result = $crate::combinators::core::Combinator::parse(&p, $ctx);

        match result {
            Ok(expr) => expr,
            Err(NoMatch(_) | Eof(_)) => return Err(Ok($lhs)),
            Err(ScanErr(err)) => return Err(Err(err)),
        }
    }};
}
use prec_wrap;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::BinaryExpr;
    use pg_ast::BoolExpr;
    use pg_ast::BoolExpr::Not;
    use pg_ast::CollationExpr;
    use pg_ast::ExprNode::InArray;
    use pg_ast::ExprNode::IntegerConst as Int;
    use pg_ast::ExprNode::IsDistinct;
    use pg_ast::ExprNode::IsDocument;
    use pg_ast::ExprNode::IsFalse;
    use pg_ast::ExprNode::IsNormalized;
    use pg_ast::ExprNode::IsNotDistinct;
    use pg_ast::ExprNode::IsNotFalse;
    use pg_ast::ExprNode::IsNotNull;
    use pg_ast::ExprNode::IsNotTrue;
    use pg_ast::ExprNode::IsNotUnknown;
    use pg_ast::ExprNode::IsNull;
    use pg_ast::ExprNode::IsTrue;
    use pg_ast::ExprNode::IsUnknown;
    use pg_ast::ExprNode::StringConst;
    use pg_ast::JsonIsPredicate;
    use pg_ast::JsonValueKind;
    use pg_ast::Operator::Division;
    use pg_ast::Operator::Exponentiation;
    use pg_ast::Operator::LessEquals;
    use pg_ast::Operator::Subtraction;
    use pg_ast::TimezoneExpr;
    use pg_ast::TypeName::Varchar;
    use pg_ast::TypecastExpr;
    use pg_ast::UnicodeNormalForm;
    use test_case::test_matrix;

    /*
        Single expressions
     */
    #[test_matrix("1" => matches Ok(Int(1)))]
    #[test_matrix("1::varchar" => Ok(
        TypecastExpr::new(Int(1), Varchar { max_length: None }).into()
    ))]
    #[test_matrix("1 at time zone 'UTC'" => Ok(
        TimezoneExpr::new(
            Int(1),
            Some(StringConst("UTC".into()))
        ).into()
    ))]
    #[test_matrix("2 at local" => Ok(
        TimezoneExpr::new(Int(2), None).into()
    ))]
    #[test_matrix(r#"'foo' collate "C""# => Ok(
        CollationExpr::new(
            StringConst("foo".into()),
            vec!["C".into()]
        ).into()
    ))]
    #[test_matrix("1 not in (2, 3)" => Ok(
        Not(
            InArray(
                Int(1).into(),
                vec![Int(2), Int(3)]
            ).into()
        ).into()
    ))]
    #[test_matrix("1 in (2, 3)" => Ok(
        InArray(Int(1).into(), vec![Int(2), Int(3)])
    ))]
    #[test_matrix("1 isnull" => Ok(IsNull(Int(1).into())))]
    #[test_matrix("2 notnull" => Ok(IsNotNull(Int(2).into())))]
    #[test_matrix("3 is distinct from 4" => Ok(
        IsDistinct((Int(3), Int(4)).into())
    ))]
    #[test_matrix("5 is not distinct from 6" => Ok(
        IsNotDistinct((Int(5), Int(6)).into())
    ))]
    #[test_matrix("7 is document" => Ok(
        IsDocument(Int(7).into())
    ))]
    #[test_matrix("8 is not document" => Ok(
        Not(IsDocument(Int(8).into()).into()).into()
    ))]
    #[test_matrix("9 is false" => Ok(IsFalse(Int(9).into())))]
    #[test_matrix("10 is not false" => Ok(IsNotFalse(Int(10).into())))]
    #[test_matrix("11 is null" => Ok(IsNull(Int(11).into())))]
    #[test_matrix("12 is not null" => Ok(IsNotNull(Int(12).into())))]
    #[test_matrix("13 is true" => Ok(IsTrue(Int(13).into())))]
    #[test_matrix("14 is not true" => Ok(IsNotTrue(Int(14).into())))]
    #[test_matrix("15 is unknown" => Ok(IsUnknown(Int(15).into())))]
    #[test_matrix("16 is not unknown" => Ok(IsNotUnknown(Int(16).into())))]
    #[test_matrix("'foo' is nfc normalized" => Ok(
        IsNormalized(
            StringConst("foo".into()).into(),
            Some(UnicodeNormalForm::CanonicalComposition)
        )
    ))]
    #[test_matrix("'bar' is normalized" => Ok(
        IsNormalized(
            StringConst("bar".into()).into(),
            None
        )
    ))]
    #[test_matrix("'baz' is not nfd normalized" => Ok(
        Not(
            IsNormalized(
                StringConst("baz".into()).into(),
                Some(UnicodeNormalForm::CanonicalDecomposition)
            ).into()
        ).into()
    ))]
    #[test_matrix("'qux' is not normalized" => Ok(
        Not(
            IsNormalized(
                StringConst("qux".into()).into(),
                None
            ).into()
        ).into()
    ))]
    #[test_matrix("'[1]' is json" => Ok(
        JsonIsPredicate::new(StringConst("[1]".into()))
            .into()
    ))]
    #[test_matrix("'[2]' is json value" => Ok(
        JsonIsPredicate::new(StringConst("[2]".into()))
            .with_kind(JsonValueKind::Value)
            .into()
    ))]
    #[test_matrix(r#"'{"foo": 1}' is json with unique keys"# => Ok(
        JsonIsPredicate::new(StringConst(r#"{"foo": 1}"#.into()))
            .with_unique_keys(true)
            .into()
    ))]
    #[test_matrix(r#"'{"bar": 2}' is json object without unique"# => Ok(
        JsonIsPredicate::new(StringConst(r#"{"bar": 2}"#.into()))
            .with_kind(JsonValueKind::Object)
            .with_unique_keys(false)
            .into()
    ))]
    #[test_matrix("1 <= 2" => Ok(
        BinaryExpr::new(LessEquals, Int(1), Int(2)).into()
    ))]
    #[test_matrix("2 ^ 3" => Ok(
        BinaryExpr::new(Exponentiation, Int(2), Int(3)).into()
    ))]
    #[test_matrix("5 - 4" => Ok(
        BinaryExpr::new(Subtraction, Int(5), Int(4)).into()
    ))]
    #[test_matrix("6 / 3" => Ok(
        BinaryExpr::new(Division, Int(6), Int(3)).into()
    ))]
    /*
        Multiple expressions
    */
    #[test_matrix("1 and 2 and 3 or 4 or 5" => Ok(
        // ((1 AND 2 AND 3) OR 4 OR 5)
        BoolExpr::Or(vec![
            BoolExpr::And(vec![Int(1), Int(2), Int(3)]).into(),
            Int(4),
            Int(5),
        ]).into()
    ))]
    fn test_a_expr(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, a_expr)
    }
}

use crate::combinators::core::Combinator;
use crate::context::ParserContext;
use pg_ast::ExprNode;
use pg_elog::LocatedResult;
use pg_parser_core::scan;
