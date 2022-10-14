use std::fmt::Display;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum BuiltinType {
    Void,
    Bool,
}

impl Display for BuiltinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuiltinType::Void => write!(f, "Void"),
            BuiltinType::Bool => write!(f, "Bool"),
        }
    }
}
