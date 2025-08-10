type FnCombi<T> = fn(&mut ParserContext) -> scan::Result<T>;

#[derive(derive_more::Debug)]
pub struct ParserContext<'src> {
    stream: TokenStream<'src>,

    #[debug(skip)]
    expr_list: FnCombi<Vec<ExprNode>>,
}

impl<'src> ParserContext<'src> {

    pub fn new<T>(
        stream: T,
        expr_list: FnCombi<Vec<ExprNode>>,
    ) -> Self
    where
        T: Into<TokenStream<'src>>,
    {
        Self {
            stream: stream.into(),
            expr_list,
        }
    }

    pub fn stream(&self) -> &TokenStream<'src> {
        &self.stream
    }

    pub fn stream_mut(&mut self) -> &mut TokenStream<'src> {
        &mut self.stream
    }

    pub fn expr_list(&self) -> FnCombi<Vec<ExprNode>> {
        self.expr_list
    }
}

impl<'src> From<&'src str> for ParserContext<'src> {
    fn from(value: &'src str) -> Self {
        let stream = TokenStream::from(value);
        ParserContext::new(stream, no_op)
    }
}

fn no_op<T>(_: &mut ParserContext) -> scan::Result<T> {
    panic!("Called a no-op function");
}

use pg_ast::ExprNode;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
