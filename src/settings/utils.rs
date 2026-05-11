/// Trait to convert a string to a boolean
pub trait IniToBool {
    fn to_bool(&self) -> bool;
}

impl IniToBool for &str {
    fn to_bool(&self) -> bool {
        self.to_lowercase() == "true" || self == &"1" || self == &"yes" || self == &"on"
    }
}

pub trait IniToBoolOpt {
    fn to_bool_opt(&self) -> Option<bool>;
}

impl IniToBoolOpt for Option<&str> {
    fn to_bool_opt(&self) -> Option<bool> {
        match self {
            Some(s) => Some(s.to_bool()),
            None => None,
        }
    }
}
