mod attrs;
mod col_id;
mod col_label;

#[allow(unused_imports)] // TODO: eventually remove
pub(in crate::combinators) use self::{
    attrs::attrs,
    col_id::col_id,
    col_label::col_label,
};
