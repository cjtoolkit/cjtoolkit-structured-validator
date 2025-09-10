use crate::base::date_time::data::AsDateTimeData;
use crate::base::date_time::rules::{DateTimeMandatoryRules, DateTimeRangeRules};
use crate::common::locale::{ValidateErrorCollector, ValidateErrorStore};
use crate::common::validation_check::ValidationCheck;
use chrono::{DateTime, NaiveDateTime, TimeDelta, TimeZone, Utc};
use std::ops::Add;
use thiserror::Error;

/// Represents a set of rules or constraints for a date-time value.
///
/// # Fields
///
/// * `is_mandatory` - A boolean indicating whether the date-time value is mandatory.
///   - `true`: The date-time value is required.
///   - `false`: The date-time value is optional.
///
/// * `min` - An `Option` that specifies the minimum allowed date-time value.
///   - `Some(DateTime<Utc>)`: The minimum allowed date-time.
///   - `None`: No minimum constraint is applied.
///
/// * `max` - An `Option` that specifies the maximum allowed date-time value.
///   - `Some(DateTime<Utc>)`: The maximum allowed date-time.
///   - `None`: No maximum constraint is applied.
///
/// This struct is useful for validating date-time inputs against specified bounds
/// and determining whether such an input is required.
pub struct DateTimeRules {
    pub is_mandatory: bool,
    pub min: Option<DateTime<Utc>>,
    pub max: Option<DateTime<Utc>>,
}

impl Default for DateTimeRules {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            is_mandatory: true,
            min: Some(now.clone()),
            max: Some(now.clone().add(TimeDelta::days(30))),
        }
    }
}

impl Into<(DateTimeMandatoryRules, DateTimeRangeRules)> for &DateTimeRules {
    fn into(self) -> (DateTimeMandatoryRules, DateTimeRangeRules) {
        (
            DateTimeMandatoryRules {
                is_mandatory: self.is_mandatory,
            },
            DateTimeRangeRules {
                min: self.min.as_ref().map(|min| min.as_date_time_data()),
                max: self.max.as_ref().map(|max| max.as_date_time_data()),
            },
        )
    }
}

impl DateTimeRules {
    fn rules(&self) -> (DateTimeMandatoryRules, DateTimeRangeRules) {
        self.into()
    }

    fn check<Tz: TimeZone>(
        &self,
        messages: &mut ValidateErrorCollector,
        subject: Option<&DateTime<Tz>>,
    ) {
        if !self.is_mandatory && subject.is_none() {
            return;
        }
        let subject = subject.map(|s| s.as_date_time_data());
        let (mandatory_rule, range_rule) = self.rules();
        mandatory_rule.check(messages, subject.as_ref());
        if !messages.is_empty() {
            return;
        }
        range_rule.check(messages, subject.as_ref());
    }
}

/// A custom error type for handling DateTime validation errors.
///
/// This struct is derived from the `Debug`, `Error`, `PartialEq`, `Clone`, and `Default` traits,
/// providing basic functionality for debugging, error handling, equality checks, cloning, and
/// default value creation.
///
/// # Attributes
/// - `0`: A public field of the type `ValidateErrorStore` used to store error details.
///
/// # Error Message
/// This error displays the message `"DateTime Validation Error"` when formatted as a string using
/// the `Display` trait.
#[derive(Debug, Error, PartialEq, Clone, Default)]
#[error("DateTime Validation Error")]
pub struct DateTimeError(pub ValidateErrorStore);

impl ValidationCheck for DateTimeError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

