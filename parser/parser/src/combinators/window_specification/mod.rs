mod existing_window_name;
mod frame_bound;
mod frame_clause;
mod frame_extent;
mod partition_clause;
mod window_exclusion_clause;

pub(super) fn window_specification(stream: &mut TokenStream) -> scan::Result<WindowDefinition> {

    /*
        '('
            ( existing_window_name )?
            ( partition_clause )?
            ( sort_clause )?
            ( frame_clause )?
        ')'
    */

    let (name, partition, order, frame) = between_paren(
        (
            existing_window_name.optional(),
            partition_clause.optional(),
            sort_clause.optional(),
            frame_clause.optional()
        )
    ).parse(stream)?;

    let expr = WindowDefinition::new(name, partition, order, frame);
    Ok(expr)
}

#[allow(unused_imports)]
use self::{
    existing_window_name::existing_window_name,
    frame_bound::frame_bound,
    frame_clause::frame_clause,
    frame_extent::frame_extent,
    partition_clause::partition_clause,
    window_exclusion_clause::window_exclusion_clause,
};

use crate::combinators::foundation::between_paren;
use crate::combinators::foundation::Combinator;
use crate::combinators::sort_clause;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::WindowDefinition;
