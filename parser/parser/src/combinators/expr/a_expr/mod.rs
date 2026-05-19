pg_basics::reexport! {
    a_expr_primary,
    json_predicate_type_constraint,
}

pub(in crate::combinators) fn a_expr(ctx: &mut ParserContext) -> scan::Result<ExprNode> {
    a_expr_prec(0).parse(ctx)
}

enum IsExprRhs {
    DistinctFrom(ExprNode),
    False,
    Null,
    True,
    Unknown,
    Document,
    Normalized(Option<UnicodeNormalForm>),
    Json {
        kind: JsonValueKind,
        unique_keys: bool,
    },
}

macro_rules! prec_wrap {
    ($ctx:ident, $lhs:ident, $parser:expr) => {{

        let p = $parser;

        let result = $crate::combinators::core::Combinator::parse(&p, $ctx);
        let result = pg_parser_core::Optional::optional(result);

        match result {
            Ok(Some(expr)) => expr,
            Ok(None) => return Err(Ok($lhs)),
            Err(err) => return Err(Err(err)),
        }
    }};
}

macro_rules! prec_unwrap {
    ($ctx:ident, $lhs:ident, $prec_fn:ident => continue) => {{
        let result = $prec_fn($ctx, $lhs);
        match result {
            Ok(expr) => {
                $lhs = expr;
                continue
            },
            Err(expr) => $lhs = expr?,
        }
    }};
    ($ctx:ident, $lhs:ident, $prec_fn:ident => return) => {{
        let result = $prec_fn($ctx, $lhs);
        match result {
            Ok(expr) => return Ok(expr),
            Err(expr) => $lhs = expr?,
        }
    }};
}

