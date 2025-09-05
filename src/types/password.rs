//! This module contains structures and traits for working with passwords.

use crate::base::string_rules::{StringLengthRules, StringMandatoryRules, StringSpecialCharRules};
use crate::common::locale::{
    LocaleData, LocaleMessage, ValidateErrorCollector, ValidateErrorStore,
};
use crate::common::string_validator::{StrValidationExtension, StringValidator};
use crate::common::validation_check::ValidationCheck;
use thiserror::Error;

/// Represents a set of rules or constraints that define the criteria for a valid password.
///
/// # Fields
/// - `is_mandatory`:
///   Indicates whether the password is mandatory (true) or optional (false).
///
/// - `must_have_uppercase`:
///   Specifies if the password is required to contain at least one uppercase letter.
///
/// - `must_have_lowercase`:
///   Specifies if the password is required to contain at least one lowercase letter.
///
/// - `must_have_special_chars`:
///   Specifies if the password is required to contain at least one special character.
///   E.g., special characters could include `!`, `@`, `#`, `$`, etc.
///
/// - `must_have_digit`:
///   Specifies if the password is required to contain at least one numerical digit.
///
/// - `min_length`:
///   The minimum allowed length for the password, if specified.
///   If `None`, there is no minimum length restriction.
///
/// - `max_length`:
///   The maximum allowed length for the password, if specified.
///   If `None`, there is no maximum length restriction.
pub struct PasswordRules {
    pub is_mandatory: bool,
    pub must_have_uppercase: bool,
    pub must_have_lowercase: bool,
    pub must_have_special_chars: bool,
    pub must_have_digit: bool,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

impl Default for PasswordRules {
    fn default() -> Self {
        Self {
            is_mandatory: true,
            must_have_uppercase: true,
            must_have_lowercase: true,
            must_have_special_chars: true,
            must_have_digit: true,
            min_length: Some(8),
            max_length: Some(64),
        }
    }
}

impl
    Into<(
        StringMandatoryRules,
        StringLengthRules,
        StringSpecialCharRules,
    )> for &PasswordRules
{
    fn into(
        self,
    ) -> (
        StringMandatoryRules,
        StringLengthRules,
        StringSpecialCharRules,
    ) {
        (
            StringMandatoryRules {
                is_mandatory: self.is_mandatory,
            },
            StringLengthRules {
                min_length: self.min_length,
                max_length: self.max_length,
            },
            StringSpecialCharRules {
                must_have_uppercase: self.must_have_uppercase,
                must_have_lowercase: self.must_have_lowercase,
                must_have_special_chars: self.must_have_special_chars,
                must_have_digit: self.must_have_digit,
            },
        )
    }
}

impl PasswordRules {
    fn rules(
        &self,
    ) -> (
        StringMandatoryRules,
        StringLengthRules,
        StringSpecialCharRules,
    ) {
        self.into()
    }

