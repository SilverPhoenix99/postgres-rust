#[derive(Debug, Clone, Eq, PartialEq)]
pub struct XmlElement {
    name: Str,
    attributes: Option<Vec<NamedValue>>,
    content: Option<Vec<ExprNode>>,
}

impl XmlElement {
    pub fn new(name: Str) -> Self {
        Self { name, attributes: None, content: None }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_attributes(&mut self, attributes: Option<Vec<NamedValue>>) -> &mut Self {
        self.attributes = attributes;
        self
    }

    pub fn with_attributes(mut self, attributes: Vec<NamedValue>) -> Self {

        let attributes = if attributes.is_empty() { None } else { Some(attributes) };
        self.set_attributes(attributes);
        self
    }

    pub fn attributes(&self) -> Option<&[NamedValue]> {
        self.attributes.as_deref()
    }

    pub fn set_content(&mut self, content: Option<Vec<ExprNode>>) -> &mut Self {
        self.content = content;
        self
    }

    pub fn with_content(mut self, content: Vec<ExprNode>) -> Self {
        let content = if content.is_empty() { None } else { Some(content) };
        self.set_content(content);
        self
    }

    pub fn content(&self) -> Option<&[ExprNode]> {
        self.content.as_deref()
    }
}

use crate::ExprNode;
use crate::NamedValue;
use pg_basics::Str;
