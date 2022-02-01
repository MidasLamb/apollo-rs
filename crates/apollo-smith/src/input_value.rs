use crate::{name::Name, DocumentBuilder};
use arbitrary::Result;

#[derive(Debug, Clone)]
pub enum InputValue {
    // TODO
    // Variable()
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Enum(Name),
    List(Vec<InputValue>),
    Object(Vec<(Name, InputValue)>),
}

impl From<InputValue> for apollo_encoder::InputValue {
    fn from(input_value: InputValue) -> Self {
        // InputValue as we use it is not implemented in apollo_encoder
        todo!()
    }
}

impl<'a> DocumentBuilder<'a> {
    pub fn input_value(&mut self) -> Result<InputValue> {
        let val = match self.u.int_in_range(0..=7usize)? {
            // Int
            0 => InputValue::Int(self.u.arbitrary()?),
            // Float
            1 => InputValue::Float(self.u.arbitrary()?),
            // String
            2 => InputValue::String(self.limited_string(40)?),
            // Boolean
            3 => InputValue::Boolean(self.u.arbitrary()?),
            // Null
            4 => InputValue::Null,
            // Enum
            5 => {
                if !self.enum_type_defs.is_empty() {
                    // TODO get rid of this clone
                    let enum_choosed = self.choose_enum()?.clone();
                    InputValue::Enum(self.arbitrary_variant(&enum_choosed)?.clone())
                } else {
                    self.input_value()?
                }
            }
            // List
            6 => {
                // FIXME: it's wrong it should always be the same type inside
                InputValue::List(
                    (1..self.u.int_in_range(2..=4usize)?)
                        .map(|_| self.input_value())
                        .collect::<Result<Vec<_>>>()?,
                )
            }
            // Object
            7 => InputValue::Object(
                (1..self.u.int_in_range(2..=4usize)?)
                    .map(|_| Ok((self.name()?, self.input_value()?)))
                    .collect::<Result<Vec<_>>>()?,
            ),
            _ => unreachable!(),
        };

        Ok(val)
    }
}
