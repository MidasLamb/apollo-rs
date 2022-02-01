use arbitrary::Result;

use crate::DocumentBuilder;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Description {
    desc: StringValue,
}

impl From<Description> for String {
    fn from(desc: Description) -> Self {
        desc.desc.content
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringValue {
    content: String,
}

impl StringValue {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl Description {
    pub(crate) fn new(content: String) -> Self {
        Description {
            desc: StringValue::new(content),
        }
    }
}

impl<'a> DocumentBuilder<'a> {
    pub fn description(&mut self) -> Result<Description> {
        Ok(Description::new(self.limited_string(50)?))
    }
}
