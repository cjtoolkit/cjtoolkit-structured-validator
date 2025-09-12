//! This module contains structures and traits for working with names.
//!
//! The `Name` type is a tuple struct consisting of a `String` value and a `bool` flag.
//! The `String` value represents the name, and the boolean flag can be used for additional
//! semantics or functionality (e.g., marking a name as "active" or "enabled").
//!
//! The `NameRules` type defines the validation rules for a name field. It is used to
//! define whether the name field is mandatory and the permissible length range.
//!
//! The `NameError` type is used to encapsulate validation errors specific to names

use crate::base::string_rules::{StringLengthRules, StringMandatoryRules};
use crate::common::locale::{ValidateErrorCollector, ValidateErrorStore};
use crate::common::string_validator::{StrValidationExtension, StringValidator};
use crate::common::validation_check::ValidationCheck;
use thiserror::Error;

/// A structure representing the rules and constraints associated with a name field.
///
/// The `NameRules` struct is used to define validation rules for a name, including whether
/// the name is mandatory and the permissible length range.
///
/// # Fields
///
/// * `is_mandatory` (`bool`):
///   A boolean value indicating whether the name field is required (`true`) or optional (`false`).
///
/// * `min_length` (`Option<usize>`):
///   An optional field specifying the minimum allowable length for the name.
///   If it is `Some(value)`, the name must have at least `value` characters. If it is `None`,
///   no minimum length is enforced.
///
/// * `max_length` (`Option<usize>`):
///   An optional field specifying the maximum allowable length for the name.
///   If it is `Some(value)`, the name must not exceed `value` characters. If it is `None`,
///   no maximum length is enforced.
pub struct NameRules {
    pub is_mandatory: bool,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

impl Default for NameRules {
    fn default() -> Self {
        Self {
            is_mandatory: true,
            min_length: Some(5),
            max_length: Some(20),
        }
    }
}

impl Into<(StringMandatoryRules, StringLengthRules)> for &NameRules {
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

impl NameRules {
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

/// A custom error type that represents validation errors when processing names.
///
/// This error type is part of domain-specific validation and is used to encapsulate
/// detailed validation errors encountered while validating name-related fields. It
/// provides meaningful context around the failure to help in debugging and error handling.
///
/// # Derives
/// - `Debug`: Allows the error to be formatted using the `{:?}` formatter.
/// - `Error`: Implements the `std::error::Error` trait for compatibility with error handling frameworks.
/// - `PartialEq`: Enables equality comparisons for instances of `NameError`.
/// - `Clone`: Allows deep cloning of the `NameError` type.
/// - `Default`: Provides a way to create a default instance of `NameError`.
///
/// # Fields
/// - `pub ValidateErrorStore`: Encapsulates a collection of validation errors related
///   to name validation. This field is represented by a `ValidateErrorStore` structure.
///
/// # Error Message
/// The `NameError` type will return the error string `"Name Validation Error"` when
/// formatted as a string (e.g., using `error.to_string()`).
///
/// # Usage
/// This struct is typically used in the context of validating user input or data processing
/// where names need to conform to specific constraints.
#[derive(Debug, Error, PartialEq, Clone, Default)]
#[error("Name Validation Error")]
pub struct NameError(pub ValidateErrorStore);

impl ValidationCheck for NameError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

impl Into<ValidateErrorStore> for &NameError {
    fn into(self) -> ValidateErrorStore {
        self.0.clone()
    }
}

/// A structure representing a name with an associated boolean flag.
///
/// The `Name` struct consists of a `String` value and a `bool` flag.
/// The `String` value represents the name
///
/// # Derive Attributes:
/// - `Debug`: Enables formatting the `Name` struct using the `{:?}` formatter for debugging purposes.
/// - `PartialEq`: Allows comparison of two `Name` instances to check for equality.
/// - `Clone`: Enables creating a clone (deep copy) of a `Name` instance.
///
/// # Fields:
/// - `0: String` - The name represented as a string.
/// - `1: bool` - A boolean flag associated with the name, none if `true`, otherwise `false`
#[derive(Debug, PartialEq, Clone)]
pub struct Name(String, bool);

#[cfg(any(feature = "allow-default-value", test))]
impl Default for Name {
    fn default() -> Self {
        Self(String::new(), true)
    }
}

impl Name {
    /// Parses a custom name string based on the provided validation rules.
    ///
    /// # Parameters
    /// - `s`: An `Option<&str>` that represents the input name string to be parsed.
    ///   - If `None`, it will be treated as an empty string (`""`).
    /// - `rules`: A `NameRules` instance containing the validation rules to be applied for the input name string.
    ///
    /// # Returns
    /// - `Ok(Self)`: A successfully parsed and validated name, represented by an instance of `Self`.
    ///   - The resulting instance includes the parsed name string and a flag indicating whether the original input was `None`.
    /// - `Err(NameError)`: Returns a `NameError` if the input name fails validation based on the provided rules.
    ///
    /// # Errors
    /// - If validation fails based on the rules provided by the `NameRules` instance, this function will return a `NameError` with details about the failure.
    ///
    /// # Example
    /// ```
    /// use cjtoolkit_structured_validator::types::name::{NameRules, Name};
    ///
    /// let rules = NameRules::default();
    /// let result = Name::parse_custom(Some("ValidName"), rules);
    ///
    /// match result {
    ///     Ok(_) => println!("Parsed name"),
    ///     Err(_) => eprintln!("Failed to parse name"),
    /// }
    /// ```
    ///
    /// # Implementation Notes
    /// - If the input `Option<&str>` is `None`, it defaults to an empty string (`""`) for validation.
    /// - Validation errors are collected using `ValidateErrorCollector` and checked against the rules.
    /// - A `Self` instance is created with the parsed string and whether the input was `None`.
    pub fn parse_custom(s: Option<&str>, rules: NameRules) -> Result<Self, NameError> {
        let is_none = s.is_none();
        let s = s.unwrap_or_default();
        let subject = s.as_string_validator();
        let mut messages = ValidateErrorCollector::new();
        rules.check(&mut messages, &subject, is_none);
        NameError::validate_check(messages)?;
        Ok(Self(s.to_string(), is_none))
    }

