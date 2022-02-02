use std::collections::HashSet;

use apollo_encoder::InterfaceDef;
use arbitrary::Result;

use crate::{
    description::Description, directive::Directive, field::FieldDef, name::Name, DocumentBuilder,
};

#[derive(Debug, Clone)]
pub struct InterfaceTypeDef {
    pub(crate) description: Option<Description>,
    pub(crate) name: Name,
    // TODO
    // interfaces: Vec<String>
    pub(crate) directives: Vec<Directive>,
    pub(crate) fields_def: Vec<FieldDef>,
}

impl From<InterfaceTypeDef> for InterfaceDef {
    fn from(itf: InterfaceTypeDef) -> Self {
        let mut itf_def = InterfaceDef::new(itf.name.into());
        itf_def.description(itf.description.map(String::from));
        itf.fields_def
            .into_iter()
            .for_each(|f| itf_def.field(f.into()));
        itf.directives
            .into_iter()
            .for_each(|directive| itf_def.directive(directive.into()));
        // TODO interfaces

        itf_def
    }
}

impl<'a> DocumentBuilder<'a> {
    pub fn interface_type_definition(&mut self) -> Result<InterfaceTypeDef> {
        let description = self
            .u
            .arbitrary()
            .unwrap_or(false)
            .then(|| self.description())
            .transpose()?;
        let name = self.type_name()?;
        let fields_def = self.fields_definition(&[])?;
        let directives = self.directives()?;

        Ok(InterfaceTypeDef {
            description,
            name,
            fields_def,
            directives,
        })
    }

    pub fn interface_implements(&mut self) -> Result<HashSet<Name>> {
        // let num_itf = self.u.arbitrary::<usize>().unwrap() % self.interface_type_defs.len();
        let num_itf = self
            .u
            .int_in_range(0..=(self.interface_type_defs.len() - 1))?;
        let mut interface_impls = HashSet::with_capacity(num_itf);

        for _ in 0..num_itf {
            interface_impls.insert(self.u.choose(&self.interface_type_defs)?.name.clone());
        }

        Ok(interface_impls)
    }
}
