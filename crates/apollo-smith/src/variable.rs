use arbitrary::Result;

use crate::{directive::Directive, input_value::InputValue, name::Name, ty::Ty, DocumentBuilder};

#[derive(Debug)]
pub struct VariableDef {
    name: Name,
    ty: Ty,
    default_value: Option<InputValue>,
    directives: Vec<Directive>,
}

impl From<VariableDef> for apollo_encoder::VariableDef {
    fn from(var_def: VariableDef) -> Self {
        let mut new_var_def = Self::new(var_def.name.into(), var_def.ty.into());
        new_var_def.default_value(var_def.default_value.map(Into::into));
        var_def
            .directives
            .into_iter()
            .for_each(|directive| new_var_def.directive(directive.into()));

        new_var_def
    }
}

impl<'a> DocumentBuilder<'a> {
    pub fn variable_definitions(&mut self) -> Result<Vec<VariableDef>> {
        (0..self.u.int_in_range(0..=7usize)?)
            .map(|_| self.variable_definition()) // TODO do not generate duplication variable name
            .collect()
    }

    pub fn variable_definition(&mut self) -> Result<VariableDef> {
        let name = self.type_name()?;
        let ty = self.choose_ty(&self.list_existing_types())?;
        let default_value = self
            .u
            .arbitrary()
            .unwrap_or(false)
            .then(|| self.input_value())
            .transpose()?;
        let operation_type = self.u.arbitrary()?;
        let directives = self.directives()?;

        Ok(VariableDef {
            name,
            ty,
            default_value,
            directives,
        })
    }
}
