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

pub(super) fn a_expr_prec_3(ctx: &mut ParserContext, lhs: ExprNode) -> PrecResult {

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

    let (not, rhs) = prec_wrap!(ctx, lhs,
        alt!(
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
        )
    );

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

    Ok(expr)

}

use self::IsExprRhs::*;
use super::a_expr_prec;
use super::prec_wrap;
use super::PrecResult;
use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::expr::a_expr::json_predicate_type_constraint;
use crate::combinators::expr::unicode_normal_form;
use crate::combinators::json_key_uniqueness_constraint;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::BoolExpr::Not;
use pg_ast::ExprNode;
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
use pg_ast::UnicodeNormalForm;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Distinct;
use pg_lexer::Keyword::FromKw;
use pg_lexer::Keyword::Is;
use pg_lexer::Keyword::Isnull;
use pg_lexer::Keyword::Notnull;
