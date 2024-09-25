pub struct BasicStringDecoder<'src> {
    source: &'src [u8],
    is_ident: bool
}

impl<'src> BasicStringDecoder<'src> {

    #[inline(always)]
    pub fn new(source: &'src [u8], is_ident: bool) -> BasicStringDecoder<'src> {
        BasicStringDecoder { source, is_ident }
    }

    pub fn decode(&self) -> Result<String, Utf8Error> {

        let src = std::str::from_utf8(self.source)?;

        let (quote, escape) = if self.is_ident {
            (r#"""#, r#""""#)
        }
        else {
            ("'", "''")
        };

        let string = src.to_string()
            .replace(escape, quote);

        Ok(string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_string() {
        let src = b"don''t do what Donny Dont does";
        let decoder = BasicStringDecoder::new(src, false);
        assert_eq!(
            Ok("don't do what Donny Dont does".to_string()),
            decoder.decode()
        )
    }
}

use std::str::Utf8Error;
