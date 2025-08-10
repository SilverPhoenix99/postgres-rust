pg_basics::reexport! {
    cast_expr,
    coalesce_expr,
    collation_for,
    current_schema,
    extract,
    greatest_expr,
    json,
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

pub(in crate::combinators) fn func_expr_common_subexpr(ctx: &mut ParserContext) -> scan::Result<SqlFunction> {

    /*
        | CAST '(' a_expr AS Typename ')'
        | COALESCE '(' expr_list ')'
        | COLLATION FOR '(' a_expr ')'
        | CURRENT_CATALOG
        | CURRENT_DATE
        | CURRENT_ROLE
        | CURRENT_SCHEMA
        | CURRENT_TIME ( '(' ICONST ')' )?
        | CURRENT_TIMESTAMP ( '(' ICONST ')' )?
        | CURRENT_USER
        | EXTRACT '(' extract_list ')'
        | GREATEST '(' expr_list ')'
        | JSON '(' ... ')'
        | JSON_EXISTS '(' ... ')'
        | JSON_OBJECT '(' ( json_object_args )? ')'
        | JSON_QUERY '(' ... ')'
        | JSON_SCALAR '(' a_expr ')'
        | JSON_SERIALIZE '(' ... ')'
        | JSON_VALUE '(' ... ')'
        | LEAST '(' expr_list ')'
        | LOCALTIME ( '(' ICONST ')' )?
        | LOCALTIMESTAMP ( '(' ICONST ')' )?
        | MERGE_ACTION '(' ')'
        | NORMALIZE '(' a_expr ( ',' unicode_normal_form )? ')'
        | NULLIF '(' a_expr ',' a_expr ')'
        | OVERLAY '(' ( overlay_args )? ')'
        | POSITION '(' b_expr IN b_expr ')'
        | SESSION_USER
        | SUBSTRING '(' ( substring_args )? ')'
        | SYSTEM_USER
        | TREAT '(' a_expr AS Typename ')'
        | TRIM '(' trim_args ')'
        | USER
        | XMLCONCAT '(' expr_list ')'
        | XMLELEMENT '(' ... ')'
        | XMLEXISTS '(' c_expr xmlexists_argument ')'
        | XMLFOREST '(' xml_attribute_list ')'
        | XMLPARSE '(' ... ')'
        | XMLPI '(' ... ')'
        | XMLROOT '(' ... ')'
        | XMLSERIALIZE '(' ... ')'
    */

    // Peeking 2 tokens to prevent conflicts with `func_application` and `prefixed_expr_const`:
    match ctx.stream_mut().peek2() {
        Ok((K(Coalesce), Op(OpenParenthesis))) => return coalesce_expr(ctx),
        Ok((K(Collation), K(For))) => return collation_for(ctx),
        Ok((K(Extract), Op(OpenParenthesis))) => return extract(ctx).map(From::from),
        Ok((K(Greatest), Op(OpenParenthesis))) => return greatest_expr(ctx),
        Ok((K(Json), Op(OpenParenthesis))) => return json(ctx).map(From::from),
        Ok((K(JsonExists), Op(OpenParenthesis))) => return json_exists_expr(ctx).map(From::from),
        Ok((K(JsonObject), Op(OpenParenthesis))) => return json_object(ctx).map(From::from),
        Ok((K(JsonQuery), Op(OpenParenthesis))) => return json_query_expr(ctx).map(From::from),
        Ok((K(JsonScalar), Op(OpenParenthesis))) => return json_scalar(ctx),
        Ok((K(JsonSerialize), Op(OpenParenthesis))) => return json_serialize_expr(ctx).map(From::from),
        Ok((K(JsonValue), Op(OpenParenthesis))) => return json_value_func(ctx).map(From::from),
        Ok((K(Least), Op(OpenParenthesis))) => return least_expr(ctx),
        Ok((K(MergeAction), Op(OpenParenthesis))) => return merge_action(ctx),
        Ok((K(Normalize), Op(OpenParenthesis))) => return normalize(ctx).map(From::from),
        Ok((K(Nullif), Op(OpenParenthesis))) => return nullif_expr(ctx),
        Ok((K(Overlay), Op(OpenParenthesis))) => return overlay(ctx).map(From::from),
        Ok((K(Position), Op(OpenParenthesis))) => return position(ctx).map(From::from),
        Ok((K(Substring), Op(OpenParenthesis))) => return substring(ctx).map(From::from),
        Ok((K(Treat), Op(OpenParenthesis))) => return treat_expr(ctx),
        Ok((K(Trim), Op(OpenParenthesis))) => return trim(ctx).map(From::from),
        Ok((K(Xmlconcat), Op(OpenParenthesis))) => return xml_concat(ctx),
        Ok((K(Xmlelement), Op(OpenParenthesis))) => return xml_element(ctx).map(From::from),
        Ok((K(Xmlexists), Op(OpenParenthesis))) => return xml_exists(ctx).map(From::from),
        Ok((K(Xmlforest), Op(OpenParenthesis))) => return xml_forest(ctx),
        Ok((K(Xmlparse), Op(OpenParenthesis))) => return xml_parse(ctx).map(From::from),
        Ok((K(Xmlpi), Op(OpenParenthesis))) => return xml_processing_instruction(ctx).map(From::from),
        Ok((K(Xmlroot), Op(OpenParenthesis))) => return xml_root(ctx).map(From::from),
        Ok((K(Xmlserialize), Op(OpenParenthesis))) => return xml_serialize(ctx).map(From::from),
        _ => {}
    };

    alt!(
        Kw::CurrentCatalog.map(|_| CurrentCatalog),
        cast_expr.map(From::from),
        current_schema,
        role,
        time,
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    #[allow(unused_imports)]
    use scan::Error::NoMatch;
    use test_case::test_case;
    use test_case::test_matrix;

    #[test_case("current_catalog" => Ok(CurrentCatalog))]
    fn test_func_expr_common_subexpr(source: &str) -> scan::Result<SqlFunction> {
        test_parser!(source, func_expr_common_subexpr)
    }

    // These only quickly check that statements aren't missing
    #[test_matrix(
        [
            "cast ('1' as int)",
            "coalesce(1)",
            "collation for(1)",
            "current_date",
            "current_schema",
            "extract(month from 1)",
            "greatest(1)",
            "json_exists('{}', 'foo')",
            "json_object()",
            "json_query('{}', 'foo')",
            "json_scalar(1)",
            "json_serialize(1)",
            "json_value('{}', 'foo')",
            "json('{}')",
            "least(1)",
            "merge_action()",
            "normalize('foo')",
            "nullif(1, 2)",
            "overlay('foo')",
            "position('f' in 'foo')",
            "substring('foo')",
            "treat(1 as int)",
            "trim('foo')",
            "user",
            "xmlconcat('foo')",
            "xmlelement(name foo)",
            "xmlexists('foo' passing 'bar')",
            "xmlforest('foo')",
            "xmlparse(document 'foo')",
            "xmlpi(name foo)",
            "xmlroot('foo', version no value)",
            "xmlserialize(document '123' as int)",
        ]
        => matches Ok(_)
    )]
    fn test_func_expr_common_subexpr_ok(source: &str) -> scan::Result<SqlFunction> {
        test_parser!(source, func_expr_common_subexpr)
    }

    #[test_matrix(
        [
            "coalesce 1",
            "coalesce",
            "collation 1",
            "collation",
            "collation() -- empty",
            "extract 1",
            "extract",
            "greatest 1",
            "greatest",
            "json 1",
            "json_exists 1",
            "json_exists",
            "json_object 1",
            "json_object",
            "json_query 1",
            "json_query",
            "json_scalar 1",
            "json_scalar",
            "json_serialize 1",
            "json_serialize",
            "json_value 1",
            "json_value",
            "json",
            "least 1",
            "least",
            "merge_action 1",
            "merge_action",
            "normalize 1",
            "normalize",
            "nullif 1",
            "nullif",
            "overlay 1",
            "overlay",
            "position 1",
            "position",
            "substring 1",
            "substring",
            "treat 1",
            "treat",
            "trim 1",
            "trim",
            "xmlconcat 1",
            "xmlconcat",
            "xmlelement 1",
            "xmlelement",
            "xmlexists 1",
            "xmlexists",
            "xmlforest 1",
            "xmlforest",
            "xmlparse 1",
            "xmlparse",
            "xmlpi 1",
            "xmlpi",
            "xmlroot 1",
            "xmlroot",
            "xmlserialize 1",
            "xmlserialize",
        ]
        => matches Err(NoMatch(_))
    )]
    fn test_func_expr_common_subexpr_no_match(source: &str) -> scan::Result<SqlFunction> {
        test_parser!(source, func_expr_common_subexpr)
    }
}

