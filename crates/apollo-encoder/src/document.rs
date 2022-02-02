use crate::{EnumDef, InputObjectDef, InterfaceDef, ObjectDef, ScalarDef, SchemaDef, UnionDef};

#[derive(Debug)]
pub struct Document {
    // operation_definitions: Vec<OperationDef>,
    // fragment_definitions: Vec<FragmentDef>,
    schema_definition: SchemaDef,
    // Type definitions
    object_type_definitions: Vec<ObjectDef>,
    scalar_type_definitions: Vec<ScalarDef>,
    interface_type_definitions: Vec<InterfaceDef>,
    union_type_definitions: Vec<UnionDef>,
    enum_type_definitions: Vec<EnumDef>,
    input_object_type_definitions: Vec<InputObjectDef>,
}

// TODO check if extend type definition could just be normal definition without description
