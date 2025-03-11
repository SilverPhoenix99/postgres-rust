pub mod ast_node;
mod acl_parsers;
mod boolean_or_string;
mod combinators;
mod config;
mod const_numeric_parsers;
mod error;
mod expr_parsers;
mod func_arg;
mod func_name;
mod func_type;
mod function_with_argtypes;
mod generic_reset;
mod generic_set_tail;
mod located_combinator;
mod op_parsers;
mod opt_array_bounds;
mod opt_interval;
mod opt_precision;
mod opt_transaction;
mod opt_transaction_chain;
mod opt_unique_null_treatment;
mod privilege_parsers;
mod result;
mod role_parsers;
mod simple_typename;
mod stmt;
mod stmtmulti;
mod token_stream;
mod token_value;
mod transaction_mode_list;
mod typename;
mod uescape_escape;
mod var_value;
mod warning;

pub use self::{
    config::ParserConfig,
    error::{ParserError, ParserErrorKind},
    warning::ParserWarningKind,
};

pub(crate) type ParseResult<T> = Result<T, ParserError>;

pub struct ParserResult {
    pub result: ParseResult<Vec<RawStmt>>,
    pub warnings: Vec<Located<ParserWarningKind>>,
}

pub struct Parser<'src> {
    buffer: TokenStream<'src>,
}

impl<'src> Parser<'src> {

    pub fn new(source: &'src str, config: ParserConfig) -> Self {
        Self {
            buffer: TokenStream::new(source, config)
        }
    }

    /// Not reentrant (yet)!
    /// The TokenStream state is changed.
    pub fn parse(&mut self) -> ParserResult {

        let mut result = stmtmulti()
            .parse(&mut self.buffer)
            .required();

        // If it's not Eof, then something didn't match properly.
        // Discard the previous result, and mark the current location as a Syntax error.
        if !self.buffer.eof() {
            let loc = self.buffer.current_location();
            result = Err(syntax_err(loc));
        }

        ParserResult {
            result,
            warnings: mem::take(self.buffer.warnings()),
        }
    }
}

/// Post-condition: Vec **May** be empty
fn opt_type_modifiers() -> impl Combinator<Output = TypeModifiers> {

    /*
        ( '(' expr_list ')' )?
    */

    expr_list_paren()
        .optional()
        .map(Option::unwrap_or_default)
}

fn opt_varying() -> impl Combinator<Output = bool> {

    /*
        ( VARYING )?
    */

    Varying
        .optional()
        .map(|varying| varying.is_some())
}

fn opt_timezone() -> impl Combinator<Output = bool> {

    /*
        ( (WITH | WITHOUT) TIME ZONE )?
    */

    match_first!(
        With.map(|_| true),
        Without.map(|_| false)
    )
        .and_left(sequence!(Time, Zone).skip())
        .optional()
        .map(|tz| tz.unwrap_or(false))
}

/// Post-condition: Vec is **Not** empty
fn expr_list_paren() -> impl Combinator<Output = Vec<ExprNode>> {

    /*
        '(' expr_list ')'
    */

    between(
        OpenParenthesis,
        expr_list(),
        CloseParenthesis
    )
}

/// Post-condition: Vec is **Not** empty
fn expr_list() -> impl Combinator<Output = Vec<ExprNode>> {

    /*
        a_expr ( ',' a_expr )*
    */

    many_sep(Comma, a_expr())
}

/// Post-condition: Vec is **Not** empty
///
/// Alias: `opt_column_list`
fn opt_name_list() -> impl Combinator<Output = Vec<Str>> {

    /*
        '(' name_list ')'
    */

    between(OpenParenthesis, name_list(), CloseParenthesis)
}

/// Post-condition: Vec is **Not** empty
fn var_name() -> impl Combinator<Output = QualifiedName> {

    /*
        col_id ( '.' col_id )*
    */

    many_sep(Dot, col_id())
}

/// Post-condition: Vec is **Not** empty
///
/// Alias: `columnList`
fn name_list() -> impl Combinator<Output = Vec<Str>> {

    /*
        col_id ( ',' col_id )*
    */

    many_sep(Comma, col_id())
}

/// Post-condition: Vec is **Not** empty
fn col_id_list(separator: OperatorKind) -> impl Combinator<Output = QualifiedName> {

    /*
        col_id ( <separator> col_id )*
    */

    many_sep(separator, col_id())
}

/// Post-condition: Vec is **Not** empty
fn qualified_name_list() -> impl Combinator<Output = Vec<RangeVar>> {

    /*
        qualified_name ( ',' qualified_name )*
    */

    many_sep(Comma, qualified_name())
}

