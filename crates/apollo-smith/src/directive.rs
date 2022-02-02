use arbitrary::Result;

use crate::{argument::Argument, name::Name, DocumentBuilder};

// TODO implement DirectiveDef

#[derive(Debug, Clone, PartialEq)]
pub struct Directive {
    pub(crate) name: Name,
    pub(crate) arguments: Vec<Argument>,
}

impl From<Directive> for apollo_encoder::Directive {
    fn from(directive: Directive) -> Self {
        let mut new_directive = Self::new(directive.name.into());
        directive
            .arguments
            .into_iter()
            .for_each(|arg| new_directive.arg(arg.into()));

        new_directive
    }
}

impl<'a> DocumentBuilder<'a> {
    pub fn directives(&mut self) -> Result<Vec<Directive>> {
        // TODO choose only existing directives
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