use pg_ast::SqlFunction;
use pg_ast::SqlFunction::CurrentCatalog;
use pg_combinators::alt;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Coalesce;
use pg_lexer::Keyword::Collation;
use pg_lexer::Keyword::Extract;
use pg_lexer::Keyword::For;
use pg_lexer::Keyword::Greatest;
use pg_lexer::Keyword::Json;
use pg_lexer::Keyword::JsonExists;
use pg_lexer::Keyword::JsonObject;
use pg_lexer::Keyword::JsonQuery;
use pg_lexer::Keyword::JsonScalar;
use pg_lexer::Keyword::JsonSerialize;
use pg_lexer::Keyword::JsonValue;
use pg_lexer::Keyword::Least;
use pg_lexer::Keyword::MergeAction;
use pg_lexer::Keyword::Normalize;
use pg_lexer::Keyword::Nullif;
use pg_lexer::Keyword::Overlay;
use pg_lexer::Keyword::Position;
use pg_lexer::Keyword::Substring;
use pg_lexer::Keyword::Treat;
use pg_lexer::Keyword::Trim;
use pg_lexer::Keyword::Xmlconcat;
use pg_lexer::Keyword::Xmlelement;
use pg_lexer::Keyword::Xmlexists;
use pg_lexer::Keyword::Xmlforest;
use pg_lexer::Keyword::Xmlparse;
use pg_lexer::Keyword::Xmlpi;
use pg_lexer::Keyword::Xmlroot;
use pg_lexer::Keyword::Xmlserialize;
use pg_lexer::OperatorKind::OpenParenthesis;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenValue::Keyword as K;
use pg_parser_core::stream::TokenValue::Operator as Op;
