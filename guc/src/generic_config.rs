/// Variable fields, initialized at runtime
#[derive(Debug, Default)]
pub struct GenericConfig {
    /// status bits
    status: GucStatus,
    /// source of the current actual value
    source: GucSource,
    /// source of the reset_value
    reset_source: GucSource,
    /// context that set the current value
    scontext: GucContext,
    /// context that set the reset value
    reset_scontext: GucContext,
    /// role that set the current value
    srole: Oid,
    /// role that set the reset value
    reset_srole: Oid,
    /// stacked prior values
    stack: Vec<GucStack>,
    //*/ "extra" pointer for current actual value
    // extra: Option<Opaque>,
    //*/ list for variables that have source different from `GucSource::Default`
    // nondef_link: Vec<Opaque>,
    //*/ list for variables that have non-NULL stack
    // stack_link: Vec<Opaque>,
    //*/ list link for variables that have the `GUC_NEEDS_REPORT` bit set in status
    // report_link: Vec<Opaque>,
    /// if variable is `GUC_REPORT`, value last sent to client (`None` if not yet sent)
    last_reported: Option<String>,
    /// file current setting is from (`None` if not set in config file)
    sourcefile: Option<String>,
    /// line in source file
    sourceline: i32,
}

use crate::GucContext;
use crate::GucSource;
use crate::GucStack;
use crate::GucStatus;
use pg_basics::Oid;
