use arbitrary::Result;

use crate::{input_value::InputValue, name::Name, DocumentBuilder};

#[derive(Debug, Clone)]
pub struct ArgumentsDef {}

#[derive(Debug, Clone)]
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
}
