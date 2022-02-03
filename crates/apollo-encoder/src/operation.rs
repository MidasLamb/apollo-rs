use std::fmt;

use crate::{Directive, SelectionSet, VariableDef};

#[derive(Debug)]
pub struct OperationDef {
    operation_type: OperationType,
    name: Option<String>,
    variable_definitions: Vec<VariableDef>,
    directives: Vec<Directive>,
    selection_set: SelectionSet,
}

impl OperationDef {
    pub fn new(operation_type: OperationType, selection_set: SelectionSet) -> Self {
        Self {
            operation_type,
            selection_set,
            name: None,
            variable_definitions: Vec::new(),
            directives: Vec::new(),
        }
    }

    /// Set the operation def's name.
    pub fn name(&mut self, name: Option<String>) {
        self.name = name;
    }

    /// Add a variable definitions.
    pub fn variable_definition(&mut self, variable_definition: VariableDef) {
        self.variable_definitions.push(variable_definition);
    }

    /// Add a directive.
    pub fn directive(&mut self, directive: Directive) {
        self.directives.push(directive);
    }
}

impl fmt::Display for OperationDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.operation_type)?;
        if let Some(name) = &self.name {
            write!(f, " {}", name)?;
        }
        if !self.variable_definitions.is_empty() {
            write!(f, "(")?;
            for (i, var_def) in self.variable_definitions.iter().enumerate() {
                if i == self.variable_definitions.len() - 1 {
                    write!(f, "{}", var_def)?;
                } else {
                    write!(f, "{}, ", var_def)?;
                }
            }
            write!(f, ")")?;
        }
        for directive in &self.directives {
            write!(f, " {}", directive)?;
        }
        write!(f, " {}", self.selection_set)?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum OperationType {
    Query,
    Mutation,
    Subscription,
}

impl fmt::Display for OperationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperationType::Query => write!(f, "query"),
            OperationType::Mutation => write!(f, "mutation"),
            OperationType::Subscription => write!(f, "subscription"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{field::Field, Argument, Selection, Type_, Value};
    use indoc::indoc;

    #[test]
    fn it_encodes_a_query_operation() {
        let selection_set = {
            let sels = vec![
                Selection::Field(Field::new(String::from("first"))),
                Selection::Field(Field::new(String::from("second"))),
            ];
            let mut sel_set = SelectionSet::new();
            sels.into_iter().for_each(|sel| sel_set.selection(sel));

            sel_set
        };
        let var_def = VariableDef::new(
            String::from("variable_def"),
            Type_::List {
                ty: Box::new(Type_::NamedType {
                    name: String::from("Int"),
                }),
            },
        );
        let mut new_op = OperationDef::new(OperationType::Query, selection_set);
        let mut directive = Directive::new(String::from("testDirective"));
        directive.arg(Argument::new(
            String::from("first"),
            Value::String("one".to_string()),
        ));
        new_op.variable_definition(var_def);
        new_op.directive(directive);

        assert_eq!(
            new_op.to_string(),
            indoc! { r#"
                query($variable_def: [Int]) @testDirective(first: "one") {
                  first
                  second
                }
            "#}
        );
    }
}
