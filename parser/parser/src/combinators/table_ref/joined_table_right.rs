#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct JoinRightSide {
    pub table_ref: TableRef,
    pub join_kind: JoinKind,
    pub alias: Option<Alias>,
}

pub(super) fn joined_table_right(ctx: &mut ParserContext) -> scan::Result<JoinRightSide> {

    /*
          join_type_prefix JOIN table_ref_primary
        | ( join_type )? JOIN table_ref_primary join_qual
    */

    alt!(unqualified_join, qualified_join).parse(ctx)
}

fn unqualified_join(ctx: &mut ParserContext) -> scan::Result<JoinRightSide> {

    /*
        join_type_prefix JOIN table_ref_primary
    */

    let (join_kind, _, table_ref) = seq!(join_type_prefix, Join, table_ref_primary)
        .parse(ctx)?;

    Ok(JoinRightSide {
        table_ref,
        join_kind,
        alias: None
    })
}

fn qualified_join(ctx: &mut ParserContext) -> scan::Result<JoinRightSide> {

    /*
        ( join_type )? JOIN table_ref_primary join_qual
    */

    let (join_kind, _, table_ref, (join_qual, alias)) = seq!(
        join_type.optional(),
        Join,
        table_ref_primary,
        join_qual
    ).parse(ctx)?;

    let join_kind = match join_kind {
        Some(Left(_)) => Left(join_qual),
        Some(Full(_)) => Full(join_qual),
        Some(Right(_)) => Right(join_qual),
        _ => Inner(Some(join_qual)),
    };

    Ok(JoinRightSide {
        table_ref,
        join_kind,
        alias
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode::BooleanConst;
    use pg_ast::JoinQual;
    use pg_ast::RelationTableRef;
    use test_case::test_matrix;

    #[test_matrix("cross join foo" => Ok(JoinRightSide {
        table_ref: RelationTableRef::new("foo").into(),
        join_kind: JoinKind::cross_join(),
        alias: None
    }))]
    fn test_unqualified_join(source: &str) -> scan::Result<JoinRightSide> {
        test_parser!(source, unqualified_join)
    }

    #[test_matrix("join bar on true" => Ok(JoinRightSide {
        table_ref: RelationTableRef::new("bar").into(),
        join_kind: Inner(Some(
            JoinQual::On(
                Box::new(BooleanConst(true))
            )
        )),
        alias: None
    }))]
    #[test_matrix("inner join baz using(qux)" => Ok(JoinRightSide {
        table_ref: RelationTableRef::new("baz").into(),
        join_kind: Inner(Some(
            JoinQual::Using(
                vec!["qux".into()]
            )
        )),
        alias: None
    }))]
    fn test_qualified_join(source: &str) -> scan::Result<JoinRightSide> {
        test_parser!(source, qualified_join)
    }

    #[test_matrix("natural join a" => matches Ok(_))]
    #[test_matrix("join b using(x)" => matches Ok(_))]
    #[test_matrix("left join c on true" => matches Ok(_))]
    fn test_joined_table_right(source: &str) -> scan::Result<JoinRightSide> {
        test_parser!(source, joined_table_right)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::table_ref::join_qual;
use crate::combinators::table_ref::join_type;
use crate::combinators::table_ref::join_type_prefix;
use crate::combinators::table_ref::table_ref_primary;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::Alias;
use pg_ast::JoinKind;
use pg_ast::JoinKind::Full;
use pg_ast::JoinKind::Inner;
use pg_ast::JoinKind::Left;
use pg_ast::JoinKind::Right;
use pg_ast::TableRef;
use pg_lexer::Keyword::Join;
use pg_parser_core::scan;
