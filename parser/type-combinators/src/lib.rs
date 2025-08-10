pg_basics::reexport! { pub
    simple_typename,
    typename,
}

pg_basics::reexport! {
    array_bounds,
    bit,
    character,
    generic_type,
    interval_type,
    numeric,
    time,
    timestamp,
    type_modifiers,
    with_timezone,
}

#[cfg(test)]
mod tests {

    pub fn expr_list(ctx: &mut ParserContext) -> scan::Result<Vec<ExprNode>> {
        many!(sep = Comma,
            alt!(
                i32_literal.map(IntegerConst),
                string.map(StringConst)
            )
        ).parse(ctx)
    }

    use pg_ast::ExprNode;
    use pg_ast::ExprNode::IntegerConst;
    use pg_ast::ExprNode::StringConst;
    use pg_combinators::alt;
    use pg_combinators::many;
    use pg_combinators::string;
    use pg_combinators::Combinator;
    use pg_combinators::ParserContext;
    use pg_lexer::OperatorKind::Comma;
    use pg_parser_core::scan;
    use pg_sink_combinators::i32_literal;
}
