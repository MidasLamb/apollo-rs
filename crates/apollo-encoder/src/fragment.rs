use std::fmt;

use crate::{Directive, SelectionSet};

#[derive(Debug)]
pub struct FragmentDef {
    name: String,
    type_condition: TypeCondition,
    directives: Vec<Directive>,
    selection_set: SelectionSet,
}

impl FragmentDef {
    pub fn new(name: String, type_condition: TypeCondition, selection_set: SelectionSet) -> Self {
        Self {
            name,
            type_condition,
            selection_set,
            directives: Vec::new(),
        }
    }

    /// Add a directive.
    pub fn directive(&mut self, directive: Directive) {
        self.directives.push(directive)
    }
}

impl fmt::Display for FragmentDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fragment {} {}", self.name, self.type_condition)?;
        for directive in &self.directives {
            write!(f, " {}", directive)?;
        }
        writeln!(f, "{}", self.selection_set);

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FragmentSpread {
    name: String,
    directives: Vec<Directive>,
}

impl FragmentSpread {
    pub fn new(name: String) -> Self {
        Self {
            name,
            directives: Vec::new(),
        }
    }

    /// Add a directive.
    pub fn directive(&mut self, directive: Directive) {
        self.directives.push(directive)
    }
}

impl fmt::Display for FragmentSpread {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "...{}", self.name)?;
        for directive in &self.directives {
            write!(f, " {}", directive)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InlineFragment {
    type_condition: Option<TypeCondition>,
    directives: Vec<Directive>,
    selection_set: SelectionSet,
}

impl InlineFragment {
    pub fn new(selection_set: SelectionSet) -> Self {
        Self {
            selection_set,
            type_condition: Option::default(),
            directives: Vec::new(),
        }
    }

    /// Add a directive.
    pub fn directive(&mut self, directive: Directive) {
        self.directives.push(directive)
    }

    /// Set the inline fragment's type condition.
    pub fn type_condition(&mut self, type_condition: Option<TypeCondition>) {
        self.type_condition = type_condition;
    }
}

impl fmt::Display for InlineFragment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "...")?;
        if let Some(type_condition) = &self.type_condition {
            write!(f, " {}", type_condition)?;
        }
        for directive in &self.directives {
            write!(f, " {}", directive)?;
        }

        write!(f, " {}", self.selection_set)?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeCondition {
    name: String,
}

impl TypeCondition {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl fmt::Display for TypeCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "on {}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::{field::Field, Selection};

    use super::*;

    #[test]
    fn it_encodes_fragment_definitions() {
        let selections = vec![
            Selection::Field(Field::new(String::from("myField"))),
            Selection::FragmentSpread(FragmentSpread::new(String::from("myFragment"))),
        ];
        let mut selection_set = SelectionSet::new();
        selections
            .into_iter()
            .for_each(|s| selection_set.selection(s));
        let mut inline_fragment = InlineFragment::new(selection_set);
        inline_fragment.type_condition(Some(TypeCondition::new(String::from("User"))));

        assert_eq!(
            inline_fragment.to_string(),
            indoc! {r#"
                ... on User {
                  myField
                  ...myFragment
                }
            "#}
        );
    }
}
