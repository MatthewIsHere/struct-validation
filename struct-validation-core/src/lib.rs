/// Represents an error that occurs during validation of a struct's field.
///
/// Each `ValidationError` contains the name of the field that failed validation
/// and an associated error message explaining the reason for the failure.
#[derive(Debug)]
pub struct ValidationError {
    /// The name of the field that failed validation.
    pub field: String,
    
    /// A message describing the validation error.
    pub message: String,
}

impl ValidationError {
    /// Creates a new `ValidationError` for a specific field with a given message.
    ///
    /// # Arguments
    ///
    /// * `field` - The name of the field that failed validation.
    /// * `message` - A description of the validation error.
    ///
    /// # Examples
    ///
    /// ```
    /// use struct_validation_core::ValidationError;
    ///
    /// let error = ValidationError::new("username", "must not be empty");
    /// assert_eq!(error.field, "username");
    /// assert_eq!(error.message, "must not be empty");
    /// ```
    pub fn new(field: &str, message: &str) -> Self {
        Self {
            field: field.to_string(),
            message: message.to_string(),
        }
    }

    /// Adds a prefix to the field name, separated by a dot.
    ///
    /// This can be useful for nested validation errors where the field
    /// is part of a larger struct.
    ///
    /// # Arguments
    ///
    /// * `prefix` - The prefix to add to the field name.
    ///
    /// # Examples
    ///
    /// ```
    /// use struct_validation_core::ValidationError;
    ///
    /// let mut error = ValidationError::new("username", "must not be empty");
    /// error.add_prefix("user");
    /// assert_eq!(error.field, "user.username");
    /// ```
    pub fn add_prefix(&mut self, prefix: &str) {
        self.field = format!("{}.{}", prefix, self.field);
    }
}

/// A trait for validating structs.
///
/// Implement this trait for your structs to define custom validation logic.
/// The `validate` method should return a vector of `ValidationError`s indicating
/// any validation failures.
pub trait Validate {
    /// Validates the current instance and returns a list of validation errors.
    ///
    /// If the instance is valid, the returned vector should be empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use struct_validation_core::{Validate, ValidationError};
    ///
    /// struct User {
    ///     username: String,
    ///     email: String,
    /// }
    ///
    /// impl Validate for User {
    ///     fn validate(&self) -> Vec<ValidationError> {
    ///         let mut errors = Vec::new();
    ///         
    ///         if self.username.is_empty() {
    ///             errors.push(ValidationError::new("username", "must not be empty"));
    ///         }
    ///         
    ///         if !self.email.contains('@') {
    ///             errors.push(ValidationError::new("email", "must contain '@'"));
    ///         }
    ///         
    ///         errors
    ///     }
    /// }
    /// ```
    fn validate(&self) -> Vec<ValidationError>;
}

/// A macro to simplify validation checks.
///
/// **Usage:** `validate!(vec, (boolean test expression), "field", "message")`
///
/// This macro checks the provided boolean test expression, and if it evaluates to `true`,
/// it pushes a new `ValidationError` onto the provided vector.
///
/// # Arguments
///
/// * `$vec` - The vector to which the `ValidationError` will be added.
/// * `$test` - A boolean expression that determines whether to add the error.
/// * `$field_name` - The name of the field related to the validation error.
/// * `$message` - A message describing the validation error.
///
/// # Examples
///
/// ```
/// use struct_validation_core::{validate, ValidationError};
///
/// let mut errors = Vec::new();
/// let username = "";
/// validate!(errors, username.is_empty(), "username", "must not be empty");
/// assert_eq!(errors.len(), 1);
/// assert_eq!(errors[0].field, "username");
/// assert_eq!(errors[0].message, "must not be empty");
/// ```
#[macro_export]
macro_rules! validate {
    ($vec:expr, $test:expr, $field_name:expr, $message:expr) => {
        if $test {
            $vec.push($crate::ValidationError::new($field_name, $message));
        }
    };
}
