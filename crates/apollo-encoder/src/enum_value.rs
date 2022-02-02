use std::fmt;

use crate::{Directive, StringValue};

/// The __EnumValue type represents one of possible values of an enum.
///
/// *EnumValueDefinition*:
///     Description? EnumValue Directives?
///
/// Detailed documentation can be found in [GraphQL spec](https://spec.graphql.org/October2021/#sec-The-__EnumValue-Type).
///
/// ### Example
/// ```rust
/// use apollo_encoder::{EnumValue};
///
/// let mut enum_ty = EnumValue::new("CARDBOARD_BOX".to_string());
/// enum_ty.description(Some("Box nap spot.".to_string()));
/// enum_ty.deprecated(Some("Box was recycled.".to_string()));
///
/// assert_eq!(
///     enum_ty.to_string(),
///     r#"  "Box nap spot."
///   CARDBOARD_BOX @deprecated(reason: "Box was recycled.")"#
/// );
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct EnumValue {
    // Name must return a String.
    name: String,
    // Description may return a String or null.
    description: StringValue,
    // Deprecated returns true if this enum value should no longer be used, otherwise false.
    is_deprecated: bool,
    // Deprecation reason optionally provides a reason why this enum value is deprecated.
    deprecation_reason: StringValue,
    /// The vector of directives
    directives: Vec<Directive>,
}

impl EnumValue {
    /// Create a new instance of EnumValue.
    pub fn new(name: String) -> Self {
        Self {
            name,
            is_deprecated: false,
            description: StringValue::Field { source: None },
            deprecation_reason: StringValue::Reason { source: None },
            directives: Vec::new(),
        }
    }

    /// Set the Enum Value's description.
    pub fn description(&mut self, description: Option<String>) {
        self.description = StringValue::Field {
            source: description,
        };
    }

    /// Set the Enum Value's deprecation properties.
    pub fn deprecated(&mut self, reason: Option<String>) {
        self.is_deprecated = true;
        self.deprecation_reason = StringValue::Reason { source: reason };
    }

    /// Add a directive.
    pub fn directive(&mut self, directive: Directive) {
        self.directives.push(directive)
    }
}

impl fmt::Display for EnumValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)?;
        write!(f, "  {}", self.name)?;

        if self.is_deprecated {
            write!(f, " @deprecated")?;
            if let StringValue::Reason { source: _ } = &self.deprecation_reason {
                write!(f, "(reason:")?;
                write!(f, "{}", self.deprecation_reason)?;
                write!(f, ")")?
            }
        }
        for directive in &self.directives {
            write!(f, " {}", directive)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Argument, Value};

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_encodes_an_enum_value() {
        let enum_ty = EnumValue::new("CAT_TREE".to_string());
        assert_eq!(enum_ty.to_string(), "  CAT_TREE");
    }

    #[test]
    fn it_encodes_an_enum_value_with_desciption() {
        let mut enum_ty = EnumValue::new("CAT_TREE".to_string());
        enum_ty.description(Some("Top bunk of a cat tree.".to_string()));
        assert_eq!(
            enum_ty.to_string(),
            r#"  "Top bunk of a cat tree."
  CAT_TREE"#
        );
    }
    #[test]
    fn it_encodes_an_enum_value_with_deprecated() {
        let mut enum_ty = EnumValue::new("CARDBOARD_BOX".to_string());
        enum_ty.description(Some("Box nap\nspot.".to_string()));
        enum_ty.deprecated(Some("Box was recycled.".to_string()));

        assert_eq!(
            enum_ty.to_string(),
            r#"  """
  Box nap
  spot.
  """
  CARDBOARD_BOX @deprecated(reason: "Box was recycled.")"#
        );
    }

    #[test]
    fn it_encodes_an_enum_value_with_directive() {
        let mut enum_ty = EnumValue::new("CARDBOARD_BOX".to_string());
        let mut directive = Directive::new(String::from("testDirective"));
        directive.arg(Argument::new(
            String::from("first"),
            Value::List(vec![Value::Int(1), Value::Int(2)]),
        ));
        enum_ty.description(Some("Box nap\nspot.".to_string()));
        enum_ty.directive(directive);

        assert_eq!(
            enum_ty.to_string(),
            r#"  """
  Box nap
  spot.
  """
  CARDBOARD_BOX @testDirective(first: [1, 2])"#
        );
    }

    #[test]
    fn it_encodes_an_enum_value_with_deprecated_block_string_value() {
        let mut enum_ty = EnumValue::new("CARDBOARD_BOX".to_string());
        enum_ty.description(Some("Box nap\nspot.".to_string()));
        enum_ty.deprecated(Some("Box was \"recycled\".".to_string()));

        assert_eq!(
            enum_ty.to_string(),
            r#"  """
  Box nap
  spot.
  """
  CARDBOARD_BOX @deprecated(reason:
  """
  Box was "recycled".
  """
  )"#
        );
    }
}
