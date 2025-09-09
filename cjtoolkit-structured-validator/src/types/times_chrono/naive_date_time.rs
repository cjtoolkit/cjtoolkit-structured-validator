use crate::base::date_time::data::AsDateTimeData;
use crate::base::date_time::rules::{DateTimeMandatoryRules, DateTimeRangeRules};
use crate::common::locale::{ValidateErrorCollector, ValidateErrorStore};
use crate::common::validation_check::ValidationCheck;
use chrono::{NaiveDateTime, TimeDelta, Utc};
use std::ops::Add;
use thiserror::Error;

/// A struct that defines validation rules for a `NaiveDateTime`.
///
/// This struct is used to impose constraints on a `NaiveDateTime`,
/// such as whether the field is mandatory, and optional minimum
/// or maximum bounds for the date and time value.
///
/// # Fields
///
/// * `is_mandatory` - A boolean flag that specifies whether
///   the `NaiveDateTime` is required. If set to `true`, the
///   user must provide a value.
///
/// * `min` - An optional `NaiveDateTime` value representing
///   the lower bound for the allowable datetime. If `Some`,
///   the given datetime must not be earlier than this value.
///   If `None`, no minimum constraint is applied.
///
/// * `max` - An optional `NaiveDateTime` value representing
///   the upper bound for the allowable datetime. If `Some`,
///   the given datetime must not be later than this value.
///   If `None`, no maximum constraint is applied.
pub struct NaiveDateTimeRules {
    pub is_mandatory: bool,
    pub min: Option<NaiveDateTime>,
    pub max: Option<NaiveDateTime>,
}

impl Default for NaiveDateTimeRules {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            is_mandatory: true,
            min: Some(now.clone().naive_utc()),
            max: Some(now.clone().naive_utc().add(TimeDelta::days(30))),
        }
    }
}

impl NaiveDateTimeRules {
    fn rules(&self, date_format: Option<&str>) -> (DateTimeMandatoryRules, DateTimeRangeRules) {
        (
            DateTimeMandatoryRules {
                is_mandatory: self.is_mandatory,
            },
            DateTimeRangeRules {
                min: self
                    .min
                    .as_ref()
                    .map(|min| (date_format.clone(), min).as_date_time_data()),
                max: self
                    .max
                    .as_ref()
                    .map(|max| (date_format.clone(), max).as_date_time_data()),
            },
        )
    }

    fn check(
        self,
        subject: Option<&NaiveDateTime>,
        messages: &mut ValidateErrorCollector,
        date_format: Option<&str>,
    ) {
        if !self.is_mandatory && subject.is_none() {
            return;
        }
        let subject = subject.map(|s| (date_format.clone(), s).as_date_time_data());
        let (mandatory_rule, range_rule) = self.rules(date_format);
        mandatory_rule.check(messages, subject.as_ref());
        if !messages.is_empty() {
            return;
        }
        range_rule.check(messages, subject.as_ref());
    }
}

/// A custom error type used to represent validation errors related to `NaiveDateTime`.
///
/// This struct encapsulates a `ValidateErrorStore`, which contains details about
/// the validation errors, making it easier to track and debug issues during
/// the validation of `NaiveDateTime` values.
///
/// # Derive Traits
/// - `Debug`: Allows instances of this error to be formatted using the `Debug` formatter.
/// - `Error`: Marks this struct as an implementation of Rust's standard `Error` trait.
/// - `PartialEq`: Enables equality comparisons between different instances of the error.
/// - `Clone`: Allows duplication of this error structure.
/// - `Default`: Provides a default constructor for creating an instance with default values.
///
/// # Error Display
/// The `#[error("NaiveDateTime Validation Error")]` is a part of the `thiserror` crate,
/// which provides a custom error message when this error is displayed.
///
/// # Fields
/// - `pub ValidateErrorStore`: A data structure that holds the details of validation errors.
#[derive(Debug, Error, PartialEq, Clone, Default)]
#[error("NaiveDateTime Validation Error")]
pub struct NaiveDateTimeError(pub ValidateErrorStore);

impl ValidationCheck for NaiveDateTimeError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

/// `NaiveDateTimeValue` is a wrapper struct for an `Option<NaiveDateTime>`,
/// allowing for easier handling of nullable or optional `NaiveDateTime` values.
///
/// This struct derives the following traits:
/// - `Debug`: Allows formatting the value using the `{:?}` formatter.
/// - `PartialEq`: Enables equality and inequality comparisons between instances of `NaiveDateTimeValue`.
/// - `Clone`: Provides support for producing a copy of the struct and its inner value.
/// - `Default`: Supplies a default empty (None) value for the struct when explicitly required.
///
/// # Fields
/// - `0`: An `Option<NaiveDateTime>` where the value is either:
///     - `None`: Represents the absence of a `NaiveDateTime`.
///     - `Some(value)`: Represents a wrapped `NaiveDateTime`.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct NaiveDateTimeValue(Option<NaiveDateTime>);

