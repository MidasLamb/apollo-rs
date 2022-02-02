use std::fmt;

use crate::Argument;

/// Directives can be used to describe additional information for types, fields, fragments and operations.
#[derive(Debug, PartialEq, Clone)]
pub struct Directive {
    name: String,
    args: Vec<Argument>,
}

impl Directive {
    pub fn new(name: String) -> Self {
        Self {
            name,
            args: Vec::new(),
        }
    }

    pub fn arg(&mut self, arg: Argument) {
        self.args.push(arg);
    }
}

impl fmt::Display for Directive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{}", self.name)?;

        if !self.args.is_empty() {
            for (i, arg) in self.args.iter().enumerate() {
                match i {
                    0 => write!(f, "({}", arg)?,
                    _ => write!(f, ", {}", arg)?,
                }
            }
            write!(f, ")")?;
        }

        Ok(())
    }
}
