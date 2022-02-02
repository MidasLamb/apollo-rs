use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Variable(String),
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Enum(String),
    List(Vec<Value>),
    Object(Vec<(String, Value)>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Variable(v) => write!(f, "${v}"),
            Self::Int(i) => write!(f, "{i}"),
            Self::Float(fl) => write!(f, "{fl}"),
            Self::String(s) => write!(f, r#""{s}""#),
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Null => write!(f, "null"),
            Self::Enum(val) => write!(f, "{val}"),
            Self::List(list) => write!(
                f,
                "[{}]",
                list.iter()
                    .map(|elt| format!("{elt}"))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Object(obj) => write!(
                f,
                "{{ {} }}",
                obj.iter()
                    .map(|(k, v)| format!("{}: {v}", String::from(k)))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}
