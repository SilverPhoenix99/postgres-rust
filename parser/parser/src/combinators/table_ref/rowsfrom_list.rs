pub(super) fn rowsfrom_list(stream: &mut TokenStream) -> scan::Result<Vec<RangeFunction>> {

    /*
        rowsfrom_item ( ',' rowsfrom_item )*
    */

    many!(sep = Comma, rowsfrom_item).parse(stream)
}

fn rowsfrom_item(stream: &mut TokenStream) -> scan::Result<RangeFunction> {

    /*
        func_expr_windowless ( col_def_list )?
    */

    let (func_call, cols) = seq!(func_expr_windowless, col_def_list.optional())
        .parse(stream)?;

    let mut range_func = RangeFunction::new(func_call);
    range_func.set_column_definitions(cols);

    Ok(range_func)
}

/// Alias: `opt_col_def_list`
fn col_def_list(stream: &mut TokenStream) -> scan::Result<Vec<SimpleColumnDefinition>> {

    /*
        AS '(' table_func_element_list ')'
    */

    let (_, cols) = seq!(As, paren!(table_func_element_list))
        .parse(stream)?;

    Ok(cols)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::RangeFunction;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::IntegerConst,
        pg_ast::FuncArgsKind,
        pg_ast::FuncCall,
        pg_ast::NamedValue,
        pg_ast::TypeName::Int4,
        pg_basics::Location,
    };

    #[test_case("foo(*)" => Ok(
        RangeFunction::new(
            FuncCall::new(
                vec!["foo".into()],
                FuncArgsKind::Wildcard { order_within_group: None }
            ).into()
        )
    ))]
    #[test_case("bar(1) as (a int)" => Ok(
        RangeFunction::new(
            FuncCall::new(
                vec!["bar".into()],
                FuncArgsKind::All {
                    args: vec![
                        (
                            NamedValue::unnamed(IntegerConst(1)),
                            Location::new(4..5, 1, 5)
                        )
                    ],
                    order: None
                }
            ).into()
        )
        .with_column_definitions(vec![
            SimpleColumnDefinition::new("a", Int4)
        ])
    ))]
    fn test_rowsfrom_item(source: &str) -> scan::Result<RangeFunction> {
        test_parser!(source, rowsfrom_item)
    }

    #[test_case("as (foo int)" => matches Ok(_))]
    fn test_col_def_list(source: &str) -> scan::Result<Vec<SimpleColumnDefinition>> {
        test_parser!(source, col_def_list)
    }
}

use crate::combinators::foundation::many;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::func_expr_windowless;
use crate::combinators::table_func_element_list;
use pg_ast::RangeFunction;
use pg_ast::SimpleColumnDefinition;
use pg_combinators::Combinator;
use pg_lexer::Keyword::As;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