impl NaiveDateTimeValue {
    /// Parses a `NaiveDateTime` object with the provided rules and an optional custom format.
    ///
    /// # Parameters
    /// - `subject`: An `Option<NaiveDateTime>` representing the date and time to be parsed.
    ///   If `None`, no date or time is provided for validation.
    /// - `rules`: A `NaiveDateTimeRules` struct that defines the validation rules to
    ///   be applied to the provided `NaiveDateTime`.
    /// - `format`: An `Option<&str>` representing the custom format to use during the
    ///   parsing and validation process. If `None`, a default format may be applied
    ///   as determined by the `rules`.
    ///
    /// # Returns
    /// - `Ok(Self)`: If the `subject` passes the validation checks specified by the `rules`
    ///   and conforms to the optional `format`, the function returns an instance of the `Self`
    ///   type wrapping the validated `NaiveDateTime`.
    /// - `Err(NaiveDateTimeError)`: If any validation checks fail or there is an issue with
    ///   the format, a `NaiveDateTimeError` is returned detailing the errors.
    ///
    /// # Errors
    /// - Returns `NaiveDateTimeError` if the `subject` does not conform to the provided
    ///   `rules` or the specified `format` results in a validation failure.
    ///
    /// # Example
    /// ```rust
    /// use chrono::Utc;
    /// use cjtoolkit_structured_validator::types::times_chrono::naive_date_time::{NaiveDateTimeRules, NaiveDateTimeValue};
    /// let rules = NaiveDateTimeRules::default();
    /// let subject = Some(Utc::now().naive_utc());
    /// let format = Some("%Y-%m-%d %H:%M:%S");
    ///
    /// let parsed = NaiveDateTimeValue::parse_custom_with_format(subject, rules, format);
    /// match parsed {
    ///     Ok(valid_date) => println!("Parsed date: {:?}", valid_date),
    ///     Err(e) => println!("Failed to parse date: {:?}", e),
    /// }
    /// ```
    pub fn parse_custom_with_format(
        subject: Option<NaiveDateTime>,
        rules: NaiveDateTimeRules,
        format: Option<&str>,
    ) -> Result<Self, NaiveDateTimeError> {
        let mut messages = ValidateErrorCollector::new();
        rules.check(subject.as_ref(), &mut messages, format);
        NaiveDateTimeError::validate_check(messages)?;
        Ok(Self(subject))
    }

    /// Parses a `NaiveDateTime` from a given `subject` using provided custom `rules`.
    ///
    /// This function allows you to parse a `NaiveDateTime` object based on custom-defined
    /// rules (`NaiveDateTimeRules`). It acts as a wrapper for the more granular
    /// `parse_custom_with_format` function but does not include an explicit format,
    /// instead opting for the default behavior.
    ///
    /// # Arguments
    ///
    /// * `subject` - An `Option<NaiveDateTime>` representing the date and time to be parsed.
    ///               If `None`, the function may consider the parsing invalid depending on
    ///               the provided rules.
    /// * `rules` - A `NaiveDateTimeRules` object that defines the parsing logic or
    ///             constraints to validate or adjust the `subject`.
    ///
    /// # Returns
    ///
    /// A `Result<Self, NaiveDateTimeError>`:
    ///
    /// * On success, returns an instance of the type implementing this method (`Self`),
    ///   representing the parsed `NaiveDateTime` object.
    /// * On failure, returns a `NaiveDateTimeError` indicating the nature of the parsing
    ///   failure, such as invalid inputs or rule mismatches.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::Utc;
    /// use cjtoolkit_structured_validator::types::times_chrono::naive_date_time::{NaiveDateTimeRules, NaiveDateTimeValue};
    /// let datetime = Some(Utc::now().naive_utc());
    /// let rules = NaiveDateTimeRules::default();
    /// let parsed = NaiveDateTimeValue::parse_custom(datetime, rules);
    /// match parsed {
    ///     Ok(result) => println!("Parsed successfully: {:?}", result),
    ///     Err(err) => eprintln!("Failed to parse: {:?}", err),
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// For advanced customization, such as using a specific date-time format,
    /// consider using the `parse_custom_with_format` function directly.
    ///
    /// # Errors
    ///
    /// This function will return a `NaiveDateTimeError` if the `subject` is invalid,
    /// does not conform to the `rules`, or fails parsing for any other reason.
    pub fn parse_custom(
        subject: Option<NaiveDateTime>,
        rules: NaiveDateTimeRules,
    ) -> Result<Self, NaiveDateTimeError> {
        Self::parse_custom_with_format(subject, rules, None)
    }

