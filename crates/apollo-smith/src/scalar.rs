use arbitrary::Result;

use crate::{description::Description, directive::Directive, name::Name, DocumentBuilder};

#[derive(Debug)]
pub struct ScalarTypeDef {
    pub(crate) name: Name,
    pub(crate) description: Option<Description>,
    pub(crate) directives: Vec<Directive>,
    pub(crate) extend: bool,
}

impl From<ScalarTypeDef> for apollo_encoder::ScalarDef {
    fn from(scalar_def: ScalarTypeDef) -> Self {
        let mut new_scalar_def = Self::new(scalar_def.name.into());
        scalar_def
            .directives
            .into_iter()
            .for_each(|directive| new_scalar_def.directive(directive.into()));
        if scalar_def.extend {
            new_scalar_def.extend();
        }

        new_scalar_def
    }
}

impl<'a> DocumentBuilder<'a> {
    pub fn scalar_type_definition(&mut self) -> Result<ScalarTypeDef> {
        let name = self.type_name()?;
        let description = self
            .u
            .arbitrary()
            .unwrap_or(false)
            .then(|| self.description())
            .transpose()?;
        let directives = self.directives()?;
        // Extended scalar must have directive
        let extend = !directives.is_empty() && self.u.arbitrary().unwrap_or(false);

        Ok(ScalarTypeDef {
            name,
            description,
            directives,
            extend,
        })
    }
}
