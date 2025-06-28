mod frame_bound;
mod frame_extent;
mod opt_existing_window_name;
mod opt_frame_clause;
mod opt_partition_clause;
mod opt_window_exclusion_clause;

pub(super) fn window_specification(stream: &mut TokenStream) -> scan::Result<WindowDefinition> {

    /*
        '(' opt_existing_window_name opt_partition_clause ( sort_clause )? opt_frame_clause ')'
    */

    let (name, partition, order, frame) = between!(paren : stream =>
        seq!(stream =>
            opt_existing_window_name,
            opt_partition_clause,
            sort_clause.optional(),
            opt_frame_clause
        )
    )?;

    let expr = WindowDefinition::new(name, partition, order, frame);
    Ok(expr)
}

#[allow(unused_imports)]
use self::{
    frame_bound::frame_bound,
    frame_extent::frame_extent,
    opt_existing_window_name::opt_existing_window_name,
    opt_frame_clause::opt_frame_clause,
    opt_partition_clause::opt_partition_clause,
    opt_window_exclusion_clause::opt_window_exclusion_clause,
};

use crate::combinators::foundation::between;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::sort_clause;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::WindowDefinition;
