use crate::parse::Exp::{ self, * };

#[derive(Debug)]
pub enum Form {
    Define(String, Exp),
    Unknown,
}

impl Form {
    pub fn from_exp(x: Exp) -> Self {
        match x {
            SExp { kind, args, kwargs } => {
                match *kind {
                    Atom(atom) => {
                        match atom.as_str() {
                            "def" => {
                                let Some(Atom(key)) = args.get(0) else { todo!() };
                                let Some(value) = args.get(1) else { todo!() };
                                Self::Define(key.clone(), value.clone())
                            }
                            _ => Self::Unknown
                        }
                    }
                    _ => Self::Unknown
                }
            }
            _ => Self::Unknown
        }
    }
}