    fn check(
        &self,
        messages: &mut ValidateErrorCollector,
        subject: &StringValidator,
        is_none: bool,
    ) {
        if !self.is_mandatory && is_none {
            return;
        }
        let (mandatory_rule, length_rule, special_char_rule) = self.rules();
        mandatory_rule.check(messages, subject);
        if !messages.is_empty() {
            return;
        }
        length_rule.check(messages, subject);
        special_char_rule.check(messages, subject);
    }
}

/// Represents an error that occurs during password validation.
///
/// This struct is a wrapper around `ValidateErrorStore` and includes a custom error message
/// "Password Validation Error". It is used to encapsulate validation errors encountered while
/// handling password-related operations.
///
/// # Attributes
///
/// * `0` - A `ValidateErrorStore` instance that contains the details of validation errors.
///
/// # Derive Attributes
///
/// * `Debug` - Enables formatting the struct using the `{:?}` formatter for debugging purposes.
/// * `Error` - Implements the `std::error::Error` trait, making it an error type.
/// * `PartialEq` - Allows for comparison between two `PasswordError` instances for equality.
/// * `Clone` - Enables cloning of `PasswordError` instances.
/// * `Default` - Provides a default value for the struct, where `ValidateErrorStore` is also defaulted.
#[derive(Debug, Error, PartialEq, Clone, Default)]
#[error("Password Validation Error")]
pub struct PasswordError(pub ValidateErrorStore);

impl ValidationCheck for PasswordError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

/// A struct representing a combination of a password and its visibility state.
///
/// The `Password` struct contains two fields:
/// - A `String` which holds the actual password.
/// - A `bool` which indicates whether the password is visible or hidden.
///
/// # Derived Traits
/// - `PartialEq`: Allows comparison between two `Password` instances to check for equality.
/// - `Clone`: Enables the creation of a deep copy of a `Password` instance.
#[derive(PartialEq, Clone)]
pub struct Password(String, bool);

impl Default for Password {
    fn default() -> Self {
        Self(String::new(), true)
    }
}

/// A struct representing a validation error when a password does not meet
/// the requirements or criteria defined by a specific locale.
///
/// `PasswordDoesNotMatchLocale` is used to indicate that the provided password
/// fails to adhere to locale-specific password policies, such as minimum
/// length, required character sets, or other regional rules.
///
/// This struct itself does not hold any data and serves as a marker for this
/// specific type of validation error.
///
/// This struct can be leveraged in password validation frameworks or libraries
/// to distinguish and handle errors related to locale-specific password rules.
pub struct PasswordDoesNotMatchLocale;

impl LocaleMessage for PasswordDoesNotMatchLocale {
    fn get_locale_data(&self) -> LocaleData {
        LocaleData::new("validate-password-does-not-match")
    }
}

impl Password {
    /// Parses an optional string into a custom password type based on provided rules.
    ///
    /// # Parameters
    ///
    /// * `s`: An `Option<&str>` representing the input string to be parsed.
    ///   - If `None`, this indicates an empty password.
    ///   - If `Some`, the string will be validated according to the provided password rules.
    /// * `rules`: A `PasswordRules` instance that specifies the validation rules for the password.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Self)` if the string successfully satisfies all the password rules, with:
    ///   - The inner `String` representing the processed password.
    ///   - A `bool` flag indicating whether the original input was `None` (true if none, false otherwise).
    ///
    /// If the string does not satisfy the provided password rules, a `PasswordError` is returned containing validation error details.
    ///
    /// # Errors
    ///
    /// This function returns a `PasswordError` if:
    /// * The password violates any of the provided `PasswordRules`.
    /// * Any additional validation errors occur during rule checking.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cjtoolkit_structured_validator::types::password::{PasswordRules, Password, PasswordError};
    /// let password_rules = PasswordRules::default(); // Define your password rules.
    /// let input = Some("mySecurePa8s#"); // Example input string.
    /// let result = Password::parse_custom(input, password_rules);
    /// match result {
    ///     Ok(_) => println!("Password parsed successfully"),
    ///     Err(_) => println!("Failed to parse password"),
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// * The function uses a `ValidateErrorCollector` to collect and report multiple validation errors simultaneously.
    /// * If `s` is `None`, it will default to an empty string (`""`) for validation.
    pub fn parse_custom(s: Option<&str>, rules: PasswordRules) -> Result<Self, PasswordError> {
        let is_none = s.is_none();
        let s = s.unwrap_or_default();
        let subject = s.as_string_validator();
        let mut messages = ValidateErrorCollector::new();
        rules.check(&mut messages, &subject, is_none);
        PasswordError::validate_check(messages)?;
        Ok(Self(s.to_string(), is_none))
    }

