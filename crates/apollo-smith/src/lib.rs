// Implement DocumentBuilder
// Implement DocumentBuilderAllocations
// Transform every function in grammar to a method of impl DocumentBuilder

// fn f64_const(
//     u: &mut Unstructured,
//     _: &Module,
//     builder: &mut CodeBuilder,
// ) -> Result<Instruction> {
//     let x = u.arbitrary::<f64>()?;
//     builder.push_operands(&[ValType::F64]);
//     Ok(Instruction::F64Const(x))
// }

use apollo_encoder::Schema;
use arbitrary::{Result, Unstructured};
use enum_::EnumTypeDef;
use interface::InterfaceTypeDef;
use object::ObjectTypeDef;

pub(crate) mod argument;
pub(crate) mod description;
pub(crate) mod document;
pub(crate) mod field;
// pub(crate) mod grammar;
pub(crate) mod directive;
pub(crate) mod enum_;
pub(crate) mod input_value;
pub(crate) mod interface;
pub(crate) mod name;
pub(crate) mod object;
pub(crate) mod ty;

pub struct DocumentBuilder<'a> {
    pub(crate) u: &'a mut Unstructured<'a>,
    pub(crate) object_type_defs: Vec<ObjectTypeDef>,
    pub(crate) interface_type_defs: Vec<InterfaceTypeDef>,
    pub(crate) enum_type_defs: Vec<EnumTypeDef>,
}

impl<'a> DocumentBuilder<'a> {
    pub fn new(u: &'a mut Unstructured<'a>) -> Result<Self> {
        let mut builder = Self {
            u,
            object_type_defs: Vec::new(),
            interface_type_defs: Vec::new(),
            enum_type_defs: Vec::new(),
        };

        for _ in 0..builder.u.int_in_range(1..=50)? {
            let enum_type_def = builder.enum_type_definition()?;
            builder.enum_type_defs.push(enum_type_def);
        }

        for _ in 0..builder.u.int_in_range(1..=50)? {
            let interface_type_def = builder.interface_type_definition()?;
            builder.interface_type_defs.push(interface_type_def);
        }

        for _ in 0..builder.u.int_in_range(1..=50)? {
            let object_type_def = builder.object_type_definition()?;
            builder.object_type_defs.push(object_type_def);
        }

        Ok(builder)
    }

    pub fn finish(self) -> Schema {
        let mut schema = Schema::new();
        self.enum_type_defs
            .into_iter()
            .for_each(|enum_| schema.enum_(enum_.into()));
        self.interface_type_defs
            .into_iter()
            .for_each(|itf| schema.interface(itf.into()));
        self.object_type_defs
            .into_iter()
            .for_each(|obj| schema.object(obj.into()));

        schema
    }
}
