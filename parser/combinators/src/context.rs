#[derive(derive_more::Debug)]
pub struct ParserContext<'src> {
    stream: TokenStream<'src>,
}

impl<'src> ParserContext<'src> {

    pub fn new<T>(stream: T) -> Self
    where
        T: Into<TokenStream<'src>>,
    {
        Self {
            stream: stream.into()
        }
    }

    pub fn stream_mut(&mut self) -> &mut TokenStream<'src> {
        &mut self.stream
    }
}

impl<'src> From<&'src str> for ParserContext<'src> {
    fn from(value: &'src str) -> Self {
        let stream = TokenStream::from(value);
        ParserContext::new(stream)
    }
}

use pg_parser_core::stream::TokenStream;
