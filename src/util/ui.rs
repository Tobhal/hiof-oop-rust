use std::{
    error::Error,
    fmt
};

pub struct NoFieldError(pub usize);

impl fmt::Display for NoFieldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No field with index = {}", self.0)
    }
}

impl fmt::Debug for NoFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No field with index = {}", self.0)
    }
}

impl Error for NoFieldError {}

pub struct Field<'f> {
    pub(crate) name: &'f str,
    pub(crate) value: String
}

pub trait FieldEditable {
    fn edit_field(&mut self, index: usize, value: String) -> Result<(), Box<dyn Error>>;
    fn get_field(& self) -> Vec<Field>;
}
