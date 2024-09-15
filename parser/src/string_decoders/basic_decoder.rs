use std::str::Utf8Error;

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

        let src = self.source.to_vec();

        let (quote, escape) = if self.is_ident {
            (r#"""#, r#""""#)
        }
        else {
            ("'", "''")
        };

        let string = String::from_utf8(src)
            .map_err(|e| e.utf8_error())?
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
