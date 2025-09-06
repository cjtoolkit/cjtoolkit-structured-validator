//! This module contains structures and traits for working with URLs.

use crate::base::string_rules::StringMandatoryRules;
use crate::common::locale::{
    LocaleData, LocaleMessage, ValidateErrorCollector, ValidateErrorStore,
};
use crate::common::string_validator::{StrValidationExtension, StringValidator};
use crate::common::validation_check::ValidationCheck;
use thiserror::Error;
use url::Url as UrlValue;

/// A structure to define rules or constraints associated with a URL.
///
/// # Fields
///
/// * `is_mandatory` - A boolean field indicating whether the URL is mandatory or optional.
/// When set to `true`, the URL is required; when set to `false`, it is optional.
pub struct UrlRules {
    pub is_mandatory: bool,
}

impl Default for UrlRules {
    fn default() -> Self {
        Self { is_mandatory: true }
    }
}

impl Into<StringMandatoryRules> for &UrlRules {
    fn into(self) -> StringMandatoryRules {
        StringMandatoryRules {
            is_mandatory: self.is_mandatory,
        }
    }
}

impl UrlRules {
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

/// Represents an error that occurs during URL validation.
///
/// This error structure is used to encapsulate validation errors related to URLs.
/// It derives a number of standard traits to provide additional functionality,
/// such as debugging, cloning, and error handling.
///
/// # Derives
/// - `Debug`: Allows formatting the error for debugging purposes.
/// - `Error`: Enables compatibility with the error-handling ecosystem.
/// - `PartialEq`: Allows comparison between `UrlError` instances.
/// - `Clone`: Provides the ability to create a duplicate of the `UrlError`.
/// - `Default`: Provides a default value for `UrlError`.
///
/// # Display
/// The `Display` implementation for this error will output: `"Url Validation Error"`.
///
/// # Fields
/// - `0`: A `ValidateErrorStore` instance, which contains details about the validation errors encountered.
#[derive(Debug, Error, PartialEq, Clone, Default)]
#[error("Url Validation Error")]
pub struct UrlError(pub ValidateErrorStore);

impl ValidationCheck for UrlError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

/// A structure representing a URL with its components.
///
/// The `Url` struct is a tuple struct that encapsulates the following:
/// - A `String` representing the URL.
/// - An optional `UrlValue` that may provide additional information or metadata for the URL.
/// - A `bool` indicating whether the URL is none or not.
///
/// # Attributes
///
/// - `0: String`
///     The core string representation of the URL.
/// - `1: Option<UrlValue>`
///     Represents an optional value associated with the URL, which could provide extra data or configuration.
/// - `2: bool`
///     A boolean flag that determines whether the URL is active.
///
/// # Traits
///
/// - `Debug`
///     The structure implements `Debug` to allow for formatting via the `{:?}` formatter.
/// - `PartialEq`
///     The structure implements `PartialEq` to allow equality comparisons between `Url` instances.
/// - `Clone`
///     The structure implements `Clone` to allow creating an exact copy of the `Url` instance.
///
/// Note: The use of `Option<UrlValue>` assumes that `UrlValue` is defined elsewhere in the codebase or imported appropriately.
#[derive(Debug, PartialEq, Clone)]
pub struct Url(String, Option<UrlValue>, bool);

impl Default for Url {
    fn default() -> Self {
        Self(String::default(), None, true)
    }
}

/// A struct representing a locale associated with URL values.
///
/// The `UrlValueLocale` struct is currently a unit struct,
/// meaning it does not contain any fields or methods. It can
/// serve as a marker, placeholder, or be expanded in the future
/// to include additional functionality or data relevant to
/// managing locales in the context of URL values.
///
/// # Key
/// `validate-invalid-url`
pub struct UrlValueLocale;

impl LocaleMessage for UrlValueLocale {
    fn get_locale_data(&self) -> LocaleData {
        LocaleData::new("validate-invalid-url")
    }
}

impl Url {
    /// Parses a custom URL string based on provided validation rules.
    ///
    /// This method attempts to parse the given optional URL string (`s`) and validate it
    /// according to the specified `rules`. If the input string is invalid or fails validation,
    /// an error is returned. Otherwise, it produces a successfully validated URL representation.
    ///
    /// # Parameters
    ///
    /// - `s`: An `Option` containing a string slice (`&str`) to parse. If `None` is provided as input, the function treats it as an empty string (`""`).
    /// - `rules`: The `UrlRules` instance containing the validation rules/checks to apply to the input.
    ///
    /// # Returns
    ///
    /// - `Ok(Self)`: If the string is successfully parsed and passes all validation checks, a new instance
    ///   of `Self` is returned containing the processed string and additional metadata.
    /// - `Err(UrlError)`: If validation or parsing fails, an `UrlError` containing details about the error is returned.
    ///
    /// # Errors
    ///
    /// - Returns an ` UrlError `:
    ///   - If validation fails based on the provided `rules`.
    ///   - If the input string is not a valid URL.
    ///
    /// # Notes
    ///
    /// - If the input `s` is `None`, it will be treated as an empty string for parsing and validation.
    /// - This function utilizes the `ValidateErrorCollector` to accumulate and handle multiple validation errors if any occur.
    ///
    /// # Examples
    ///
    /// ```
    /// use cjtoolkit_structured_validator::types::url::{UrlRules, Url};
    ///
    /// let rules = UrlRules::default(); // Example rule setup
    /// let input = Some("https://example.com");
    ///
    /// let result = Url::parse_custom(input, rules);
    ///
    /// match result {
    ///     Ok(_) => println!("Parsed successfully"),
    ///     Err(_) => eprintln!("Failed to parse"),
    /// }
    /// ```
    pub fn parse_custom(s: Option<&str>, rules: UrlRules) -> Result<Self, UrlError> {
        let is_none = s.is_none();
        let s = s.unwrap_or_default();
        let subject = s.as_string_validator();
        let mut messages = ValidateErrorCollector::new();
        rules.check(&mut messages, &subject, is_none);
        UrlError::validate_check(messages)?;
        let url = match UrlValue::parse(s) {
            Ok(url) => url,
            Err(_) => {
                let mut messages = ValidateErrorCollector::new();
                messages.push(("Invalid URL".to_string(), Box::new(UrlValueLocale)));
                return Err(UrlError(messages.into()));
            }
        };

        Ok(Self(s.to_string(), Some(url), is_none))
    }

