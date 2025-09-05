use crate::common::locale::{ValidateErrorCollector, ValidateErrorStore};

/// A trait for performing validation checks and handling validation-related errors.
///
/// This trait provides methods to:
/// - Create a new validation error instance from an error store.
/// - Collect and validate errors, returning a result depending on whether any errors are found.
///
/// # Associated Types
/// - `Self`: The concrete type that implements the `ValidationCheck` trait, which represents validation errors.
///
/// # Required Methods
///
/// ## `validate_new`
///
/// Creates a new instance of the implementing type using a provided `ValidateErrorStore`.
///
/// ### Parameters
/// - `messages`: An instance of `ValidateErrorStore` that holds validation error information.
///
/// ### Returns
/// - `Self`: A new instance of the implementing type initialized with the given error messages.
///
///
/// # Provided Methods
///
/// ## `validate_check`
///
/// Performs a validation check using a `ValidateErrorCollector`. If the collector contains errors,
/// it returns an error wrapped in the implementing type; otherwise, it succeeds with `Ok(())`.
///
/// ### Parameters
/// - `messages`: An instance of `ValidateErrorCollector` that holds collected validation errors.
///
/// ### Returns
/// - `Ok(())`: If the collector does not contain any errors.
/// - `Err(Self)`: If the collector contains errors, an error instance of the implementing type is returned.
///
/// The default implementation checks if the provided `messages` is empty. If it is empty, it returns an `Ok(())`.
/// Otherwise, it converts the messages into a `ValidateErrorStore` and creates a new validation error instance using `validate_new`.
///
///
pub trait ValidationCheck: Sized {
    fn validate_new(messages: ValidateErrorStore) -> Self;

    fn validate_check(messages: ValidateErrorCollector) -> Result<(), Self> {
        if messages.is_empty() {
            Ok(())
        } else {
            Err(Self::validate_new(messages.into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::string_rules::StringMandatoryLocale;

    struct TestValidationCheck;

    impl ValidationCheck for TestValidationCheck {
        fn validate_new(_: ValidateErrorStore) -> Self {
            Self
        }
    }

    #[test]
    fn test_validate_check_is_err() {
        let mut messages = ValidateErrorCollector::new();
        messages.push(("error".to_string(), Box::new(StringMandatoryLocale)));
        assert!(TestValidationCheck::validate_check(messages).is_err());
    }

    #[test]
    fn test_validate_check_is_ok() {
        let messages = ValidateErrorCollector::new();
        assert!(TestValidationCheck::validate_check(messages).is_ok());
    }
}