fn a_expr_prec(prec: u8) -> impl Fn(&mut ParserContext) -> scan::Result<ExprNode> {
    move |ctx| {

        /*
            Converted to precedence climbing.

            a_expr:
                  ✅ a_expr_primary
                | ✅ a_expr TYPECAST Typename                                                  // %left(14)

                | ✅ a_expr IN '(' expr_list ')'                                               // %left(13)
                | a_expr IN '(' SelectStmt ')'                                              // %nonassoc(5)

                | ✅ a_expr NOT IN '(' expr_list ')'                                           // %left(13)
                | a_expr NOT IN '(' SelectStmt ')'                                          // %nonassoc(5)

                | ✅ a_expr AT ( LOCAL | TIME ZONE a_expr )                                    // %left(12)
                | ✅ a_expr COLLATE any_name                                                   // %left(10)

                | ✅ a_expr '^' a_expr                                                         // %left(9)
                | a_expr '^' sub_type '(' ( SelectStmt | a_expr ) ')'                       // %left(6)

                | ✅ a_expr additive_op a_expr                                                 // %left(8)
                | a_expr additive_op sub_type '(' ( SelectStmt | a_expr ) ')'               // %left(6)

                | ✅ a_expr multiplicative_op a_expr                                           // %left(7)
                | a_expr multiplicative_op sub_type '(' ( SelectStmt | a_expr ) ')'         // %left(6)

                | a_expr misc_op sub_type '(' ( SelectStmt | a_expr ) ')'                   // %left(6)
                | a_expr misc_op a_expr                                                     // %left(6)

                | a_expr ILIKE sub_type '(' ( SelectStmt | a_expr ) ')'                     // %left(6)
                | a_expr ILIKE a_expr ( ESCAPE a_expr )?                                    // %nonassoc(5)

                | a_expr LIKE sub_type '(' ( SelectStmt | a_expr ) ')'                      // %left(6)
                | a_expr LIKE a_expr ( ESCAPE a_expr )?                                     // %nonassoc(5)

                | a_expr NOT ILIKE sub_type '(' ( SelectStmt | a_expr ) ')'                 // %left(6)
                | a_expr NOT ILIKE a_expr ( ESCAPE a_expr )?                                // %nonassoc(5)
                | a_expr NOT LIKE sub_type '(' ( SelectStmt | a_expr ) ')'                  // %left(6)
                | a_expr NOT LIKE a_expr ( ESCAPE a_expr )?                                 // %nonassoc(5)

                | a_expr boolean_op sub_type '(' ( SelectStmt | a_expr ) ')'                // %left(6)
                | ✅ a_expr boolean_op a_expr                                                  // %nonassoc(4)

                | a_expr ( NOT )? BETWEEN ( ASYMMETRIC | SYMMETRIC )? b_expr AND a_expr     // %nonassoc(5)
                | a_expr ( NOT )? SIMILAR TO a_expr ( ESCAPE a_expr )?                      // %nonassoc(5)

                | ✅ a_expr ISNULL                                                            // %nonassoc(3)
                | ✅ a_expr NOTNULL                                                           // %nonassoc(3)
                | ✅ a_expr IS ( NOT )? DISTINCT FROM a_expr                                  // %nonassoc(3)
                | ✅ a_expr IS ( NOT )? DOCUMENT                                              // %nonassoc(3)
                | ✅ a_expr IS ( NOT )? FALSE                                                 // %nonassoc(3)
                | ✅ a_expr IS ( NOT )? NULL                                                  // %nonassoc(3)
                | ✅ a_expr IS ( NOT )? TRUE                                                  // %nonassoc(3)
                | ✅ a_expr IS ( NOT )? UNKNOWN                                               // %nonassoc(3)
                | ✅ a_expr IS ( NOT )? ( unicode_normal_form )? NORMALIZED                   // %nonassoc(3)
                | ✅ a_expr IS ( NOT )? JSON                                                  // %nonassoc(3)
                      ( json_predicate_type_constraint )?
                      ( json_key_uniqueness_constraint )?

                | ✅ a_expr AND a_expr                                                        // %left(1)
                | ✅ a_expr OR a_expr                                                         // %left(0)
        */

        let mut lhs = a_expr_primary(ctx)?;

        loop {

            // a_expr TYPECAST Typename  -- %left(14)
            if prec <= 14 {
                prec_unwrap!(ctx, lhs, a_expr_prec_14 => continue);
            }

            if prec <= 13 {

                // a_expr NOT IN '(' expr_list ')'  -- %left(13)
                if
                    let Ok(toks) = ctx.stream_mut().peek_n::<5>()
                    && matches!(toks, [Keyword(Kw::Not), Keyword(In), Operator(OpenParenthesis), ..])
                    // must Not be select_stmt
                    && ! matches!(toks,
                        [.., Keyword(With | Select | Table), _]
                        | [.., Keyword(Values), Operator(OpenParenthesis)]
                    )
                    && let Some((_, expr_list)) = seq!(skip(2), paren!(expr_list))
                        .parse(ctx)
                        .optional()?
                {
                    lhs = Not(InArray(expr_list).into()).into();
                    continue
                }

                // a_expr IN '(' expr_list ')'  -- %left(13)
                if
                    let Ok(toks) = ctx.stream_mut().peek_n::<4>()
                    && matches!(toks, [Keyword(In), Operator(OpenParenthesis), ..])
                    // must Not be select_stmt
                    && ! matches!(toks,
                        [.., Keyword(With | Select | Table), _]
                        | [.., Keyword(Values), Operator(OpenParenthesis)]
                    )
                    && let Some((_, expr_list)) = seq!(skip(1), paren!(expr_list))
                        .parse(ctx)
                        .optional()?
                {
                    lhs = InArray(expr_list);
                    continue
                }
            }

            // a_expr AT ( LOCAL | TIME ZONE a_expr )  -- %left(12)
            if prec <= 12 && let Some((_, zone)) = {
                seq!(At, alt!(
                    Local.map(|_| None),
                    seq!(Time, Zone, a_expr_prec(13)).map(|(.., tz)| Some(tz))
                ))
                .parse(ctx)
                .optional()?
            } {
                lhs = TimezoneExpr::new(lhs, zone).into();
                continue
            }

            // a_expr COLLATE any_name  -- %left(10)
            if prec <= 10 && let Some(collation) = collate_clause(ctx).optional()? {
                lhs = CollationExpr::new(lhs, collation).into();
                continue
            }

            // a_expr '^' a_expr  -- %left(9)
            if prec <= 9
                // must Not be followed by `ALL(`/`ANY(`/`SOME(`
                && ! matches!(ctx.stream_mut().peek_n::<3>(), Ok([
                    Operator(Circumflex),
                    Keyword(All | Any | SomeKw),
                    Operator(OpenParenthesis)
                ]))
                && let Some((op, rhs)) = {
                    seq!(exponentiation_op, a_expr_prec(10))
                        .parse(ctx)
                        .optional()?
            } {
                lhs = BinaryExpr::new(op, lhs, rhs).into();
                continue
            }

            // a_expr additive_op a_expr  -- %left(8)
            if prec <= 8
                // must Not be followed by `ALL(`/`ANY(`/`SOME(`
                && ! matches!(ctx.stream_mut().peek_n::<3>(), Ok([
                    Operator(Minus | Plus),
                    Keyword(All | Any | SomeKw),
                    Operator(OpenParenthesis)
                ]))
                && let Some((op, rhs)) = {
                    seq!(additive_op, a_expr_prec(9))
                        .parse(ctx)
                        .optional()?
                }
            {
                lhs = BinaryExpr::new(op, lhs, rhs).into();
                continue
            }

            // a_expr multiplicative_op a_expr  -- %left(7)
            if prec <= 7
                // must Not be followed by `ALL(`/`ANY(`/`SOME(`
                && ! matches!(ctx.stream_mut().peek_n::<3>(), Ok([
                    Operator(Mul | Div | Percent),
                    Keyword(All | Any | SomeKw),
                    Operator(OpenParenthesis)
                ]))
                && let Some((op, rhs)) = {
                    seq!(multiplicative_op, a_expr_prec(8))
                        .parse(ctx)
                        .optional()?
                }
            {
                lhs = BinaryExpr::new(op, lhs, rhs).into();
                continue
            }

            // TODO

            // a_expr boolean_op a_expr  -- %nonassoc(4)
            if prec <= 4 {
                prec_unwrap!(ctx, lhs, a_expr_prec_4 => return);
            }

            /*
                All %nonassoc(3):
                  a_expr ISNULL
                | a_expr NOTNULL
                | a_expr IS ( NOT )? DISTINCT FROM a_expr_prec(4)
                | a_expr IS ( NOT )? DOCUMENT
                | a_expr IS ( NOT )? FALSE
                | a_expr IS ( NOT )? NULL
                | a_expr IS ( NOT )? TRUE
                | a_expr IS ( NOT )? UNKNOWN
                | a_expr IS ( NOT )? ( unicode_normal_form )? NORMALIZED
                | a_expr IS ( NOT )? JSON ( json_predicate_type_constraint )? ( json_key_uniqueness_constraint )?
            */
            if prec <= 3 {

                let rhs = alt!(
                    Isnull.map(|_| (None, Null)),
                    Notnull.map(|_| (Some(Kw::Not), Null)),
                    seq!(
                        Is,
                        Kw::Not.optional(),
                        alt!(
                            seq!(Distinct, FromKw, a_expr_prec(4))
                                .map(|(.., rhs)| DistinctFrom(rhs)),
                            Kw::Document.map(|_| Document),
                            Kw::False.map(|_| False),
                            Kw::Null.map(|_| Null),
                            Kw::True.map(|_| True),
                            Kw::Unknown.map(|_| Unknown),
                            seq!(
                                unicode_normal_form.optional(),
                                Kw::Normalized
                            ).map(|(form, _)|
                                Normalized(form)
                            ),
                            seq!(
                                Kw::Json,
                                json_predicate_type_constraint.optional(),
                                json_key_uniqueness_constraint.optional()
                            ).map(|(_, kind, unique_keys)|
                                Json {
                                    kind: kind.unwrap_or_default(),
                                    unique_keys: unique_keys.unwrap_or_default()
                                }
                            )
                        )
                    )
                    .map(|(.., not, rhs)| (not, rhs))
                ).parse(ctx)
                    .optional()?;

                if let Some((not, rhs)) = rhs {

                    let expr = match (rhs, not.is_some()) {
                        (DistinctFrom(rhs), false) => IsDistinct((lhs, rhs).into()),
                        (DistinctFrom(rhs), true) => IsNotDistinct((lhs, rhs).into()),
                        (False, false) => IsFalse(lhs.into()),
                        (False, true) => IsNotFalse(lhs.into()),
                        (Null, false) => IsNull(lhs.into()),
                        (Null, true) => IsNotNull(lhs.into()),
                        (True, false) => IsTrue(lhs.into()),
                        (True, true) => IsNotTrue(lhs.into()),
                        (Unknown, false) => IsUnknown(lhs.into()),
                        (Unknown, true) => IsNotUnknown(lhs.into()),
                        (Document, not) => {
                            let expr = IsDocument(lhs.into());
                            if not {
                                Not(expr.into()).into()
                            }
                            else {
                                expr
                            }
                        },
                        (Normalized(form), not) => {
                            let expr = IsNormalized(lhs.into(), form);
                            if not {
                                Not(expr.into()).into()
                            }
                            else {
                                expr
                            }
                        },
                        (Json { kind, unique_keys }, not) => {

                            let expr = JsonIsPredicate::new(lhs)
                                .with_kind(kind)
                                .with_unique_keys(unique_keys);

                            let expr = IsJson(expr.into());

                            if not {
                                Not(expr.into()).into()
                            }
                            else {
                                expr
                            }
                        },
                    };

                    return Ok(expr)
                }
            }

            // a_expr AND a_expr  -- %left(1)
            if prec <= 1 && let Some((_, rhs)) = seq!(And, a_expr_prec(2)).parse(ctx).optional()? {

                if let ExprNode::BoolExpr(BoolExpr::And(args)) = &mut lhs {
                    // Flatten "a AND b AND c ..." to a single BoolExpr on sight
                    args.push(rhs);
                }
                else {
                    lhs = BoolExpr::And(vec![lhs, rhs]).into();
                }

                continue
            }

            // a_expr OR a_expr  -- %left(0)
            if prec == 0 {
                prec_unwrap!(ctx, lhs, a_expr_prec_0 => continue);
            }

            // No more matches
            return Ok(lhs)
        }

    }
}