    /// Parses an optional string into a `Self` type, returning a result indicating
    /// either successful parsing or an error.
    ///
    /// # Parameters
    /// - `s`: An `Option<&str>` that represents the input string to be parsed. If `None` is provided,
    ///   the function handles it accordingly as part of the parsing process.
    ///
    /// # Returns
    /// - `Ok(Self)`: If parsing is successful, the function returns an instance of `Self`.
    /// - `Err(UrlError)`: If parsing fails, an error of type `UrlError` is returned.
    ///
    /// # Behavior
    /// This function delegates the parsing process to the `parse_custom` method, using the default
    /// `UrlRules` as the configuration for parsing.
    ///
    /// # Examples
    /// ```rust
    /// use cjtoolkit_structured_validator::types::url::Url;
    /// let url_str = Some("https://example.com");
    /// let parsed_url = Url::parse(url_str);
    /// assert!(parsed_url.is_ok());
    ///
    /// let invalid_url_str = Some("ht@tp://invalid-url");
    /// let invalid_url = Url::parse(invalid_url_str);
    /// assert!(invalid_url.is_err());
    ///
    /// let none_input = None;
    /// let none_result = Url::parse(none_input);
    /// assert!(none_result.is_err()); // Handles `None` gracefully
    /// ```
    ///
    /// # Errors
    /// This function returns an ` UrlError ` if:
    /// - The input string is invalid, according to the parsing rules.
    /// - An empty or `None` input is provided and cannot be parsed.
    ///
    /// # Notes
    /// - This is a high-level parsing function that uses `UrlRules::default()` for parsing logic.
    ///   For more control over parsing behavior, use the `parse_custom` function directly with
    ///   customized `UrlRules`.
    pub fn parse(s: Option<&str>) -> Result<Self, UrlError> {
        Self::parse_custom(s, UrlRules::default())
    }

    /// Retrieves the underlying `UrlValue` if it exists.
    ///
    /// This function attempts to access the `UrlValue` stored within the
    /// current instance and returns it as an `Option<&UrlValue>`.
    ///
    /// # Returns
    ///
    /// - `Some(&UrlValue)`: If the instance contains a `UrlValue`.
    /// - `None`: If no `UrlValue` is present.
    ///
    /// # Notes
    ///
    /// - This function does not modify the state of the instance.
    /// - The returned `UrlValue` is a reference, not an owned value.
    pub fn as_url(&self) -> Option<&UrlValue> {
        self.1.as_ref()
    }

    /// Returns a string slice representation of the inner value.
    ///
    /// # Returns
    /// A string slice (`&str`) of the inner value.
    ///
    /// # Notes
    /// This assumes the inner value is an instance that implements the `as_str`
    /// method, such as `String` or a similarly compatible type.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Converts the current instance into an `Option<Url>`.
    ///
    /// If the internal boolean field (`self.2`) is `true`, this method returns `None`.
    /// Otherwise, it returns `Some(self)`, wrapping the current instance.
    ///
    /// # Returns
    /// - `None` if the internal boolean field is `true`.
    /// - `Some(Url)` if the internal boolean field is `false`.
    pub fn into_option(self) -> Option<Url> {
        if self.2 { None } else { Some(self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_url() {
        let url = Url::parse(Some("https://www.example.com"));
        assert!(url.is_ok());
    }

    #[test]
    fn test_invalid_url() {
        let url = Url::parse(Some("www.example.com"));
        assert!(url.is_err());
    }
}
