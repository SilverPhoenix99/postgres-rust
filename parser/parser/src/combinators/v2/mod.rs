mod all_or_var_name;
mod any_name;
mod attrs;
mod col_id;
mod col_label;
mod var_name;

#[allow(unused_imports)] // TODO: eventually remove
pub(in crate::combinators) use self::{
    all_or_var_name::all_or_var_name,
    any_name::{any_name, any_name_list},
    attrs::attrs,
    col_id::col_id,
    col_label::col_label,
    var_name::var_name,
};
