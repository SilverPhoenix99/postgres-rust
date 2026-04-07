#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct JsonFormat {
    kind: Option<JsonFormatKind>,
    encoding: Option<JsonEncoding>,
}

impl JsonFormat {

    pub fn text() -> Self {
        Self {
            kind: Some(Text),
            encoding: None
        }
    }

    pub fn set_kind(&mut self, kind: Option<JsonFormatKind>) -> &mut Self {
        self.kind = kind;
        self
    }

    pub fn with_kind(mut self, kind: JsonFormatKind) -> Self {
        self.kind = Some(kind);
        self
    }

    pub fn kind(&self) -> Option<JsonFormatKind> {
        self.kind
    }

    pub fn set_encoding(&mut self, encoding: Option<JsonEncoding>) -> &mut Self {
        self.encoding = encoding;
        self
    }

    pub fn with_encoding(mut self, encoding: JsonEncoding) -> Self {
        self.encoding = Some(encoding);
        self
    }

    pub fn encoding(&self) -> Option<JsonEncoding> {
        self.encoding
    }
}

/// Alias: `JsonFormatType`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonFormatKind {
    /// `JSON`
    Text,
    /// `JSONB`
    Binary
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonEncoding {
    UTF8,
    UTF16,
    UTF32,
}

use JsonFormatKind::Text;
