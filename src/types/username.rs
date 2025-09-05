//! This module contains structures and traits for working with usernames.

use crate::base::string_rules::{StringLengthRules, StringMandatoryRules};
use crate::common::locale::{
    LocaleData, LocaleMessage, ValidateErrorCollector, ValidateErrorStore,
};
use crate::common::string_validator::{StrValidationExtension, StringValidator};
use crate::common::validation_check::ValidationCheck;
use thiserror::Error;

/// Struct representing the rules and constraints applied to a username.
///
/// This struct defines attributes to control whether a username is required
/// and imposes optional length restrictions.
///
/// # Fields
/// - `is_mandatory`
///   A boolean flag indicating if the username is mandatory.
///   If `true`, a username must be provided; if `false`, it is optional.
///
/// - `min_length`
///   An optional `usize` specifying the minimum allowable length for the username.
///   If `Some(value)`, the username must be at least `value` characters long.
///   If `None`, there is no minimum length restriction.
///
/// - `max_length`
///   An optional `usize` specifying the maximum allowable length for the username.
///   If `Some(value)`, the username must be at most `value` characters long.
///   If `None`, there is no maximum length restriction.
///
/// This example specifies a username requirement that is mandatory, with a
/// minimum of 3 characters and a maximum of 16 characters.
pub struct UsernameRules {
    pub is_mandatory: bool,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

impl Default for UsernameRules {
    fn default() -> Self {
        Self {
            is_mandatory: true,
            min_length: Some(5),
            max_length: Some(30),
        }
    }
}

impl Into<(StringMandatoryRules, StringLengthRules)> for &UsernameRules {
    fn into(self) -> (StringMandatoryRules, StringLengthRules) {
        (
            StringMandatoryRules {
                is_mandatory: self.is_mandatory,
            },
            StringLengthRules {
                min_length: self.min_length,
                max_length: self.max_length,
            },
        )
    }
}

impl UsernameRules {
    fn rules(&self) -> (StringMandatoryRules, StringLengthRules) {
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
        let (mandatory_rule, length_rule) = self.rules();
        mandatory_rule.check(messages, subject);
        if !messages.is_empty() {
            return;
        }
        length_rule.check(messages, subject);
    }
}

///
/// Represents an error encountered during the validation of a username.
///
/// This struct is used to encapsulate validation errors specific to usernames
/// and provides a mechanism to store and manage multiple validation error details.
///
/// # Attributes
/// - `ValidateErrorStore`: A collection or storage that contains detailed information
///   about the specific validation errors encountered.
///
/// # Derives
/// - `Debug`: Enables formatting of the `UsernameError` for debugging purposes.
/// - `Error`: Implements the `std::error::Error` trait for compatibility with Rust's error handling conventions.
/// - `PartialEq`: Enables comparison between `UsernameError` instances for equality.
/// - `Clone`: Allows cloning of `UsernameError` instances.
/// - `Default`: Provides a default value for `UsernameError`, initializing an empty `ValidateErrorStore`.
///
/// # Error Message
/// The default error message for this type is: `"Username Validation Error"`.
///
/// # Usage
/// This struct is typically used internally in validation logic and may be returned
/// when a username fails to meet the specified validation criteria.
#[derive(Debug, Error, PartialEq, Clone, Default)]
#[error("Username Validation Error")]
pub struct UsernameError(pub ValidateErrorStore);

impl ValidationCheck for UsernameError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

/// A struct that represents a username with additional metadata.
///
/// The `Username` struct is a tuple struct consisting of:
/// - A `String` representing the username itself.
/// - A `bool` indicating additional information about the username, such as
///   whether it has been verified or meets certain criteria (interpreted based
///   on context).
///
/// # Traits Implemented
/// - `Debug`: Enables formatting the `Username` struct for debugging purposes.
/// - `PartialEq`: Allows for equality comparison between `Username` instances.
/// - `Clone`: Provides the ability to create duplicate instances of `Username`.
#[derive(Debug, PartialEq, Clone)]
pub struct Username(String, bool);

impl Default for Username {
    fn default() -> Self {
        Self(String::new(), true)
    }
}

/// A trait that defines a method to check if a provided username is already taken.
///
/// This trait can be implemented for types that manage or validate usernames, allowing them to
/// provide functionality to check if a given username exists (i.e., is already in use).
pub trait IsUsernameTaken {
    fn is_username_taken(&self, username: &str) -> bool;
}

/// This trait defines an asynchronous method to check if a given username is already taken.
///
/// # Required Method
///
/// - `is_username_taken_async`: Takes a reference to a username (`&str`) and returns
///   a future that resolves to a `bool`, indicating whether the username is already taken.
///
/// # Parameters
///
/// - `self`: The implementor object of the trait.
/// - `username`: A string slice that contains the username to check.
///
/// # Returns
///
/// This method returns an `impl Future` with an output of `bool`. When awaited, this future
/// will resolve to:
/// - `true`: If the username is already in use.
/// - `false`: If the username is available.
pub trait IsUsernameTakenAsync {
    fn is_username_taken_async(&self, username: &str) -> impl Future<Output = bool>;
}

/// A struct representing the locale or message type for the "username taken" error.
///
/// This struct can be used as part of an error handling system or localization framework
/// to represent scenarios where the provided username is already in use.
///
/// # Key
/// `validate-username-taken`
pub struct UsernameTakenLocale;

impl LocaleMessage for UsernameTakenLocale {
    fn get_locale_data(&self) -> LocaleData {
        LocaleData::new("validate-username-taken")
    }
}

impl Username {
    /// Parses and validates a custom username string based on predefined rules.
    ///
    /// # Parameters
    /// - `s`: An `Option<&str>` representing the input username string to be parsed.
    ///        If `None`, a default empty string will be used.
    /// - `rules`: A `UsernameRules` object which defines the validation rules to
    ///            enforce on the provided username.
    ///
    /// # Returns
    /// - `Result<Self, UsernameError>`: If the username passes all validation rules, it
    ///   returns an instance of the implementing struct wrapped in `Ok`. If validation fails,
    ///   returns a `UsernameError` wrapped in `Err`.
    ///
    /// # Workflow
    /// 1. Checks if the input username `s` is `None` and assigns a default value (empty string) if true.
    /// 2. Converts the input string into a proper validation subject (likely for easier validation handling).
    /// 3. Initializes a `ValidateErrorCollector` to aggregate any rule validation errors that occur.
    /// 4. Applies the specified rules to check the username, capturing errors if validation fails.
    /// 5. Checks the collected errors using `UsernameError::validate_check`. If errors exist, an appropriate
    ///    `UsernameError` is returned.
    /// 6. Constructs and returns the implementing struct containing the parsed string and its `is_none` status.
    ///
    /// # Errors
    /// - Returns a `UsernameError` if validation fails, providing detailed information on why the parsing or checks failed.
    ///
    /// # Examples
    /// ```rust
    /// use cjtoolkit_structured_validator::types::username::{UsernameRules, Username};
    /// let rules = UsernameRules::default();
    /// let result = Username::parse_custom(Some("validuser123"), rules);
    /// assert!(result.is_ok());
    ///
    /// let rules = UsernameRules::default();
    /// let result = Username::parse_custom(Some("inv"), rules);
    /// assert!(result.is_err());
    /// ```
    pub fn parse_custom(s: Option<&str>, rules: UsernameRules) -> Result<Self, UsernameError> {
        let is_none = s.is_none();
        let s = s.unwrap_or_default();
        let subject = s.as_string_validator();
        let mut messages = ValidateErrorCollector::new();
        rules.check(&mut messages, &subject, is_none);
        UsernameError::validate_check(messages)?;
        Ok(Self(s.to_string(), is_none))
    }