    /// Parses the provided input string (`Option<&str>`) and attempts to create an instance of the type implementing this function.
    ///
    /// This function applies the default set of password rules to validate and process the input.
    /// If the string is valid, according to the default rules, it returns the constructed instance (`Self`) wrapped in a `Result::Ok`.
    /// Otherwise, it returns a `PasswordError` wrapped in a `Result::Err`.
    ///
    /// # Arguments
    ///
    /// * `s` - An optional reference to a string slice (`&str`) representing the input to be parsed. If `None` is provided,
    ///         the function may fail depending on the validation rules.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - If the input string meets the criteria defined by the default password rules.
    /// * `Err(PasswordError)` - If the input string violates the default password rules or fails other validations.
    ///
    /// # Examples
    ///
    /// ```
    /// use cjtoolkit_structured_validator::types::password::Password;
    ///
    /// let input = Some("Strong@password1");
    /// let result = Password::parse(input);
    /// assert!(result.is_ok());
    ///
    /// let invalid_input = Some("weak");
    /// let result = Password::parse(invalid_input);
    /// assert!(result.is_err());
    ///
    /// let empty_input = None;
    /// let result = Password::parse(empty_input);
    /// assert!(result.is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return a [`PasswordError`] if:
    /// * The provided input does not conform to the default [`PasswordRules`].
    /// * Any other validation checks fail while processing.
    ///
    /// This function delegates the parsing logic to the `parse_custom` method, supplying
    /// the `PasswordRules::default()` instance as the default rules for validation.
    ///
    /// See also: [`Self::parse_custom`], [`PasswordRules::default`], [`PasswordError`].
    pub fn parse(s: Option<&str>) -> Result<Self, PasswordError> {
        Self::parse_custom(s, PasswordRules::default())
    }

    /// Validates that the provided password confirmation matches the original password.
    ///
    /// # Parameters
    /// - `password_confirm`: A reference to a string slice representing the password confirmation.
    ///
    /// # Returns
    /// - `Result<Self, PasswordError>`:
    ///   - `Ok(Self)`: If the password confirmation matches the original password, it returns a clone of the current object.
    ///   - `Err(PasswordError)`: If the password confirmation does not match, it returns a `PasswordError` containing validation errors.
    ///
    /// # Errors
    /// - Returns `PasswordError` if the provided password confirmation does not match the existing password. The error
    ///   message will include a user-friendly explanation, such as "Password does not match".
    ///
    /// # Notes
    /// This function uses a `ValidateErrorCollector` to aggregate potential validation errors.
    /// If any error is detected (e.g., mismatched passwords), it is wrapped and returned as part
    /// of the `PasswordError`.
    pub fn parse_confirm(&self, password_confirm: &str) -> Result<Self, PasswordError> {
        let mut msgs = ValidateErrorCollector::new();

        (password_confirm != self.as_str()).then(|| {
            msgs.push((
                "Password does not match".to_string(),
                Box::new(PasswordDoesNotMatchLocale),
            ));
        });

        PasswordError::validate_check(msgs)?;
        Ok(self.clone())
    }

    /// Provides a string slice reference to the inner value.
    ///
    /// This method allows access to the inner string slice (`&str`) of the object.
    /// It borrows the value immutably, meaning the caller can read but not modify the string.
    ///
    /// # Returns
    /// A string slice (`&str`) that references the inner value.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Converts the current instance into an `Option<Password>`.
    ///
    /// # Returns
    /// - `None` if the second element of the tuple (`self.1`) is `true`.
    /// - `Some(self)` if the second element of the tuple (`self.1`) is `false`.
    ///
    /// This method is useful when conditions based on the boolean flag determine
    /// whether the password should be available or not.
    pub fn into_option(self) -> Option<Password> {
        if self.1 { None } else { Some(self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_parse_error_password_confirmation_mismatch() {
        let password = Password("match".to_string(), false);
        let password = password.parse_confirm("mismatch");
        assert!(password.is_err());
    }

    #[test]
    fn test_password_parse_error_password_confirmation_match() {
        let password = Password("match".to_string(), false);
        let password = password.parse_confirm("match");
        assert!(password.is_ok());
    }
}
