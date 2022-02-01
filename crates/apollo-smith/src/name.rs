use arbitrary::Result;

use crate::DocumentBuilder;

const CHARSET_LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const CHARSET_NUMBERS: &[u8] = b"0123456789";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    pub(crate) name: String,
}

impl From<Name> for String {
    fn from(val: Name) -> Self {
        val.name
    }
}

impl Name {
    pub const fn new(name: String) -> Self {
        Self { name }
    }
}

impl<'a> DocumentBuilder<'a> {
    pub fn name(&mut self) -> Result<Name> {
        Ok(Name::new(self.limited_string(30)?))
    }

    /// Generate an object/interface type name
    pub fn type_name(&mut self) -> Result<Name> {
        let mut new_name = self.limited_string(30)?;
        if self
            .object_type_defs
            .iter()
            .map(|o| &o.name)
            .chain(self.interface_type_defs.iter().map(|itf| &itf.name))
            .any(|n| n.name == new_name)
        {
            new_name.push_str(&format!("{}", self.object_type_defs.len()));
        }
        Ok(Name::new(new_name))
    }

    pub fn name_with_index(&mut self, index: usize) -> Result<Name> {
        let mut name = self.limited_string(30)?;
        name.push_str(&format!("{}", index));

        Ok(Name::new(name))
    }

    pub fn known_name(&mut self) -> Name {
        Name::new(String::from("RANDOM_NAME"))
    }

    // Mirror what happens in `Arbitrary for String`, but do so with a clamped size.
    pub(crate) fn limited_string(&mut self, max_size: usize) -> Result<String> {
        loop {
            let size = self.u.int_in_range(0..=max_size)?;

            let gen_str = String::from_utf8(
                (0..size)
                    .map(|curr_idx| {
                        // TODO fix this by using choose
                        let idx = self.u.arbitrary::<usize>()?;

                        // Cannot start with a number
                        let ch = if curr_idx == 0 {
                            CHARSET_LETTERS[idx % CHARSET_LETTERS.len()]
                        } else {
                            let idx = idx % (CHARSET_LETTERS.len() + CHARSET_NUMBERS.len());
                            if idx < CHARSET_LETTERS.len() {
                                CHARSET_LETTERS[idx]
                            } else {
                                CHARSET_NUMBERS[idx - CHARSET_LETTERS.len()]
                            }
                        };

                        Ok(ch)
                    })
                    .collect::<Result<Vec<u8>>>()?,
            )
            .unwrap();

            if !gen_str.is_empty() {
                break Ok(gen_str);
            }
        }
    }
}
