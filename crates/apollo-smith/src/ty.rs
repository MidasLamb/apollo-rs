use apollo_encoder::Type_;
use arbitrary::Result;
use once_cell::sync::Lazy;

use crate::{name::Name, DocumentBuilder};

// TODO should be include in document builder when created
static BUILTIN_SCALAR_NAMES: Lazy<[Ty; 5]> = Lazy::new(|| {
    [
        Ty::Named(Name::new(String::from("Int"))),
        Ty::Named(Name::new(String::from("Float"))),
        Ty::Named(Name::new(String::from("String"))),
        Ty::Named(Name::new(String::from("Boolean"))),
        Ty::Named(Name::new(String::from("ID"))),
    ]
});

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    Named(Name),
    List(Box<Ty>),
    NonNull(Box<Ty>),
}

impl From<Ty> for Type_ {
    fn from(val: Ty) -> Self {
        match val {
            Ty::Named(name) => Type_::NamedType { name: name.into() },
            Ty::List(ty) => Type_::List {
                ty: Box::new((*ty).into()),
            },
            Ty::NonNull(ty) => Type_::NonNull {
                ty: Box::new((*ty).into()),
            },
        }
    }
}

impl<'a> DocumentBuilder<'a> {
    pub fn ty(&mut self) -> Result<Ty> {
        self.generate_ty(true)
    }

    pub fn choose_ty(&mut self, existing_types: &[Ty]) -> Result<Ty> {
        self._choose_ty(existing_types, true)
    }

    fn _choose_ty(&mut self, existing_types: &[Ty], is_nullable: bool) -> Result<Ty> {
        let ty: Ty = match self.u.int_in_range(0..=2usize)? {
            // Named type
            0 => {
                let used_type_names: Vec<&Ty> = existing_types
                    .iter()
                    .chain(BUILTIN_SCALAR_NAMES.iter())
                    .collect();

                self.u.choose(&used_type_names)?.to_owned().clone()
            }
            // List type
            1 => Ty::List(Box::new(self._choose_ty(existing_types, true)?)),
            // Non Null type
            2 => {
                if is_nullable {
                    Ty::NonNull(Box::new(self._choose_ty(existing_types, false)?))
                } else {
                    self._choose_ty(existing_types, is_nullable)?
                }
            }
            _ => unreachable!(),
        };

        Ok(ty)
    }

    // Private method
    fn generate_ty(&mut self, is_nullable: bool) -> Result<Ty> {
        let ty = match self.u.int_in_range(0..=2usize)? {
            // Named type
            0 => Ty::Named(self.name()?),
            // List type
            1 => Ty::List(Box::new(self.generate_ty(true)?)),
            // Non Null type
            2 => {
                if is_nullable {
                    Ty::NonNull(Box::new(self.generate_ty(false)?))
                } else {
                    self.generate_ty(is_nullable)?
                }
            }
            _ => unreachable!(),
        };

        Ok(ty)
    }
}