    /// Attempts to parse an optional `NaiveDateTime` value using the default rules defined in
    /// `NaiveDateTimeRules`.
    ///
    /// # Parameters
    /// - `subject`: An `Option<NaiveDateTime>` that represents the date-time value to be parsed. If
    ///   `None`, parsing will likely return an error based on the custom logic within
    ///   `Self::parse_custom`.
    ///
    /// # Returns
    /// - `Result<Self, NaiveDateTimeError>`: On success, returns an instance of `Self` parsed
    ///   from the given `subject` using the default rules. On failure, returns a
    ///   `NaiveDateTimeError` indicating the nature of the parsing failure.
    ///
    /// # Errors
    /// This method will return a `NaiveDateTimeError` if:
    /// - The `subject` is `None` and cannot be parsed.
    /// - The given `subject` does not adhere to the default `NaiveDateTimeRules`.
    ///
    /// # Notes
    /// - This method delegates the parsing logic to the private `parse_custom` method, using
    ///   the default implementation of `NaiveDateTimeRules`.
    /// - Ensure that the `subject` value, if present, is well-formed to avoid parsing errors.
    pub fn parse(subject: Option<NaiveDateTime>) -> Result<Self, NaiveDateTimeError> {
        Self::parse_custom(subject, NaiveDateTimeRules::default())
    }

    /// Parses a given `NaiveDateTime` value using an optional custom format and returns the parsed result.
    ///
    /// # Parameters
    ///
    /// - `subject`:
    ///   An `Option<NaiveDateTime>` representing the datetime value to be parsed. If `None`, the parsing may fail
    ///   depending on the implementation of the `parse_custom_with_format` function.
    ///
    /// - `format`:
    ///   An `Option<&str>` representing the custom format string to use for parsing. If `None`, a default format
    ///   may be used or the parsing might fail depending on the rules defined in `NaiveDateTimeRules::default()`.
    ///
    /// # Returns
    ///
    /// - `Ok(Self)`:
    ///   If the parsing operation is successful, returns an instance of `Self` wrapping the parsed value.
    ///
    /// - `Err(NaiveDateTimeError)`:
    ///   If the parsing operation fails, returns an error of type `NaiveDateTimeError`.
    ///
    /// # Errors
    ///
    /// - This function returns `NaiveDateTimeError` when parsing fails due to an invalid `subject`, incorrect `format`,
    ///   or if the provided format doesn't match the rules expected by the `NaiveDateTimeRules::default()`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::Utc;
    /// use cjtoolkit_structured_validator::types::times_chrono::naive_date_time::NaiveDateTimeValue;
    /// let datetime = Some(Utc::now().naive_utc());
    /// let custom_format = Some("%Y-%m-%d %H:%M:%S");
    /// let result = NaiveDateTimeValue::parse_with_format(datetime, custom_format);
    /// match result {
    ///     Ok(parsed) => println!("Parsed successfully: {:?}", parsed),
    ///     Err(e) => println!("Failed to parse: {:?}", e),
    /// }
    /// ```
    ///
    /// This function delegates the actual parsing logic to `Self::parse_custom_with_format` and uses the
    /// default parsing rules (`NaiveDateTimeRules::default()`) unless specified otherwise in the custom implementation.
    pub fn parse_with_format(
        subject: Option<NaiveDateTime>,
        format: Option<&str>,
    ) -> Result<Self, NaiveDateTimeError> {
        Self::parse_custom_with_format(subject, NaiveDateTimeRules::default(), format)
    }

    /// Converts the current value into an `Option<NaiveDateTime>` if applicable.
    ///
    /// # Returns
    ///
    /// - Returns `Some(NaiveDateTime)` if the inner value of the struct is set (`self.0`).
    /// - Returns `None` if the inner value is not set (`self.0` is not present or invalid).
    ///
    /// # Note
    ///
    /// This function clones the inner value `self.0` if it exists. Ensure
    /// that cloning is efficient and does not cause unnecessary overhead when dealing
    /// with repeated calls.
    pub fn as_naive_date_time(&self) -> Option<NaiveDateTime> {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_custom() {
        let subject = Some(Utc::now().naive_utc().add(TimeDelta::days(1)));
        let rules = NaiveDateTimeRules::default();
        let result = NaiveDateTimeValue::parse_custom(subject, rules);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_default_err() {
        let result = NaiveDateTimeValue::parse(None);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_default_max_min_ok() {
        let subject = Some(Utc::now().naive_utc().add(TimeDelta::days(1)));
        let result = NaiveDateTimeValue::parse(subject);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_default_max_err() {
        let subject = Some(Utc::now().naive_utc().add(TimeDelta::days(31)));
        let result = NaiveDateTimeValue::parse(subject);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_default_min_err() {
        let subject = Some(Utc::now().naive_utc().add(TimeDelta::days(-1)));
        let result = NaiveDateTimeValue::parse(subject);
        assert!(result.is_err());
    }
}
