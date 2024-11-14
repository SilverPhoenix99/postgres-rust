pub struct BasicStringDecoder<'src> {
    source: &'src str,
    is_ident: bool
}

impl<'src> BasicStringDecoder<'src> {

    #[inline(always)]
    pub fn new(source: &'src str, is_ident: bool) -> BasicStringDecoder<'src> {
        BasicStringDecoder { source, is_ident }
    }

    pub fn decode(&self) -> Box<str> {

        let (quote, escape) = if self.is_ident {
            (r#"""#, r#""""#)
        }
        else {
            ("'", "''")
        };

        self.source.replace(escape, quote)
            .into_boxed_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_string() {
        let src = "don''t do what Donny Dont does";
        let decoder = BasicStringDecoder::new(src, false);
        assert_eq!("don't do what Donny Dont does", decoder.decode().as_ref())
    }
}