fn qualified_name() -> impl Combinator<Output = RangeVar> {

    /*
        col_id attrs{1,3}
    */

    located(any_name())
        .map_result(|result| {
            let (mut qn, loc) = result?;

            match qn.as_mut_slice() {
                [relation] => {
                    let relation = mem::take(relation);
                    Ok(RangeVar::new(relation))
                },
                [schema, relation] => {
                    let schema = mem::take(schema);
                    let relation = mem::take(relation);
                    Ok(
                        RangeVar::new(relation)
                            .with_schema(schema)
                    )
                },
                [catalog, schema, relation] => {
                    let catalog = mem::take(catalog);
                    let schema = mem::take(schema);
                    let relation = mem::take(relation);
                    Ok(
                        RangeVar::new(relation)
                            .with_schema(schema)
                            .with_catalog(catalog)
                    )
                },
                _ => {
                    let err = ParserError::new(ImproperQualifiedName(NameList(qn)), loc);
                    Err(err.into())
                }
            }
        })
}

/// Post-condition: Vec is **Not** empty
fn any_name_list() -> impl Combinator<Output=Vec<QualifiedName>> {

    /*
        any_name ( ',' any_name )*
    */

    many_sep(Comma, any_name())
}

/// Post-condition: Vec is **Not** empty
///
/// Alias: `handler_name`
fn any_name() -> impl Combinator<Output = QualifiedName> {

    /*
        col_id attrs
    */

    attrs(col_id())
}

/// Post-condition: Vec is **Not** empty
fn attrs<F>(prefix: F) -> impl Combinator<Output = QualifiedName>
where
    F: Combinator<Output = Str>
{
    /*
        prefix ( '.' col_label )*
    */

    many_pre(
        prefix,
        Dot.and_right(col_label())
    )
}

/// Aliases:
/// * `ColLabel`
/// * `attr_name`
fn col_label() -> impl Combinator<Output = Str> {
    match_first!(
        identifier().map(From::from),
        keyword_if(|_| true).map(From::from)
    )
}

/// Aliases:
/// * `ColId`
/// * `name`
fn col_id() -> impl Combinator<Output = Str> {
    match_first!(
        identifier().map(From::from),
        Unreserved.map(From::from),
        ColumnName.map(From::from),
    )
}

fn type_function_name() -> impl Combinator<Output = Str> {
    match_first!(
        identifier().map(From::from),
        Unreserved.map(From::from),
        TypeFuncName.map(From::from),
    )
}

/// Alias: `NonReservedWord`
fn non_reserved_word() -> impl Combinator<Output = Str> {
    match_first!(
        identifier().map(From::from),
        Unreserved.map(From::from),
        ColumnName.map(From::from),
        TypeFuncName.map(From::from),
    )
}

/// Alias: `BareColLabel`
fn bare_col_label() -> impl Combinator<Output = Str> {
    match_first!(
        identifier().map(From::from),
        keyword_if(|kw| kw.details().bare()).map(From::from)
    )
}

/// Production: `'(' ICONST ')'`
fn i32_literal_paren() -> impl Combinator<Output = i32> {

    between(OpenParenthesis, integer(), CloseParenthesis)
        .map(From::from)
}

