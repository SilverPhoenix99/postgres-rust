#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct JsonFormat {
    kind: Option<JsonFormatKind>,
    encoding: Option<JsonEncoding>,
}

impl JsonFormat {
    pub fn new(kind: Option<JsonFormatKind>, encoding: Option<JsonEncoding>) -> Self {
        Self { kind, encoding }
    }

    pub fn text() -> Self {
        Self::new(Some(Text), None)
    }

    pub fn kind(&self) -> Option<JsonFormatKind> {
        self.kind
    }

    pub fn encoding(&self) -> Option<JsonEncoding> {
        self.encoding
    }

    pub fn with_encoding(mut self, encoding: JsonEncoding) -> Self {
        self.encoding = Some(encoding);
        self
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