    /// Parses the given optional string reference into an instance of `Self` using the default
    /// `NameRules`.
    ///
    /// # Arguments
    ///
    /// * `s` - An `Option` containing a string slice to be parsed. If `None`, the parsing will
    ///         handle it accordingly, depending on the implementation logic.
    ///
    /// # Returns
    ///
    /// * `Result<Self, NameError>` - On success, this function returns an instance of `Self`.
    ///                                On failure, it returns a `NameError` indicating the issue
    ///                                encountered during parsing.
    ///
    /// # Behavior
    ///
    /// This function delegates the parsing logic to `Self::parse_custom`, using the default
    /// `NameRules` configuration. It provides a simplified interface for parsing when custom
    /// rules are not required.
    ///
    /// # Errors
    ///
    /// This function can return a `NameError` if the input string does not satisfy the expected
    /// format or validation rules defined in the `NameRules`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cjtoolkit_structured_validator::types::name::Name;
    /// match Name::parse(Some("ExampleName")) {
    ///     Ok(_) => println!("Parsed successfully"),
    ///     Err(_) => eprintln!("Failed to parse"),
    /// }
    /// ```
    ///
    /// This example demonstrates how to parse a string into `MyType`, handling both success
    /// and failure cases appropriately.
    pub fn parse(s: Option<&str>) -> Result<Self, NameError> {
        Self::parse_custom(s, NameRules::default())
    }

    /// Returns a string slice (`&str`) reference to the underlying string.
    ///
    /// # Returns
    ///
    /// A `&str` slice referencing the internal string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Converts the current instance into an `Option<Name>`.
    ///
    /// # Returns
    ///
    /// - Returns `None` if the second field in the tuple (`self.1`) is `true`.
    /// - Returns `Some(self)` if the second field in the tuple (`self.1`) is `false`.
    ///
    /// # Note
    ///
    /// This function assumes the type `Name` is the same as the type of `self` and
    /// that `self` is a tuple-like structure where the second element (`self.1`) is
    /// a boolean value used as the condition for determining a result.
    pub fn into_option(self) -> Option<Name> {
        if self.1 { None } else { Some(self) }
    }
}

pub mod name_alias {
    use super::*;

    pub type TitleRules = NameRules;
    pub type TitleError = NameError;
    pub type Title = Name;

    pub type FirstNameRules = NameRules;
    pub type FirstNameError = NameError;
    pub type FirstName = Name;

    pub type ForeNameRules = NameRules;
    pub type ForeNameError = NameError;
    pub type ForeName = Name;

    pub type MiddleNameRules = NameRules;
    pub type MiddleNameError = NameError;
    pub type MiddleName = Name;

    pub type LastNameRules = NameRules;
    pub type LastNameError = NameError;
    pub type LastName = Name;

    pub type AddressLineRules = NameRules;
    pub type AddressLineError = NameError;
    pub type AddressLine = Name;

    pub type FieldRules = NameRules;
    pub type FieldError = NameError;
    pub type Field = Name;
}