/// A wrapper struct that encapsulates an optional `DateTime` object
/// with an associated `TimeZone`.
///
/// This struct is generic over the `TimeZone` type, allowing flexible
/// time zone handling and operations.
///
/// # Type Parameters
/// - `Tz`: The time zone that implements the `TimeZone` trait. This can either
///   be a fixed offset or a dynamically determined time zone.
///
/// # Fields
/// - `0`: An `Option` holding a `DateTime<Tz>` instance. This can either be:
///   - `Some(DateTime<Tz>)`: When a valid `DateTime` value is present.
///   - `None`: When no `DateTime` value is assigned.
///
/// # Usage
/// The `DateTimeValue` struct is often used to represent an optional
/// `DateTime` value, encapsulating the concept of nullable or absent
/// datetime values with a specific time zone context.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(any(feature = "allow-default-value", test), derive(Default))]
pub struct DateTimeValue<Tz: TimeZone>(Option<DateTime<Tz>>);

impl<Tz: TimeZone> DateTimeValue<Tz> {
    /// Parses a custom datetime object using the provided rules.
    ///
    /// # Arguments
    ///
    /// * `subject` - An `Option` containing a `DateTime<Tz>` that represents the
    ///   datetime to be parsed. If `None`, no datetime is used in parsing.
    /// * `rules` - A `DateTimeRules` object that defines the rules for parsing and
    ///   validation of the datetime.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - Returns an instance of the struct if the `subject` passes the
    ///   rule validation without errors.
    /// * `Err(DateTimeError)` - Returns an error if validation fails or any issues
    ///   arise during rule-checking.
    ///
    /// # Errors
    ///
    /// This function will return errors for any of the following reasons:
    ///
    /// * The provided `rules` are violated.
    /// * Internal validation fails due to invalid input or rule enforcement.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::{DateTime, Utc};
    /// use cjtoolkit_structured_validator::types::times_chrono::date_time::{DateTimeRules, DateTimeValue};
    /// let rules = DateTimeRules::default(); // Assume `DateTimeRules` is constructed
    /// let maybe_date: Option<DateTime<Utc>> = Some(Utc::now());
    ///
    /// match DateTimeValue::parse_custom(maybe_date, rules) {
    ///     Ok(_) => println!("Parsed successfully"),
    ///     Err(e) => println!("Failed to parse: {:?}", e),
    /// }
    /// ```
    pub fn parse_custom(
        subject: Option<DateTime<Tz>>,
        rules: DateTimeRules,
    ) -> Result<Self, DateTimeError> {
        let mut messages = ValidateErrorCollector::new();
        rules.check(&mut messages, subject.as_ref());
        DateTimeError::validate_check(messages)?;
        Ok(Self(subject))
    }

    /// Parses a `NaiveDateTime` with a given timezone and applies custom date-time rules.
    ///
    /// This function takes an optional `NaiveDateTime`, a set of custom date-time parsing rules,
    /// and a timezone. It converts the naive date-time into a timezone-aware `DateTime` and
    /// then parses it based on the provided rules.
    ///
    /// # Arguments
    ///
    /// * `subject` - An `Option<NaiveDateTime>` value that represents the date and time to be parsed.
    ///   If `None`, the function cannot process the input.
    /// * `rules` - A `DateTimeRules` struct which defines the custom rules for parsing the date and time.
    /// * `tz` - A `Tz` instance representing the timezone in which the naive date-time should be interpreted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - If parsing is successful, it returns a parsed instance of the desired type (`Self`).
    /// * `Err(DateTimeError)` - If there is an issue during timezone conversion or parsing,
    ///   it returns a `DateTimeError`.
    ///
    /// # Panics
    ///
    /// This function may panic if the naive date-time does not map to a valid timezone-aware
    /// `DateTime` when used with the provided timezone (`tz`).
    pub fn parse_custom_naive_with_tz(
        subject: Option<NaiveDateTime>,
        rules: DateTimeRules,
        tz: Tz,
    ) -> Result<Self, DateTimeError> {
        let subject = subject.map(|s| s.and_local_timezone(tz).unwrap());
        Self::parse_custom(subject, rules)
    }

