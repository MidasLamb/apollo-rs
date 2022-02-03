use std::collections::HashSet;

use arbitrary::Result;

use crate::{
    argument::ArgumentsDef, description::Description, directive::Directive, name::Name, ty::Ty,
    DocumentBuilder,
};

#[derive(Debug, Clone)]
pub struct FieldDef {
    pub(crate) description: Option<Description>,
    pub(crate) name: Name,
    pub(crate) arguments_definition: Option<ArgumentsDef>,
    pub(crate) ty: Ty,
    pub(crate) directives: Vec<Directive>,
}

impl From<FieldDef> for apollo_encoder::FieldDef {
    fn from(val: FieldDef) -> Self {
        let mut field = Self::new(val.name.into(), val.ty.into());
        if let Some(arg) = val.arguments_definition {
            arg.input_value_definitions
                .into_iter()
                .for_each(|input_val| field.arg(input_val.into()));
        }
        field.description(val.description.map(String::from));
        val.directives
            .into_iter()
            .for_each(|directive| field.directive(directive.into()));

        field
    }
}

impl<'a> DocumentBuilder<'a> {
    pub fn fields_definition(&mut self, exclude: &[&Name]) -> Result<Vec<FieldDef>> {
        let num_fields = self.u.int_in_range(2..=50usize)?;
        let mut fields_names = HashSet::with_capacity(num_fields);

        for i in 0..num_fields {
            let name = self.name_with_index(i)?;
            if !exclude.contains(&&name) {
                fields_names.insert(name);
            }
        }

        // TODO add mechanism to add own type for recursive type
        let available_types: Vec<Ty> = self.list_existing_types();

        fields_names
            .into_iter()
            .map(|field_name| {
                Ok(FieldDef {
                    description: self
                        .u
                        .arbitrary()
                        .unwrap_or(false)
                        .then(|| self.description())
                        .transpose()?,
                    name: field_name,
                    arguments_definition: self
                        .u
                        .arbitrary()
                        .unwrap_or(false)
                        .then(|| self.arguments_definition())
                        .transpose()?,
                    ty: self.choose_ty(&available_types)?,
                    directives: self.directives()?,
                })
            })
            .collect()
    }
}
