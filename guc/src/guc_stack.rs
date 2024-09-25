#[derive(Debug)]
pub struct GucStack {
    /// Nesting depth at which we made entry.
    nest_level: i32,
    state: GucStackState,
    /// Source of the prior value.
    source: GucSource,
    /// Context that set the prior value.
    /// Masked value's source must be `GucSource::Session`, so no need to store it.
    scontext: GucContext,
    /// Context that set the masked value.
    masked_scontext: GucContext,
    /// Role that set the prior value.
    srole: Oid,
    /// Role that set the masked value.
    masked_srole: Oid,
    /// Previous value of variable.
    prior: ConfigVar,
    /// `SET` value in a `GucStackState::Local` entry.
    masked: ConfigVar,
}

use crate::{ConfigVar, GucContext, GucSource, GucStackState};
use postgres_basics::Oid;