/// '+' | '-'
fn sign() -> impl Combinator<Output = OperatorKind> {
    use OperatorKind::{Minus, Plus};
    operator_if(|op| matches!(op, Minus | Plus))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::OperatorKind::Dot;
    use postgres_basics::guc::BackslashQuote;

    pub(in crate::parser) static DEFAULT_CONFIG: ParserConfig = ParserConfig::new(true, BackslashQuote::SafeEncoding);

    #[test]
    /// All these methods are similar, so no point in repeating tests:
    /// * test_var_name
    /// * test_name_list
    fn test_col_id_list() {
        let mut stream = TokenStream::new("test.qualified.name", DEFAULT_CONFIG);
        let expected = vec![
            "test".into(),
            "qualified".into(),
            "name".into()
        ];

        assert_eq!(Ok(expected), col_id_list(Dot).parse(&mut stream));
    }

    #[test]
    fn test_expr_list() {
        let mut stream = TokenStream::new("1, 2, 3", DEFAULT_CONFIG);

        let expected = vec![
            ExprNode::IntegerConst(1),
            ExprNode::IntegerConst(2),
            ExprNode::IntegerConst(3),
        ];

        assert_eq!(Ok(expected), expr_list().parse(&mut stream));
    }

    #[test]
    fn test_any_name_list() {
        let source = "qual.name_, second.qualif";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = vec![
            vec!["qual".into(), "name_".into()],
            vec!["second".into(), "qualif".into()]
        ];

        assert_eq!(Ok(expected), any_name_list().parse(&mut stream));
    }

    #[test]
    fn test_any_name() {
        let source = "some_.qualified_.name_";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = any_name().parse(&mut stream);

        let expected: QualifiedName = vec![
            "some_".into(),
            "qualified_".into(),
            "name_".into()
        ];

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_qualified_name() {
        let source = "some_catalog.some_schema.some_relation";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = RangeVar::new("some_relation".into())
            .with_schema("some_schema".into())
            .with_catalog("some_catalog".into());

        assert_eq!(Ok(expected), qualified_name().parse(&mut stream));
    }

    #[test]
    fn test_qualified_name_list() {
        let source = "relation_,schema_.relation_, catalog_.schema_.relation_";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = vec![
            RangeVar::new("relation_".into()),
            RangeVar::new("relation_".into())
                .with_schema("schema_".into()),
            RangeVar::new("relation_".into())
                .with_schema("schema_".into())
                .with_catalog("catalog_".into())
        ];

        assert_eq!(Ok(expected), qualified_name_list().parse(&mut stream));
    }

    #[test]
    fn test_attrs() {
        let source = ".qualified_.name_";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let parser = combinators::parser(|_| Ok("*some*".into()));
        let actual = attrs(parser).parse(&mut stream);

        let expected: QualifiedName = vec![
            "*some*".into(),
            "qualified_".into(),
            "name_".into()
        ];

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_i32_literal_paren() {
        let mut stream = TokenStream::new(" (123 )", DEFAULT_CONFIG);
        assert_eq!(Ok(123), i32_literal_paren().parse(&mut stream));
    }

    #[test]
    fn test_col_id() {
        let source = "cascaded xxyyzz coalesce";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("cascaded".into()), col_id().parse(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), col_id().parse(&mut stream));
        assert_eq!(Ok("coalesce".into()), col_id().parse(&mut stream));
    }

    #[test]
    fn test_type_function_name() {
        let source = "before xxyyzz collation";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("before".into()), type_function_name().parse(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), type_function_name().parse(&mut stream));
        assert_eq!(Ok("collation".into()), type_function_name().parse(&mut stream));
    }

    #[test]
    fn test_non_reserved_word() {
        let source = "breadth xxyyzz boolean authorization";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("breadth".into()), non_reserved_word().parse(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), non_reserved_word().parse(&mut stream));
        assert_eq!(Ok("boolean".into()), non_reserved_word().parse(&mut stream));
        assert_eq!(Ok("authorization".into()), non_reserved_word().parse(&mut stream));
    }

    #[test]
    fn test_col_label() {
        let source = "sequence xxyyzz character";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("sequence".into()), col_label().parse(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), col_label().parse(&mut stream));
        assert_eq!(Ok("character".into()), col_label().parse(&mut stream));
    }

    #[test]
    fn test_bare_col_label() {
        let source = "sequence xxyyzz";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("sequence".into()), bare_col_label().parse(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), bare_col_label().parse(&mut stream));
    }
}

use crate::lexer::Keyword::Time;
use crate::lexer::Keyword::Varying;
use crate::lexer::Keyword::With;
use crate::lexer::Keyword::Without;
use crate::lexer::Keyword::Zone;
use crate::lexer::KeywordCategory::ColumnName;
use crate::lexer::KeywordCategory::TypeFuncName;
use crate::lexer::KeywordCategory::Unreserved;
use crate::lexer::OperatorKind;
use crate::lexer::OperatorKind::CloseParenthesis;
use crate::lexer::OperatorKind::Comma;
use crate::lexer::OperatorKind::Dot;
use crate::lexer::OperatorKind::OpenParenthesis;
use crate::parser::ast_node::ExprNode;
use crate::parser::ast_node::QualifiedName;
use crate::parser::ast_node::RangeVar;
use crate::parser::ast_node::RawStmt;
use crate::parser::ast_node::TypeModifiers;
use crate::parser::combinators::between;
use crate::parser::combinators::identifier;
use crate::parser::combinators::integer;
use crate::parser::combinators::keyword_if;
use crate::parser::combinators::many_pre;
use crate::parser::combinators::many_sep;
use crate::parser::combinators::match_first;
use crate::parser::combinators::operator_if;
use crate::parser::combinators::sequence;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::error::syntax_err;
use crate::parser::error::NameList;
use crate::parser::error::ParserErrorKind::ImproperQualifiedName;
use crate::parser::expr_parsers::a_expr;
use crate::parser::located_combinator::located;
use crate::parser::opt_transaction::opt_transaction;
use crate::parser::opt_transaction_chain::opt_transaction_chain;
use crate::parser::result::Required;
use crate::parser::stmtmulti::stmtmulti;
use crate::parser::token_stream::TokenStream;
use postgres_basics::Located;
use postgres_basics::Str;
use std::mem;
