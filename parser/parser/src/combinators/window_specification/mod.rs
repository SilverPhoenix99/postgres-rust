mod frame_bound;
mod frame_extent;
mod opt_existing_window_name;
mod opt_frame_clause;
mod opt_partition_clause;
mod opt_window_exclusion_clause;

pub(super) fn window_specification() -> impl Combinator<Output = WindowDefinition> {

    /*
        '(' opt_existing_window_name opt_partition_clause ( sort_clause )? opt_frame_clause ')'
    */

    between_paren(
        sequence!(
            opt_existing_window_name(),
            opt_partition_clause(),
            sort_clause().optional(),
            opt_frame_clause()
        )
        .map(|(name, partition, order, frame)|
            WindowDefinition::new(name, partition, order, frame)
        )
    )
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
use crate::combinators::between_paren;
use crate::combinators::foundation::sequence;
use crate::combinators::foundation::Combinator;
use crate::combinators::sort_clause;
use pg_ast::WindowDefinition;