    /// Parses a given string slice (`Option<&str>`) into a `Self` instance using the default username rules.
    ///
    /// # Arguments
    /// - `s`: An optional string slice (`Option<&str>`) representing the username to be parsed.
    ///        If `None`, parsing will fail with an appropriate error.
    ///
    /// # Returns
    /// - `Ok(Self)`: If the provided string slice satisfies the default username rules.
    /// - `Err(UsernameError)`: If the parsing fails, for instance, due to the input violating username rules or being `None`.
    ///
    /// # Behavior
    /// This function relies on the `parse_custom` method to perform the parsing operation,
    /// delegating it with `UsernameRules::default()` which represents the default validation rules.
    ///
    /// # Examples
    /// ```
    /// use cjtoolkit_structured_validator::types::username::Username;
    /// let username = Some("valid_username");
    /// let parsed = Username::parse(username);
    /// assert!(parsed.is_ok());
    ///
    /// let invalid_username = Some("inv");
    /// let parsed = Username::parse(invalid_username);
    /// assert!(parsed.is_err());
    /// ```
    ///
    /// # Errors
    /// Return a `UsernameError` if:
    /// - The provided input is malformed or invalid, according to the default rules.
    /// - The input is `None` and cannot be processed.
    ///
    /// # See Also
    /// Refer to `parse_custom` if you need to parse with custom rules.
    pub fn parse(s: Option<&str>) -> Result<Self, UsernameError> {
        Self::parse_custom(s, UsernameRules::default())
    }

    /// Checks whether the username represented by the current instance is already taken.
    ///
    /// This method relies on an external service implementing the `IsUsernameTaken` trait
    /// to determine if the username is already registered or in use. If the username is
    /// taken, an error message is added to the validation error collector, and an error is
    /// returned. If the username is not taken, the method returns `Ok` with the current instance.
    ///
    /// # Type Parameters
    /// * `T` - A type that implements the `IsUsernameTaken` trait. This is used to query
    ///         whether the username is taken or not.
    ///
    /// # Parameters
    /// * `service` - A reference to an object implementing the `IsUsernameTaken` trait,
    ///               which is used for checking username availability.
    ///
    /// # Returns
    /// * `Ok(Self)` - If the username is successfully validated and is not taken.
    /// * `Err(UsernameError)` - If the username is already taken or if there are validation
    ///                          issues.
    ///
    /// # Errors
    /// * Returns a `UsernameError` if the username is already taken, with a localized message
    ///   indicating the issue. Validation failures are captured and reported through the error.
    ///
    /// # Implementation Details
    /// This method uses a `ValidateErrorCollector` to gather error messages. If the `is_username_taken`
    /// method on the provided service returns `true`, it adds a localized error message indicating
    /// that the username is already taken. The `UsernameError::validate_check` function is then
    /// called to process the collected errors and return a result.
    pub fn check_username_taken<T: IsUsernameTaken>(
        &self,
        service: &T,
    ) -> Result<Self, UsernameError> {
        let mut messages = ValidateErrorCollector::new();

        service.is_username_taken(self.as_str()).then(|| {
            messages.push(("Already taken".to_string(), Box::new(UsernameTakenLocale)));
        });

        UsernameError::validate_check(messages)?;
        Ok(self.clone())
    }

