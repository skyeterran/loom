use crate::parse::Exp::{ self, * };

#[derive(Debug)]
pub enum Form {
    Define(String, Exp),
    Unknown,
}

impl Form {
    pub fn from_exp(x: Exp) -> Self {
        match x {
            SExp { car, cdr } => {
                match *car {
                    Atom(atom) => {
                        match atom.as_str() {
                            "def" => {
                                let Some(Atom(key)) = cdr.get(0) else { todo!() };
                                let Some(value) = cdr.get(1) else { todo!() };
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
