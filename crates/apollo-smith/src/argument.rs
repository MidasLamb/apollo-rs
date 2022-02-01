use arbitrary::Result;

use crate::{
    input_value::{InputValue, InputValueDef},
    name::Name,
    DocumentBuilder,
};

#[derive(Debug, Clone)]
pub struct ArgumentsDef {
    pub(crate) input_value_definitions: Vec<InputValueDef>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub(crate) name: Name,
    pub(crate) value: InputValue,
}
// TODO implement From

impl<'a> DocumentBuilder<'a> {
    pub fn arguments(&mut self) -> Result<Vec<Argument>> {
        let num_arguments = self.u.int_in_range(0..=4)?;
        let arguments = (0..num_arguments)
            .map(|_| self.argument())
            .collect::<Result<Vec<_>>>()?;

        Ok(arguments)
    }

    pub fn argument(&mut self) -> Result<Argument> {
        let name = self.name()?;
        let value = self.input_value()?;

        Ok(Argument { name, value })
    }

    pub fn arguments_definition(&mut self) -> Result<ArgumentsDef> {
        Ok(ArgumentsDef {
            input_value_definitions: self.input_values_def()?,
        })
    }
}
