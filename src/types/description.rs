//! This module contains structures and traits for working with text-based descriptions.

use crate::base::string_rules::{StringLengthRules, StringMandatoryRules};
use crate::common::locale::{ValidateErrorCollector, ValidateErrorStore};
use crate::common::string_validator::{StrValidationExtension, StringValidator};
use crate::common::validation_check::ValidationCheck;
use thiserror::Error;

/// A struct representing the rules for a description field.
///
/// This struct defines constraints and requirements for a text-based description,
/// such as whether it is mandatory, and its minimum and maximum length.
///
/// # Fields
///
/// * `is_mandatory` (`bool`): Indicates whether the description field is mandatory.
///   - `true`: The description is required.
///   - `false`: The description is optional.
///
/// * `min_length` (`Option<usize>`): The minimum allowable length for the description.
///   - `Some(usize)`: The minimum length is specified.
///   - `None`: No minimum length is enforced.
///
/// * `max_length` (`Option<usize>`): The maximum allowable length for the description.
///   - `Some(usize)`: The maximum length is specified.
///   - `None`: No maximum length is enforced.
pub struct DescriptionRules {
    pub is_mandatory: bool,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

impl Default for DescriptionRules {
    fn default() -> Self {
        Self {
            is_mandatory: true,
            min_length: None,
            max_length: Some(40),
        }
    }
}

impl Into<(StringMandatoryRules, StringLengthRules)> for &DescriptionRules {
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

impl DescriptionRules {
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

/// A struct representing a validation error for descriptions.
///
/// This struct implements the `Debug`, `Error`, `PartialEq`, `Clone`, and `Default` traits,
/// and it is used to handle validation errors specifically related to descriptions.
///
/// # Attributes
/// * `ValidateErrorStore` - A type encapsulating the store of validation errors.
#[derive(Debug, Error, PartialEq, Clone, Default)]
#[error("Description Validation Error")]
pub struct DescriptionError(pub ValidateErrorStore);

impl ValidationCheck for DescriptionError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Description(String, bool);

impl Default for Description {
    fn default() -> Self {
        Self(String::new(), true)
    }
}

impl Description {
    /// Parses a custom input string according to specified validation rules and returns a
    /// structured description, or an error if the input violates the rules.
    ///
    /// # Parameters
    ///
    /// * `s` - An `Option<&str>` input string to parse. If `None`, the function assumes
    ///          a default empty string (`""`).
    /// * `rules` - A `DescriptionRules` object defining the validation logic to apply to the input.
    ///
    /// # Returns
    ///
    /// * `Result<Self, DescriptionError>` -
    ///     - `Ok(Self)` - If the input string passes validation, it returns a `Self` instance
    ///       containing the processed string and a boolean indicating whether the input was `None`.
    ///     - `Err(DescriptionError)` - If validation errors are encountered, a `DescriptionError`
    ///       containing the collected errors is returned.
    ///
    /// # Behavior
    ///
    /// 1. Checks if the provided input string (`s`) is `None`. If it is `None`, treats the input as
    ///    an empty string (`""`).
    /// 2. Validates the string using the `as_string_validator()` method.
    /// 3. Collects validation errors using a `ValidateErrorCollector`.
    /// 4. Applies the provided `rules` via the `check` method to validate the string and records
    ///    any issues with the `ValidateErrorCollector`.
    /// 5. If validation errors were collected by the rules, a `DescriptionError` is returned.
    /// 6. If no validation errors are found, returns a successfully parsed `Self` object with
    ///    the input string and a flag indicating whether it originated from a `None` value.
    ///
    /// # Errors
    ///
    /// Returns a `DescriptionError` if the input string violates any of the rules defined in
    /// `DescriptionRules`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cjtoolkit_structured_validator::types::description::{DescriptionRules, Description};
    /// let rules = DescriptionRules::default();
    /// let result = Description::parse_custom(Some("valid description"), rules);
    /// assert!(result.is_ok());
    ///
    /// let rules = DescriptionRules::default();
    /// let result_with_none = Description::parse_custom(None, rules);
    /// assert!(result_with_none.is_err());
    /// ```
    pub fn parse_custom(
        s: Option<&str>,
        rules: DescriptionRules,
    ) -> Result<Self, DescriptionError> {
        let is_none = s.is_none();
        let s = s.unwrap_or_default();
        let subject = s.as_string_validator();
        let mut messages = ValidateErrorCollector::new();
        rules.check(&mut messages, &subject, is_none);
        DescriptionError::validate_check(messages)?;
        Ok(Self(s.to_string(), is_none))
    }

    /// Parses an optional string slice into an instance of the implementing type, utilizing the default parsing rules.
    ///
    /// # Arguments
    ///
    /// * `s` - An optional string slice that may contain the value to be parsed.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - If the parsing succeeds and the string conforms to the expected format.
    /// * `Err(DescriptionError)` - If the parsing fails due to invalid input or does not meet the required rules.
    ///
    /// # Behavior
    ///
    /// This function uses the default `DescriptionRules` when parsing the provided string. If `s` is `None`,
    /// it depends on the implementing type's behavior when handling `None` cases (e.g., returning a default value
    /// or producing an error).
    ///
    /// For custom parsing rules, use the `parse_custom` method directly.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cjtoolkit_structured_validator::types::description::{Description};
    /// let result = Description::parse(Some("example input"));
    /// match result {
    ///     Ok(_) => println!("Parsed successfully"),
    ///     Err(_) => println!("Failed to parse"),
    /// }
    /// ```
    pub fn parse(s: Option<&str>) -> Result<Self, DescriptionError> {
        Self::parse_custom(s, DescriptionRules::default())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_option(self) -> Option<Description> {
        if self.1 { None } else { Some(self) }
    }
}

pub mod description_alias {
    use super::*;

    pub type TextRules = DescriptionRules;
    pub type TextError = DescriptionError;
    pub type Text = Description;

    pub type BodyRules = DescriptionRules;
    pub type BodyError = DescriptionError;
    pub type Body = Description;

    pub type SummaryRules = DescriptionRules;
    pub type SummaryError = DescriptionError;
    pub type Summary = Description;

    pub type IngredientsRules = DescriptionRules;
    pub type IngredientsError = DescriptionError;
    pub type Ingredients = Description;
}
