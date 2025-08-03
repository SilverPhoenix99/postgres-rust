pg_basics::reexport! {
    cast_expr,
    coalesce_expr,
    collation_for,
    current_schema,
    extract,
    greatest_expr,
    json_exists_expr,
    json_object,
    json_query_expr,
    json_scalar,
    json_serialize_expr,
    json_value_func,
    least_expr,
    merge_action,
    normalize,
    nullif_expr,
    overlay,
    position,
    role,
    substring,
    time,
    treat_expr,
    trim,
    xml_attribute_list,
    xml_concat,
    xml_element,
    xml_exists,
    xml_forest,
    xml_parse,
    xml_processing_instruction,
    xml_root,
    xml_serialize,
}

pub(super) fn func_expr_common_subexpr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
          CAST '(' a_expr AS Typename ')'
        | COALESCE '(' expr_list ')'
        | COLLATION FOR '(' a_expr ')'
        | CURRENT_CATALOG
        | CURRENT_SCHEMA
        | EXTRACT '(' extract_list ')'
        | GREATEST '(' expr_list ')'
        | LEAST '(' expr_list ')'
        | MERGE_ACTION '(' ')'
        | NORMALIZE '(' a_expr ( ',' unicode_normal_form )? ')'
        | NULLIF '(' a_expr ',' a_expr ')'
        | OVERLAY '(' ( overlay_args )? ')'
        | POSITION '(' b_expr IN b_expr ')'
        | SUBSTRING '(' ( substring_args )? ')'
        | TREAT '(' a_expr AS Typename ')'
        | TRIM '(' trim_args ')'
        | CURRENT_DATE
        | CURRENT_TIME ( '(' ICONST ')' )?
        | CURRENT_TIMESTAMP ( '(' ICONST ')' )?
        | LOCALTIME ( '(' ICONST ')' )?
        | LOCALTIMESTAMP ( '(' ICONST ')' )?
        | CURRENT_ROLE
        | CURRENT_USER
        | SESSION_USER
        | SYSTEM_USER
        | USER
        | JSON_EXISTS '(' ... ')'
        | JSON_OBJECT '(' ( json_object_args )? ')'
        | JSON_QUERY '(' ... ')'
        | JSON_SCALAR '(' a_expr ')'
        | JSON_SERIALIZE '(' json_value_expr ( json_returning_clause )? ')'
        | JSON_VALUE '(' ... ')'
        | XMLCONCAT '(' expr_list ')'
        | XMLELEMENT '(' ... ')'
        | XMLEXISTS '(' c_expr xmlexists_argument ')'
        | XMLFOREST '(' xml_attribute_list ')'
        | XMLPARSE '(' ... ')'
        | XMLPI '(' ... ')'
        | XMLROOT '(' ... ')'
        | XMLSERIALIZE '(' ... ')'
    */

    alt!(
        Kw::CurrentCatalog.map(|_| CurrentCatalog),
        cast_expr.map(From::from),
        coalesce_expr,
        collation_for,
        current_schema,
        extract.map(From::from),
        greatest_expr,
        json_common_subexpr,
        least_expr,
        merge_action,
        normalize.map(From::from),
        nullif_expr,
        overlay.map(From::from),
        position.map(From::from),
        role,
        substring.map(From::from),
        time,
        treat_expr,
        trim.map(From::from),
        xml_common_subexpr,
    ).parse(stream)
}

fn json_common_subexpr(stream: &mut TokenStream) -> scan::Result<ExprNode> {
    alt!(
        json_exists_expr.map(From::from),
        json_object.map(From::from),
        json_query_expr.map(From::from),
        json_scalar,
        json_serialize_expr.map(From::from),
        json_value_func.map(From::from),
    ).parse(stream)
}

fn xml_common_subexpr(stream: &mut TokenStream) -> scan::Result<ExprNode> {
    alt!(
        xml_concat,
        xml_element.map(From::from),
        xml_exists.map(From::from),
        xml_forest,
        xml_parse.map(From::from),
        xml_processing_instruction.map(From::from),
        xml_root.map(From::from),
        xml_serialize.map(From::from),
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("current_catalog" => Ok(CurrentCatalog))]
    // These only quickly check that statements aren't missing:
    #[test_case("cast ('1' as int)" => matches Ok(_))]
    #[test_case("coalesce(1)" => matches Ok(_))]
    #[test_case("collation for(1)" => matches Ok(_))]
    #[test_case("current_date" => matches Ok(_))]
    #[test_case("current_schema" => matches Ok(_))]
    #[test_case("extract(month from 1)" => matches Ok(_))]
    #[test_case("greatest(1)" => matches Ok(_))]
    #[test_case("json_exists('{}', 'foo')" => matches Ok(_))]
    #[test_case("json_object()" => matches Ok(_))]
    #[test_case("json_query('{}', 'foo')" => matches Ok(_))]
    #[test_case("json_scalar(1)" => matches Ok(_))]
    #[test_case("json_serialize(1)" => matches Ok(_))]
    #[test_case("json_value('{}', 'foo')" => matches Ok(_))]
    #[test_case("least(1)" => matches Ok(_))]
    #[test_case("merge_action()" => matches Ok(_))]
    #[test_case("normalize('foo')" => matches Ok(_))]
    #[test_case("nullif(1, 2)" => matches Ok(_))]
    #[test_case("overlay('foo')" => matches Ok(_))]
    #[test_case("position('f' in 'foo')" => matches Ok(_))]
    #[test_case("substring('foo')" => matches Ok(_))]
    #[test_case("treat(1 as int)" => matches Ok(_))]
    #[test_case("trim('foo')" => matches Ok(_))]
    #[test_case("user" => matches Ok(_))]
    #[test_case("xmlconcat('foo')" => matches Ok(_))]
    #[test_case("xmlelement(name foo)" => matches Ok(_))]
    #[test_case("xmlexists('foo' passing 'bar')" => matches Ok(_))]
    #[test_case("xmlforest('foo')" => matches Ok(_))]
    #[test_case("xmlparse(document 'foo')" => matches Ok(_))]
    #[test_case("xmlpi(name foo)" => matches Ok(_))]
    #[test_case("xmlroot('foo', version no value)" => matches Ok(_))]
    #[test_case("xmlserialize(document '123' as int)" => matches Ok(_))]
    fn test_func_expr_common_subexpr(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, func_expr_common_subexpr)
    }
}

use crate::combinators::foundation::alt;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
use pg_ast::ExprNode::CurrentCatalog;
use pg_lexer::Keyword as Kw;
