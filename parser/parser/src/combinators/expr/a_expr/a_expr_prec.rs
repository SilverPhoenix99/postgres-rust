/// Runs the parser and decodes [`PrecResult`].
macro_rules! prec_parse {

    ($lhs:ident, $prec_combinator:expr => continue) => {

        let result = $prec_combinator;
        match result {
            Ok(expr) => {
                $lhs = expr;
                continue
            },
            Err(expr) => $lhs = expr?,
        }
    };

    ($lhs:ident, $prec_combinator:expr => return) => {

        let result = $prec_combinator;
        match result {
            Ok(expr) => return Ok(expr),
            Err(expr) => $lhs = expr?,
        }
    };
}

macro_rules! prec_climb {

    ($prec_var:ident, $( $prec:literal : $prec_fn:ident => $type:ident ),+ $(,)?) => {
        move |ctx| {

            let mut lhs = a_expr_primary(ctx)?;

            loop {

                $(
                    if $prec_var <= $prec {
                        prec_parse!(lhs, $prec_fn(ctx, lhs) => $type);
                    }
                )+

                // No more matches
                return Ok(lhs)
            }
        }
    }
}

pub(super) fn a_expr_prec(prec: u8) -> impl Fn(&mut ParserContext) -> scan::Result<ExprNode> {

    prec_climb! { prec,
        // a_expr TYPECAST Typename  -- %left(14)
        14: a_expr_prec_14 => continue,

        // a_expr NOT IN '(' expr_list ')'  -- %left(13)
        // a_expr IN '(' expr_list ')'  -- %left(13)
        13: a_expr_prec_13 => continue,

        // a_expr AT ( LOCAL | TIME ZONE a_expr )  -- %left(12)
        12: a_expr_prec_12 => continue,

        // a_expr COLLATE any_name  -- %left(10)
        10: a_expr_prec_10 => continue,

        // a_expr '^' a_expr  -- %left(9)
        9: a_expr_prec_9 => continue,

        // a_expr additive_op a_expr  -- %left(8)
        8: a_expr_prec_8 => continue,

        // a_expr multiplicative_op a_expr  -- %left(7)
        7: a_expr_prec_7 => continue,

        /*
            All %left(6):
                  a_expr '^' sub_type '(' ( SelectStmt | a_expr ) ')'
                | a_expr additive_op sub_type '(' ( SelectStmt | a_expr ) ')'
                | a_expr multiplicative_op sub_type '(' ( SelectStmt | a_expr ) ')'
                | a_expr misc_op sub_type '(' ( SelectStmt | a_expr ) ')'
                | a_expr misc_op a_expr
                | a_expr ILIKE sub_type '(' ( SelectStmt | a_expr ) ')'
                | a_expr LIKE sub_type '(' ( SelectStmt | a_expr ) ')'
                | a_expr NOT ILIKE sub_type '(' ( SelectStmt | a_expr ) ')'
                | a_expr NOT LIKE sub_type '(' ( SelectStmt | a_expr ) ')'
                | a_expr boolean_op sub_type '(' ( SelectStmt | a_expr ) ')'
        */
        // TODO 6: a_expr_prec_6 => continue,

        /*
            All %nonassoc(5):
                  a_expr ( NOT )? IN '(' SelectStmt ')'
                | a_expr ( NOT )? ILIKE a_expr ( ESCAPE a_expr )?
                | a_expr ( NOT )? LIKE a_expr ( ESCAPE a_expr )?
                | a_expr ( NOT )? BETWEEN ( ASYMMETRIC | SYMMETRIC )? b_expr AND a_expr
                | a_expr ( NOT )? SIMILAR TO a_expr ( ESCAPE a_expr )?
        */
        // TODO 5: a_expr_prec_5 => return,

        // a_expr boolean_op a_expr  -- %nonassoc(4)
        4: a_expr_prec_4 => return,

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
        3: a_expr_prec_3 => return,

        // a_expr AND a_expr  -- %left(1)
        1: a_expr_prec_1 => continue,

        // a_expr OR a_expr  -- %left(0)
        0: a_expr_prec_0 => continue,
    }
}

use super::a_expr_prec_0;
use super::a_expr_prec_1;
use super::a_expr_prec_10;
use super::a_expr_prec_12;
use super::a_expr_prec_13;
use super::a_expr_prec_14;
use super::a_expr_prec_3;
use super::a_expr_prec_4;
use super::a_expr_prec_7;
use super::a_expr_prec_8;
use super::a_expr_prec_9;
use super::a_expr_primary;
use crate::context::ParserContext;
use pg_ast::ExprNode;
use pg_parser_core::scan;