    /// Asynchronously checks if the username is already taken using the provided service and validates the result.
    ///
    /// # Arguments
    ///
    /// * `service` - A reference to a type that implements the `IsUsernameTakenAsync` trait. This service is used to
    ///   determine if the username is already taken.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - Returns a clone of the current instance (`Self`) if the username passes validation (not taken).
    /// * `Err(UsernameError)` - Returns an error of type `UsernameError` if the username is already taken or a validation error occurs.
    ///
    /// # Errors
    ///
    /// * Returns a `UsernameError` if the username is determined to be already taken by the `service`.
    /// * May return a `UsernameError` if one or more validation rules fail during the check.
    ///
    /// # Type Parameters
    ///
    /// * `T` - A type that implements the `IsUsernameTakenAsync` trait, which defines the asynchronous method
    ///   `is_username_taken_async` used for checking the username's availability.
    ///
    /// # Notes
    ///
    /// * The function uses an internal `ValidateErrorCollector` to collect validation errors.
    /// * This function internally calls `is_username_taken_async` on the `service` and expects an asynchronous boolean result.
    ///   If the result is `true`, it registers an error indicating the username is already taken.
    /// * Any validation error, such as the username being taken, will be propagated as a `UsernameError`.
    ///
    /// # Implementation Details
    ///
    /// * The function uses `ValidateErrorCollector` to aggregate errors.
    /// * If `is_username_taken_async` resolves to `true`, a localized error message ("Already taken") is pushed into the
    ///   error collector along with a reference to `UsernameTakenLocale`.
    /// * The `UsernameError::validate_check(messages)` call ensures that collected errors, if any, are validated and returned,
    ///   halting further execution if errors are present.
    pub async fn check_username_taken_async<T: IsUsernameTakenAsync>(
        &self,
        service: &T,
    ) -> Result<Self, UsernameError> {
        let mut messages = ValidateErrorCollector::new();

        service
            .is_username_taken_async(self.as_str())
            .await
            .then(|| {
                messages.push(("Already taken".to_string(), Box::new(UsernameTakenLocale)));
            });

        UsernameError::validate_check(messages)?;
        Ok(self.clone())
    }

    /// Returns the string slice representation of the current object.
    ///
    /// # Returns
    ///
    /// A string slice (`&str`) that refers to the internal string data.
    ///
    /// This method borrows from the underlying string data, meaning no
    /// additional allocations or copies are made.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Converts the `Username` wrapper into an `Option<Username>` type.
    ///
    /// If the internal boolean flag (`self.1`) is `true`, it returns `None`.
    /// Otherwise, it wraps the `Username` instance in a `Some` and returns it.
    ///
    /// # Returns
    ///
    /// * `None` - when the internal boolean flag is `true`.
    /// * `Some(Username)` - when the internal boolean flag is `false`.
    pub fn into_option(self) -> Option<Username> {
        if self.1 { None } else { Some(self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeUsernameCheckService(String);

    impl IsUsernameTaken for FakeUsernameCheckService {
        fn is_username_taken(&self, username: &str) -> bool {
            username == self.0.as_str()
        }
    }

    impl IsUsernameTakenAsync for FakeUsernameCheckService {
        async fn is_username_taken_async(&self, username: &str) -> bool {
            username == self.0.as_str()
        }
    }

    #[test]
    fn username_is_taken() {
        let username_result = Username("taken".to_string(), false);

        assert!(
            username_result
                .check_username_taken(&FakeUsernameCheckService("taken".to_string()))
                .is_err()
        )
    }

    #[test]
    fn username_is_not_taken() {
        let username_result = Username("not_taken".to_string(), false);

        assert!(
            username_result
                .check_username_taken(&FakeUsernameCheckService("taken".to_string()))
                .is_ok()
        )
    }

    #[tokio::test]
    async fn username_is_taken_async() {
        let username_result = Username("taken".to_string(), false);

        assert!(
            username_result
                .check_username_taken_async(&FakeUsernameCheckService("taken".to_string()))
                .await
                .is_err()
        )
    }

    #[tokio::test]
    async fn username_is_not_taken_async() {
        let username_result = Username("not_taken".to_string(), false);

        assert!(
            username_result
                .check_username_taken_async(&FakeUsernameCheckService("taken".to_string()))
                .await
                .is_ok()
        )
    }
}
