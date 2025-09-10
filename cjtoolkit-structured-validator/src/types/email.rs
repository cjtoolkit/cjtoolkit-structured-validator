//! This module contains structures and traits for working with email addresses.

use crate::base::string_rules::StringMandatoryRules;
use crate::common::locale::{
    LocaleData, LocaleMessage, ValidateErrorCollector, ValidateErrorStore,
};
use crate::common::string_validator::{StrValidationExtension, StringValidator};
use crate::common::validation_check::ValidationCheck;
use email_address_parser::EmailAddress;
use std::sync::Arc;
use thiserror::Error;

/// The `EmailRules` struct is used to define the rules or constraints associated with an email field.
///
/// Fields:
/// - `is_mandatory` (bool): Determines whether the email field is mandatory or optional.
///   - `true`: The email field is required and must be provided.
///   - `false`: The email field is optional and can be left empty.
pub struct EmailRules {
    pub is_mandatory: bool,
}

impl Default for EmailRules {
    fn default() -> Self {
        Self { is_mandatory: true }
    }
}

impl Into<StringMandatoryRules> for &EmailRules {
    fn into(self) -> StringMandatoryRules {
        StringMandatoryRules {
            is_mandatory: self.is_mandatory,
        }
    }
}

impl EmailRules {
    fn rule(&self) -> StringMandatoryRules {
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
        let rule = self.rule();
        rule.check(messages, subject);
    }
}

/// Represents an error type for email validation within an application.
///
/// This custom error type wraps the `ValidateErrorStore` to provide more specific
/// error handling for email validation logic. It is designed to be cloneable,
/// comparable, and debuggable, leveraging the `Error` trait for integration
/// with standard error handling mechanisms.
///
/// # Attributes
/// - `ValidateErrorStore`: The inner structure that stores validation details and errors.
///
/// # Traits
/// - `Debug`: Allows for easy debugging by formatting the error structure.
/// - `Error`: Implements the standard error trait, making it compatible with Rust's error-handling ecosystem.
/// - `PartialEq`: Enables comparison between two `EmailError` instances.
/// - `Clone`: Allows for creating a duplicate of the `EmailError`.
/// - `Default`: Provides a default instance for the `EmailError`.
///
/// # Error Message
/// The error message for this type is defined as `"Email Validation Error"`.
#[derive(Debug, Error, PartialEq, Clone, Default)]
#[error("Email Validation Error")]
pub struct EmailError(pub ValidateErrorStore);

impl ValidationCheck for EmailError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

/// A struct representing an email which encapsulates a string value of the email, an optional validated email address,
/// and a boolean flag to indicate whether the email is none or not.
///
/// # Fields
/// - `0: String` - The raw email address as a string.
/// - `1: Option<EmailAddress>` - An optional `EmailAddress` type representing a validated email, if applicable.
/// - `2: bool` - A boolean indicating whether the email is none (`true`) or not (`false`).
///
/// # Derives
/// - `Debug` - Enables formatting of the struct using the `{:?}` formatter for debugging purposes.
/// - `PartialEq` - Enables comparison of two `Email` instances for equality.
/// - `Clone` - Allows creating a clone (deep copy) of an `Email` instance.
#[derive(Debug, PartialEq, Clone)]
pub struct Email(String, Option<EmailAddress>, bool);

impl Default for Email {
    fn default() -> Self {
        Self(String::default(), None, true)
    }
}

/// `EmailAddressLocale` is an enumeration that represents the various outcomes
/// related to validating an email address. It is used to categorize common
/// issues encountered with email validation.
///
/// Variants:
///
/// - `InvalidEmail`: Indicates that the provided string is not a valid email
///   address format as per email standards. This could result from missing
///   components, extra characters, or an invalid structure (e.g., missing "@" or domain).
///
/// - `DoesNotMatch`: Indicates that the email address does not match a specific
///   expected pattern, format, or criteria. This variant can be used to enforce
///   custom validation logic beyond the standard email format, such as specific
///   domain requirements.
///
/// This enum is particularly useful for handling errors or providing detailed
/// feedback in applications that involve email registration, form input validation,
/// and similar use cases.
pub enum EmailAddressLocale {
    /// Indicates that the provided string is not a valid email address format as per email standards.
    /// # Key
    /// `validate-email-invalid`
    InvalidEmail,
    /// Indicates that the email address does not match a specific expected pattern, format, or criteria.
    /// # Key
    /// `validate-email-does-not-match`
    DoesNotMatch,
}

