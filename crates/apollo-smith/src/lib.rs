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

use arbitrary::{Result, Unstructured};
use directive::DirectiveDef;
use document::Document;
use enum_::EnumTypeDef;
use fragment::FragmentDef;
use input_object::InputObjectTypeDef;
use interface::InterfaceTypeDef;
use object::ObjectTypeDef;
use operation::OperationDef;
use scalar::ScalarTypeDef;
use schema::SchemaDef;
use union::UnionTypeDef;

pub(crate) mod argument;
pub(crate) mod description;
pub(crate) mod directive;
pub(crate) mod document;
pub(crate) mod enum_;
pub(crate) mod field;
pub(crate) mod fragment;
pub(crate) mod input_object;
pub(crate) mod input_value;
pub(crate) mod interface;
pub(crate) mod name;
pub(crate) mod object;
pub(crate) mod operation;
pub(crate) mod scalar;
pub(crate) mod schema;
pub(crate) mod selection_set;
pub(crate) mod ty;
pub(crate) mod union;
pub(crate) mod variable;

pub struct DocumentBuilder<'a> {
    pub(crate) u: &'a mut Unstructured<'a>,
    pub(crate) input_object_type_defs: Vec<InputObjectTypeDef>,
    pub(crate) object_type_defs: Vec<ObjectTypeDef>,
    pub(crate) interface_type_defs: Vec<InterfaceTypeDef>,
    pub(crate) union_type_defs: Vec<UnionTypeDef>,
    pub(crate) enum_type_defs: Vec<EnumTypeDef>,
    pub(crate) scalar_type_defs: Vec<ScalarTypeDef>,
    pub(crate) schema_defs: Vec<SchemaDef>,
    pub(crate) directive_defs: Vec<DirectiveDef>,
    pub(crate) operation_defs: Vec<OperationDef>,
    pub(crate) fragment_defs: Vec<FragmentDef>,
}

impl<'a> DocumentBuilder<'a> {
    pub fn new(u: &'a mut Unstructured<'a>) -> Result<Self> {
        let mut builder = Self {
            u,
            object_type_defs: Vec::new(),
            interface_type_defs: Vec::new(),
            enum_type_defs: Vec::new(),
            schema_defs: Vec::new(),
            directive_defs: Vec::new(),
            operation_defs: Vec::new(),
            fragment_defs: Vec::new(),
            scalar_type_defs: Vec::new(),
            union_type_defs: Vec::new(),
            input_object_type_defs: Vec::new(),
        };

        for _ in 0..builder.u.int_in_range(1..=50)? {
            let fragment_def = builder.fragment_definition()?;
            builder.fragment_defs.push(fragment_def);
        }

        for _ in 0..builder.u.int_in_range(1..=50)? {
            let scalar_type_def = builder.scalar_type_definition()?;
            builder.scalar_type_defs.push(scalar_type_def);
        }

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

        for _ in 0..builder.u.int_in_range(1..=50)? {
            let union_type_def = builder.union_type_definition()?;
            builder.union_type_defs.push(union_type_def);
        }

        for _ in 0..builder.u.int_in_range(1..=50)? {
            let input_object_type_def = builder.input_object_type_definition()?;
            builder.input_object_type_defs.push(input_object_type_def);
        }

        for _ in 0..builder.u.int_in_range(1..=50)? {
            let schema_def = builder.schema_definition()?;
            builder.schema_defs.push(schema_def);
        }

        for _ in 0..builder.u.int_in_range(1..=50)? {
            let directive_def = builder.directive_def()?;
            builder.directive_defs.push(directive_def);
        }

        for _ in 0..builder.u.int_in_range(1..=50)? {
            let operation_def = builder.operation_definition()?;
            builder.operation_defs.push(operation_def);
        }

        Ok(builder)
    }

    pub fn finish(self) -> String {
        apollo_encoder::Document::from(Document {
            schema_definitions: self.schema_defs,
            object_type_definitions: self.object_type_defs,
            interface_type_definitions: self.interface_type_defs,
            enum_type_definitions: self.enum_type_defs,
            directive_definitions: self.directive_defs,
            operation_definitions: self.operation_defs,
            fragment_definitions: self.fragment_defs,
            scalar_type_definitions: self.scalar_type_defs,
            union_type_definitions: self.union_type_defs,
            input_object_type_definitions: self.input_object_type_defs,
        })
        .to_string()
    }
}
