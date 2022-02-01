use std::collections::HashSet;

use arbitrary::Result;

use crate::{description::Description, directive::Directive, name::Name, DocumentBuilder};

#[derive(Debug, Clone)]
pub struct EnumTypeDef {
    pub(crate) description: Option<Description>,
    pub(crate) name: Name,
    // TODO
    // pub(crate) directives: Vec<Directive>,
    pub(crate) enum_values_def: HashSet<EnumValueDefinition>,
}

#[derive(Debug, Clone)]
pub struct EnumValueDefinition {
    pub(crate) description: Option<Description>,
    pub(crate) value: Name,
    // TODO
    // pub(crate) directives: Vec<Directive>,
}

impl PartialEq for EnumValueDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<'a> DocumentBuilder<'a> {
    pub fn enum_type_definition(&mut self) -> Result<EnumTypeDef> {
        let description = self
            .u
            .arbitrary()
            .unwrap_or(false)
            .then(|| self.description())
            .transpose()?;
        let name = self.type_name()?;

        //  --------------------------------- HERE
        todo!()
    }

    pub fn choose_enum(&mut self) -> Result<&EnumTypeDef> {
        self.u.choose(&self.enum_type_defs)
    }

    pub fn arbitrary_variant<'b>(&mut self, enum_: &'b EnumTypeDef) -> Result<&'b Name> {
        let arbitrary_idx = self.u.int_in_range(0..=(enum_.enum_values_def.len() - 1))?;
        Ok(enum_
            .enum_values_def
            .iter()
            .nth(arbitrary_idx)
            .map(|e| &e.value)
            .expect("cannot get variant"))
    }
}