impl LocaleMessage for EmailAddressLocale {
    fn get_locale_data(&self) -> Arc<LocaleData> {
        use LocaleData as ld;
        match self {
            Self::InvalidEmail => ld::new("validate-email-invalid"),
            Self::DoesNotMatch => ld::new("validate-email-does-not-match"),
        }
    }
}

impl Email {
    /// Parses a given input string into a custom email representation and applies validation rules.
    ///
    /// # Arguments
    ///
    /// * `s` - An `Option<&str>` representing the input email string. If `None`, it is treated as an empty string.
    /// * `rules` - An `EmailRules` instance containing the validation logic to apply.
    ///
    /// # Returns
    ///
    /// A `Result<Self, EmailError>`:
    /// - `Ok(Self)` if the input is successfully parsed and validated as per the provided rules.
    /// - `Err(EmailError)` if validation or parsing fails, containing the associated validation or parsing errors.
    ///
    /// # Logic
    ///
    /// 1. Determines if the input is `None`. If `None`, treats it as an empty string.
    /// 2. Validates the string using `rules` by invoking its `check` method, collecting any validation errors.
    /// 3. If validation errors are present, returns an `EmailError`.
    /// 4. Attempts to parse the string into an `EmailAddress`:
    ///    - If parsing succeeds, wraps the parsed email and relevant details in the result.
    ///    - If parsing fails, creates and returns an `EmailError` indicating an invalid email format.
    ///
    /// # Errors
    ///
    /// Returns an `EmailError` in the following cases:
    /// - The input fails validation as determined by `rules`.
    /// - The input string cannot be parsed into a valid `EmailAddress`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cjtoolkit_structured_validator::types::email::{EmailRules, Email};
    ///
    /// let rules = EmailRules::default();
    ///
    /// match Email::parse_custom(Some("example@domain.com"), rules) {
    ///     Ok(_) => println!("Valid email"),
    ///     Err(_) => eprintln!("Error"),
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// - The function will treat `None` inputs as empty strings but checks for their presence when applying validation rules.
    /// - The email parsing relies on the functionality of the `EmailAddress` type.
    /// - Validation errors are accumulated and returned collectively within an `EmailError`.
    pub fn parse_custom(s: Option<&str>, rules: EmailRules) -> Result<Self, EmailError> {
        let is_none = s.is_none();
        let s = s.unwrap_or_default();
        let subject = s.as_string_validator();
        let mut messages = ValidateErrorCollector::new();
        rules.check(&mut messages, &subject, is_none);
        EmailError::validate_check(messages)?;

        let email = match EmailAddress::parse(s, None) {
            Some(email) => email,
            None => {
                let mut messages = ValidateErrorCollector::new();
                messages.push((
                    "Invalid Email".to_string(),
                    Box::new(EmailAddressLocale::InvalidEmail),
                ));
                return Err(EmailError(messages.into()));
            }
        };

        Ok(Self(s.to_string(), Some(email), is_none))
    }

