use crate::{Field, FragmentSpread, InlineFragment};
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct SelectionSet {
    selections: Vec<Selection>,
}

impl SelectionSet {
    pub fn new() -> Self {
        Self {
            selections: Vec::new(),
        }
    }

    pub fn selection(&mut self, selection: Selection) {
        self.selections.push(selection);
    }
}

impl fmt::Display for SelectionSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{")?;
        for sel in &self.selections {
            writeln!(f, "{}", sel)?;
        }
        writeln!(f, "}}")?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Selection {
    Field(Field),
    FragmentSpread(FragmentSpread),
    InlineFragment(InlineFragment),
}

impl fmt::Display for Selection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Selection::Field(field) => write!(f, "  {field}"),
            Selection::FragmentSpread(fragment_spread) => write!(f, "  {fragment_spread}"),
            Selection::InlineFragment(inline_fragment) => write!(f, "  {inline_fragment}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn it_encodes_selection_set() {
        let selections = vec![
            Selection::Field(Field::new(String::from("myField"))),
            Selection::FragmentSpread(FragmentSpread::new(String::from("myFragment"))),
        ];
        let mut selection_set = SelectionSet::new();
        selections
            .into_iter()
            .for_each(|s| selection_set.selection(s));

        assert_eq!(
            selection_set.to_string(),
            indoc! {r#"
                {
                  myField
                  ...myFragment
                }
            "#}
        )
    }
}
