use crate::error::{Result, ValidationErrors};

/// Trait for validating domain objects before persistence.
pub trait Validatable {
    fn validate(&self) -> Result<()>;

    fn validate_with(errors: &mut ValidationErrors, condition: bool, field: &'static str, message: &str) {
        if !condition {
            errors.add(field, message);
        }
    }
}