type PrecResult = Result<ExprNode, LocatedResult<ExprNode>>;

fn a_expr_prec_14(ctx: &mut ParserContext, mut lhs: ExprNode) -> PrecResult {

    /*
        a_expr TYPECAST Typename  -- %left(14)
    */

    let (_, rhs) = prec_wrap!(ctx, lhs,
        seq!(Typecast, typename)
    );

    lhs = TypecastExpr::new(lhs, rhs).into();
    Ok(lhs)
}

fn a_expr_prec_4(ctx: &mut ParserContext, mut lhs: ExprNode) -> PrecResult {

    /*
        a_expr boolean_op a_expr  -- %nonassoc(4)
    */

    if ! matches!(ctx.stream_mut().peek(), Ok(Operator(Less | Equals | Greater | LessEquals | GreaterEquals | NotEquals)))
        || matches!(ctx.stream_mut().peek_n::<3>(), Ok([_, Keyword(All | Any | SomeKw), Operator(OpenParenthesis)]))
    {
        return Err(Ok(lhs))
    }

    let (op, rhs) = prec_wrap!(ctx, lhs, seq!(boolean_op, a_expr_prec(5)));

    let expr = BinaryExpr::new(op, lhs, rhs);
    Ok(expr.into())
}

fn a_expr_prec_0(ctx: &mut ParserContext, mut lhs: ExprNode) -> PrecResult {

    /*
        a_expr OR a_expr  -- %left(0)
    */

    let (_, rhs) = prec_wrap!(ctx, lhs,
        seq!(Or, a_expr_prec(1))
    );

    if let ExprNode::BoolExpr(BoolExpr::Or(args)) = &mut lhs {
        // Flatten "a OR b OR c ..." to a single BoolExpr on sight
        args.push(rhs);
    }
    else {
        lhs = BoolExpr::Or(vec![lhs, rhs]).into()
    }

    Ok(lhs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode::IntegerConst as Int;
    use pg_ast::ExprNode::StringConst;
    use pg_ast::Operator::Division;
    use pg_ast::Operator::Exponentiation;
    use pg_ast::Operator::LessEquals;
    use pg_ast::Operator::Subtraction;
    use pg_ast::TypeName::Varchar;
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
        Not(InArray(vec![Int(2), Int(3)]).into()).into()
    ))]
    #[test_matrix("1 in (2, 3)" => Ok(
        InArray(vec![Int(2), Int(3)])
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

