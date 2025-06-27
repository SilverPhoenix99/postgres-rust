pub(super) mod foundation;

mod acl;
mod all_or_var_name;
mod any_name;
mod attrs;
mod bare_col_label;
mod boolean_or_string;
mod col_id;
mod col_label;
mod const_numeric;
mod document_or_content;
mod expr;
mod expr_list;
mod expr_list_paren;
mod func_arg;
mod func_arg_expr;
mod func_name;
mod func_type;
mod function_with_argtypes;
mod generic_option;
mod generic_set_tail;
mod i32_literal_paren;
mod name_list;
mod non_reserved_word;
mod non_reserved_word_or_sconst;
mod operators;
mod opt_array_bounds;
mod opt_asc_desc;
mod opt_interval;
mod opt_nulls_order;
mod opt_precision;
mod opt_timezone;
mod opt_transaction;
mod opt_transaction_chain;
mod opt_type_modifiers;
mod opt_unique_null_treatment;
mod opt_varying;
mod paren_name_list;
mod privilege;
mod qualified_name;
mod role;
mod sign;
mod simple_typename;
mod sort_clause;
mod stmt;
mod stmtmulti;
mod string_or_null;
mod transaction_mode_list;
mod type_function_name;
mod typename;
mod var_name;
mod var_value;
mod window_specification;

// Entrypoint:
pub(super) use stmtmulti::stmtmulti;

#[allow(unused_imports)] // TODO: eventually remove
use self::{
    acl::{grantee_list, opt_drop_behavior, opt_grant_option, opt_granted_by},
    all_or_var_name::all_or_var_name,
    any_name::{any_name, any_name_list},
    attrs::attrs,
    bare_col_label::bare_col_label,
    boolean_or_string::{boolean_or_string, boolean_or_string_list},
    col_id::col_id,
    col_label::col_label,
    const_numeric::{i32_literal, signed_i32_literal, signed_number},
    document_or_content::document_or_content,
    expr_list::expr_list,
    expr_list_paren::expr_list_paren,
    func_arg::func_arg,
    func_arg_expr::{func_arg_expr, func_arg_list},
    func_name::func_name,
    func_type::func_type,
    function_with_argtypes::{function_with_argtypes, function_with_argtypes_list},
    generic_option::{generic_option, generic_options},
    generic_set_tail::generic_set_tail,
    i32_literal_paren::i32_literal_paren,
    name_list::name_list,
    non_reserved_word::non_reserved_word,
    non_reserved_word_or_sconst::non_reserved_word_or_sconst,
    operators::{any_operator, explicit_op, qual_all_op, qual_op, subquery_op},
    opt_array_bounds::opt_array_bounds,
    opt_asc_desc::opt_asc_desc,
    opt_interval::opt_interval,
    opt_nulls_order::opt_nulls_order,
    opt_precision::opt_precision,
    opt_timezone::opt_timezone,
    opt_transaction::opt_transaction,
    opt_transaction_chain::opt_transaction_chain,
    opt_type_modifiers::opt_type_modifiers,
    opt_unique_null_treatment::opt_unique_null_treatment,
    opt_varying::opt_varying,
    paren_name_list::paren_name_list,
    privilege::{privilege_list, privileges},
    qualified_name::{qualified_name, qualified_name_list},
    role::{role_id, role_list, role_spec},
    sign::sign,
    simple_typename::simple_typename,
    sort_clause::sort_clause,
    stmt::{begin_stmt, end_stmt, stmt},
    string_or_null::string_or_null,
    transaction_mode_list::transaction_mode_list,
    type_function_name::type_function_name,
    typename::typename,
    var_name::var_name,
    var_value::{var_list, var_value},
    window_specification::window_specification,
};