    /// Parses an optional string slice into an instance of the current type.
    ///
    /// This function attempts to create an instance of the type from
    /// the given optional string slice (`s`) using the default email
    /// parsing rules (`EmailRules::default()`).
    ///
    /// # Arguments
    /// * `s` - An `Option<&str>` representing the input string slice to parse.
    ///          If `None`, parsing will fail and return an error.
    ///
    /// # Returns
    /// * `Ok(Self)` - If the string slice is successfully parsed into the current type.
    /// * `Err(EmailError)` - If the string slice fails to meet the required format based
    ///                        on the email parsing rules, or if the input is `None`.
    ///
    /// # Errors
    /// This function returns an `EmailError` if the provided string slice does not conform
    /// to the required email parsing rules or is invalid.
    ///
    /// # Examples
    /// ```
    /// use cjtoolkit_structured_validator::types::email::{Email};
    ///
    /// let valid_email_input = Some("user@example.com");
    /// let invalid_email_input = Some("invalid-email");
    ///
    /// assert!(Email::parse(valid_email_input).is_ok());
    /// assert!(Email::parse(invalid_email_input).is_err());
    /// ```
    ///
    /// This function delegates the parsing to `Self::parse_custom` with
    /// default rules provided by `EmailRules::default()`.
    pub fn parse(s: Option<&str>) -> Result<Self, EmailError> {
        Self::parse_custom(s, EmailRules::default())
    }

    /// Validates and processes the confirmation email input.
    ///
    /// This function compares the provided confirmation email (`confirm_email`)
    /// with the existing email (stored in `self`). If the emails do not match,
    /// it collects the validation error and returns an appropriate `EmailError`.
    ///
    /// # Parameters
    /// - `confirm_email`: A reference to a string slice representing the email
    ///   entered for confirmation.
    ///
    /// # Returns
    /// - `Ok(Self)` containing the original email instance if the confirmation email passes validation and matches `self`.
    /// - `Err(EmailError)`: Returns an appropriate error if the confirmation email
    ///   does not match or if there are validation issues.
    ///
    /// # Errors
    /// - `EmailError`: Encapsulates a collection of one or more validation errors indicating the mismatch or other issues.
    pub fn parse_confirm(&self, confirm_email: &str) -> Result<Self, EmailError> {
        let mut messages = ValidateErrorCollector::new();
        if self.0 != confirm_email.to_string() {
            messages.push((
                "Email does not match".to_string(),
                Box::new(EmailAddressLocale::DoesNotMatch),
            ));
        }
        EmailError::validate_check(messages)?;
        Ok(self.clone())
    }

    /// Retrieves the email address associated with the object, if available.
    ///
    /// # Returns
    ///
    /// - `Some(&EmailAddress)`: A reference to the `EmailAddress` if it is available.
    /// - `None`: If no email address is associated with the object.
    pub fn as_email(&self) -> Option<&EmailAddress> {
        self.1.as_ref()
    }

    /// Returns a string slice (`&str`) that represents the value stored in the current instance.
    ///
    /// # Returns
    ///
    /// A reference to the string value contained within the instance.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Converts the current instance of the type into an `Option<Email>`.
    ///
    /// This method checks the value of a boolean flag (assumed to be the
    /// third field in the tuple, `self.2`) to determine the result:
    ///
    /// - If the flag (`self.2`) is `true`, the method returns `None`.
    /// - If the flag is `false`, the method wraps the value in a `Some`
    ///   and returns it.
    ///
    /// # Returns
    /// - `Option<Email>`: `Some(self)` if `self.2` is `false`.
    /// - `None` if `self.2` is `true`.
    pub fn into_option(self) -> Option<Email> {
        if self.2 { None } else { Some(self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let email = Email::parse(Some("test@example.com"));
        assert!(email.is_ok());
    }

    #[test]
    fn test_invalid_email() {
        let email = Email::parse(Some("test"));
        assert!(email.is_err());
    }

    #[test]
    fn test_email_confirm_valid() {
        let email = Email::parse(Some("test@example.com")).unwrap_or_default();
        let email_confirm = email.parse_confirm("test@example.com");
        assert!(email_confirm.is_ok());
    }

    #[test]
    fn test_email_confirm_invalid() {
        let email = Email::parse(Some("test@example.com")).unwrap_or_default();
        let email_confirm = email.parse_confirm("test");
        assert!(email_confirm.is_err());
    }
}