use self::IsExprRhs::DistinctFrom;
use self::IsExprRhs::Document;
use self::IsExprRhs::False;
use self::IsExprRhs::Json;
use self::IsExprRhs::Normalized;
use self::IsExprRhs::Null;
use self::IsExprRhs::True;
use self::IsExprRhs::Unknown;
use crate::alt;
use crate::combinators::additive_op;
use crate::combinators::boolean_op;
use crate::combinators::collate_clause;
use crate::combinators::core::skip;
use crate::combinators::core::Combinator;
use crate::combinators::exponentiation_op;
use crate::combinators::expr::unicode_normal_form;
use crate::combinators::expr_list;
use crate::combinators::json_key_uniqueness_constraint;
use crate::combinators::multiplicative_op;
use crate::combinators::typename;
use crate::context::ParserContext;
use crate::paren;
use crate::seq;
use pg_ast::BinaryExpr;
use pg_ast::BoolExpr;
use pg_ast::BoolExpr::Not;
use pg_ast::CollationExpr;
use pg_ast::ExprNode;
use pg_ast::ExprNode::InArray;
use pg_ast::ExprNode::IsDistinct;
use pg_ast::ExprNode::IsDocument;
use pg_ast::ExprNode::IsFalse;
use pg_ast::ExprNode::IsJson;
use pg_ast::ExprNode::IsNormalized;
use pg_ast::ExprNode::IsNotDistinct;
use pg_ast::ExprNode::IsNotFalse;
use pg_ast::ExprNode::IsNotNull;
use pg_ast::ExprNode::IsNotTrue;
use pg_ast::ExprNode::IsNotUnknown;
use pg_ast::ExprNode::IsNull;
use pg_ast::ExprNode::IsTrue;
use pg_ast::ExprNode::IsUnknown;
use pg_ast::JsonIsPredicate;
use pg_ast::JsonValueKind;
use pg_ast::TimezoneExpr;
use pg_ast::TypecastExpr;
use pg_ast::UnicodeNormalForm;
use pg_elog::LocatedResult;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::All;
use pg_lexer::Keyword::And;
use pg_lexer::Keyword::Any;
use pg_lexer::Keyword::At;
use pg_lexer::Keyword::Distinct;
use pg_lexer::Keyword::FromKw;
use pg_lexer::Keyword::In;
use pg_lexer::Keyword::Is;
use pg_lexer::Keyword::Isnull;
use pg_lexer::Keyword::Local;
use pg_lexer::Keyword::Notnull;
use pg_lexer::Keyword::Or;
use pg_lexer::Keyword::Select;
use pg_lexer::Keyword::SomeKw;
use pg_lexer::Keyword::Table;
use pg_lexer::Keyword::Time;
use pg_lexer::Keyword::Values;
use pg_lexer::Keyword::With;
use pg_lexer::Keyword::Zone;
use pg_lexer::OperatorKind::Circumflex;
use pg_lexer::OperatorKind::Div;
use pg_lexer::OperatorKind::Equals;
use pg_lexer::OperatorKind::Greater;
use pg_lexer::OperatorKind::GreaterEquals;
use pg_lexer::OperatorKind::Less;
use pg_lexer::OperatorKind::LessEquals;
use pg_lexer::OperatorKind::Minus;
use pg_lexer::OperatorKind::Mul;
use pg_lexer::OperatorKind::NotEquals;
use pg_lexer::OperatorKind::OpenParenthesis;
use pg_lexer::OperatorKind::Percent;
use pg_lexer::OperatorKind::Plus;
use pg_lexer::OperatorKind::Typecast;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenValue::Keyword;
use pg_parser_core::stream::TokenValue::Operator;
use pg_parser_core::Optional;
