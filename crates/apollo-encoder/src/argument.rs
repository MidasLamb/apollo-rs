use std::fmt;

use crate::{InputValueDef, Value};

#[derive(Debug)]
pub struct ArgumentsDef {
    input_value_definitions: Vec<InputValueDef>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Argument {
    pub(crate) name: String,
    pub(crate) value: Value,
}

impl Argument {
    pub fn new(name: String, value: Value) -> Self {
        Self { name, value }
    }
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

impl ArgumentsDef {
    pub fn new(input_value_definitions: Vec<InputValueDef>) -> Self {
        Self {
            input_value_definitions,
        }
    }
}

impl fmt::Display for ArgumentsDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        if self.input_value_definitions.len() == 1 {
            write!(f, "{}, ", self.input_value_definitions[0])?;
        } else {
            for (i, input_val_def) in self.input_value_definitions.iter().enumerate() {
                if i != self.input_value_definitions.len() - 1 {
                    write!(f, "{}, ", input_val_def)?;
                } else {
                    write!(f, "{}", input_val_def)?;
                }
            }
        }
        write!(f, ")")
    }
}

#[cfg(test)]
mod tests {
    use crate::Type_;

    use super::*;

    #[test]
    fn it_encodes_arguments_definitions() {
        let input_value_defs = vec![
            InputValueDef::new(
                String::from("first"),
                Type_::NamedType {
                    name: String::from("Int"),
                },
            ),
            InputValueDef::new(
                String::from("second"),
                Type_::List {
                    ty: Box::new(Type_::NamedType {
                        name: String::from("Int"),
                    }),
                },
            ),
        ];
        let arguments_def = ArgumentsDef::new(input_value_defs);

        assert_eq!(arguments_def.to_string(), r#"(first: Int, second: [Int])"#);
    }
}
