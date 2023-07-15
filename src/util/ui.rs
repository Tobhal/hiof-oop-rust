use std::{
    error::Error,
    fmt
};

pub use field_editable_derive::FieldEditable;
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
    fn get_fields(&self) -> Vec<(&'static str, String)>;
    fn edit_field(&mut self, field: &'static str, value: String) -> Result<(), Box<dyn Error>>;
}
