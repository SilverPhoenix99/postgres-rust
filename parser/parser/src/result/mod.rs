pub(super) mod eof;

mod maybe_match;
mod optional;
mod required;
mod scan_error_kind;
mod scan_result;
mod try_match;

pub(crate) use self::{
    maybe_match::MaybeMatch,
    optional::Optional,
    required::Required,
    scan_error_kind::ScanErrorKind,
    scan_result::ScanResult,
    try_match::TryMatch,
};
