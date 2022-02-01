use std::{collections::HashSet, hash::Hash};

use apollo_encoder::{EnumDef, EnumValue};
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

impl From<EnumTypeDef> for EnumDef {
    fn from(enum_: EnumTypeDef) -> Self {
        let mut new_enum = EnumDef::new(enum_.name.into());
        new_enum.description(enum_.description.map(String::from));
        enum_
            .enum_values_def
            .into_iter()
            .for_each(|val| new_enum.value(val.into()));

        new_enum
    }
}

#[derive(Debug, Clone, Eq)]
pub struct EnumValueDefinition {
    pub(crate) description: Option<Description>,
    pub(crate) value: Name,
    // TODO
    // pub(crate) directives: Vec<Directive>,
}

impl From<EnumValueDefinition> for EnumValue {
    fn from(enum_val: EnumValueDefinition) -> Self {
        let mut new_enum_val = Self::new(enum_val.value.into());
        new_enum_val.description(enum_val.description.map(String::from));

        new_enum_val
    }
}

impl PartialEq for EnumValueDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Hash for EnumValueDefinition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
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
        let enum_values_def = self.enum_values_definition()?;

        Ok(EnumTypeDef {
            description,
            name,
            enum_values_def,
        })
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

    pub fn enum_values_definition(&mut self) -> Result<HashSet<EnumValueDefinition>> {
        let mut enum_values_def = HashSet::with_capacity(self.u.int_in_range(2..=10usize)?);
        for i in (1..self.u.int_in_range(2..=10usize)?) {
            let description = self
                .u
                .arbitrary()
                .unwrap_or(false)
                .then(|| self.description())
                .transpose()?;
            let value = self.name_with_index(i)?;

            enum_values_def.insert(EnumValueDefinition { description, value });
        }

        Ok(enum_values_def)
    }
}