    /// Parses an optional `DateTime` object using default date-time rules and returns a `Result`.
    ///
    /// # Parameters
    /// - `subject`: An `Option<DateTime<Tz>>` representing the input date-time to parse.
    ///   - If `Some(DateTime<Tz>)`, attempts to parse the given date-time.
    ///   - If `None`, behavior depends on the implementation of `parse_custom`.
    ///
    /// # Returns
    /// - `Ok(Self)`: If the parsing is successful, returns an instance of the implementing type.
    /// - `Err(DateTimeError)`: If parsing fails, returns an appropriate error.
    ///
    /// # Errors
    /// This function will return an error if parsing the `subject` fails due to invalid input,
    /// or if the `DateTimeRules::default()` do not support the provided date-time format.
    ///
    /// # Example
    /// ```rust
    /// use chrono::{DateTime, Utc};
    /// use cjtoolkit_structured_validator::types::times_chrono::date_time::DateTimeValue;
    /// let datetime: Option<DateTime<Utc>> = Some(Utc::now());
    /// match DateTimeValue::parse(datetime) {
    ///     Ok(_) => println!("Parsed successfully"),
    ///     Err(err) => eprintln!("Failed to parse: {:?}", err),
    /// }
    /// ```
    pub fn parse(subject: Option<DateTime<Tz>>) -> Result<Self, DateTimeError> {
        Self::parse_custom(subject, DateTimeRules::default())
    }

    /// Parses a `NaiveDateTime` with the given timezone to create a `DateTime` instance.
    ///
    /// This function attempts to convert a given `NaiveDateTime` (if provided) into a
    /// timezone-aware `DateTime` using the specified timezone (`tz`). Internally, it delegates
    /// to the `parse_custom_naive_with_tz` method with default rules provided by `DateTimeRules::default()`.
    ///
    /// # Arguments
    ///
    /// * `subject` - An `Option` containing a `NaiveDateTime` which represents the naive (timezone-unaware)
    ///   datetime to be parsed. If `None`, parsing will fail with a relevant error.
    /// * `tz` - The timezone (`Tz`) to be applied to the `NaiveDateTime` to create a timezone-aware `DateTime`.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - If the parsing and timezone application are successful, it returns a `DateTime` instance.
    /// * `Err(DateTimeError)` - If there is an error during parsing, such as an invalid `NaiveDateTime` or
    ///   issues with the provided timezone, a `DateTimeError` is returned.
    ///
    /// # See Also
    ///
    /// * [`parse_custom_naive_with_tz`](self::parse_custom_naive_with_tz) - For specifying custom rules
    ///   during parsing.
    /// * [`DateTimeRules`] - Used to define and configure the default rules for parsing and validation.
    pub fn parse_naive_with_tz(
        subject: Option<NaiveDateTime>,
        tz: Tz,
    ) -> Result<Self, DateTimeError> {
        Self::parse_custom_naive_with_tz(subject, DateTimeRules::default(), tz)
    }

    /// Converts the contained `Option<DateTime<Tz>>` into a cloned `Option<DateTime<Tz>>`.
    ///
    /// # Returns
    /// - `Some(DateTime<Tz>)` if the inner value exists, returning a cloned instance of the `DateTime<Tz>`.
    /// - `None` if the inner value is `None`.
    pub fn as_date_time(&self) -> Option<DateTime<Tz>> {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_custom() {
        let subject = Some(Utc::now().add(TimeDelta::days(1)));
        let rules = DateTimeRules::default();
        let result = DateTimeValue::parse_custom(subject, rules);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_default_err() {
        let result = DateTimeValue::<Utc>::parse(None);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_default_max_min_ok() {
        let subject = Some(Utc::now().add(TimeDelta::days(1)));
        let result = DateTimeValue::parse(subject);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_default_max_err() {
        let subject = Some(Utc::now().add(TimeDelta::days(31)));
        let result = DateTimeValue::parse(subject);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_default_min_err() {
        let subject = Some(Utc::now().add(TimeDelta::days(-1)));
        let result = DateTimeValue::parse(subject);
        assert!(result.is_err());
    }
}
