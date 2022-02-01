use arbitrary::Result;

use crate::{argument::Argument, name::Name, DocumentBuilder};

#[derive(Debug, Clone, PartialEq)]
pub struct Directive {
    pub(crate) name: Name,
    pub(crate) arguments: Vec<Argument>,
}
// TODO implement From

impl<'a> DocumentBuilder<'a> {
    pub fn directives(&mut self) -> Result<Vec<Directive>> {
        let num_directives = self.u.int_in_range(0..=4)?;
        let directives = (0..num_directives)
            .map(|_| self.directive())
            .collect::<Result<Vec<_>>>()?;

        Ok(directives)
    }

    pub fn directive(&mut self) -> Result<Directive> {
        let name = self.name()?;
        let arguments = self.arguments()?;

        Ok(Directive { name, arguments })
    }
}
