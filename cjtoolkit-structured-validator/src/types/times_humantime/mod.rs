use crate::base::date_time::data::AsDateTimeData;
use crate::base::date_time::rules::{DateTimeMandatoryRules, DateTimeRangeRules};
use crate::common::locale::{ValidateErrorCollector, ValidateErrorStore};
use crate::common::validation_check::ValidationCheck;
use humantime::Timestamp;
use std::time::{Duration, SystemTime};
use thiserror::Error;

/// Represents the rules or constraints applied to a date-time field.
///
/// The `DateTimeRules` struct is used to define restrictions on a specific
/// date-time value, allowing for optional enforcement of these rules and
/// specifying minimum and maximum allowable timestamps.
///
/// # Fields
///
/// - `is_mandatory`:
///   A boolean flag indicating whether the date-time value is mandatory.
///   If `true`, the date-time value must be provided. Otherwise, it is optional.
///
/// - `min`:
///   An optional `Timestamp` representing the minimum allowable date-time value.
///   If a value is provided, any date-time before this value is considered invalid.
///   Defaults to `None`, meaning no minimum restriction is enforced.
///
/// - `max`:
///   An optional `Timestamp` representing the maximum allowable date-time value.
///   If a value is provided, any date-time after this value is considered invalid.
///   Defaults to `None`, meaning no maximum restriction is enforced.
///
/// In the above example, the `DateTimeRules` specifies that the date-time is mandatory
/// and must fall within the year 2023.
pub struct DateTimeRules {
    pub is_mandatory: bool,
    pub min: Option<Timestamp>,
    pub max: Option<Timestamp>,
}

impl Default for DateTimeRules {
    fn default() -> Self {
        let now: Timestamp = SystemTime::now().into();
        Self {
            is_mandatory: true,
            min: Some(now.clone()),
            // 30 days from now
            max: now
                .checked_add(Duration::from_secs(30 * 24 * 60 * 60))
                .map(|d| d.into()),
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

    fn check(&self, messages: &mut ValidateErrorCollector, subject: Option<&Timestamp>) {
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

/// Represents an error encountered during DateTime validation.
///
/// This struct is used to encapsulate validation errors specifically
/// associated with DateTime values. It contains a `ValidateErrorStore`,
/// which provides detailed information about the causes of the validation failure.
///
/// The struct derives the following traits:
/// - `Debug`: Enables the struct to be formatted using the `{:?}` formatter.
/// - `Error`: Indicates that this struct represents an error type.
/// - `PartialEq`: Allows comparison between instances for equality.
/// - `Clone`: Enables deep copying of the struct.
/// - `Default`: Allows the creation of the struct with default values.
///
/// Attributes:
/// - `#[derive(Debug, Error, PartialEq, Clone, Default)]`: Automatically implements standard traits for the struct.
/// - `#[error("DateTime Validation Error")]`: Sets the default error message for the `DateTimeError`.
///
/// Fields:
/// - `pub ValidateErrorStore`: A store of validation errors detailing why the DateTime validation failed.
#[derive(Debug, Error, PartialEq, Clone, Default)]
#[error("DateTime Validation Error")]
pub struct DateTimeError(pub ValidateErrorStore);

impl ValidationCheck for DateTimeError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

/// A wrapper around an `Option<Timestamp>` representing a datetime value.
///
/// The `DateTimeValue` struct is a simple abstraction that allows for optional storage
/// of a `Timestamp`. It is useful in scenarios where a timestamp may or may not
/// be present. The struct derives common traits to ensure ease of use in various contexts.
///
/// # Derives
/// - `Debug`: Enables formatting the value using the `{:?}` formatter.
/// - `PartialEq`: Allows comparison of `DateTimeValue` instances for equality.
/// - `Clone`: Enables cloning of `DateTimeValue` instances.
/// - `Default`: Provides a default value which corresponds to `DateTimeValue(None)`.
///
/// # Fields
/// - `0: Option<Timestamp>`: An optional `Timestamp` encapsulated in the `DateTimeValue`.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "allow-default-value", derive(Default))]
pub struct DateTimeValue(Option<Timestamp>);

impl DateTimeValue {
    /// Parses a custom date-time value based on the given rules and an optional timestamp.
    ///
    /// # Parameters
    ///
    /// * `subject` - An `Option<Timestamp>` containing the timestamp to be validated.
    ///   If `None`, the timestamp will be skipped during validation.
    /// * `rules` - A `DateTimeRules` instance containing the rules for validation.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - If the timestamp is valid or the rules are successfully applied.
    /// * `Err(DateTimeError)` - If there are any validation errors detected.
    ///
    /// # Errors
    ///
    /// This function will return a `DateTimeError` if:
    /// - The `rules` fail to validate the given `subject`.
    /// - There are any other validation errors collected by `ValidateErrorCollector`.
    ///
    /// # Notes
    ///
    /// - This function collects validation errors using a `ValidateErrorCollector` instance,
    ///   which aggregates any issues with the given timestamp or rules.
    pub fn parse_custom(
        subject: Option<Timestamp>,
        rules: DateTimeRules,
    ) -> Result<Self, DateTimeError> {
        let mut messages = ValidateErrorCollector::new();
        rules.check(&mut messages, subject.as_ref());
        DateTimeError::validate_check(messages)?;
        Ok(Self(subject))
    }

    /// Parses an optional `Timestamp` into a `Self` instance using the default `DateTimeRules`.
    ///
    /// # Parameters
    /// - `subject`: An `Option<Timestamp>` representing the timestamp to be parsed. If `None`,
    ///   the parsing operation will handle the absence accordingly.
    ///
    /// # Returns
    /// - `Ok(Self)`: On successful parsing, returns an instance of the type implementing this function.
    /// - `Err(DateTimeError)`: Returns an error if parsing fails.
    ///
    /// # Errors
    /// This function will return a `DateTimeError` if the `subject` cannot be parsed
    /// successfully based on the default `DateTimeRules`.
    ///
    /// # Notes
    /// This function delegates the parsing task to `Self::parse_custom` while utilizing
    /// the default rules provided by `DateTimeRules::default()`.
    pub fn parse(subject: Option<Timestamp>) -> Result<Self, DateTimeError> {
        Self::parse_custom(subject, DateTimeRules::default())
    }

    /// Converts the current object into an `Option<Timestamp>`.
    ///
    /// This method returns a cloned version of the inner `Timestamp` wrapped in an `Option`.
    /// If the current object does not have an inner value, it will return `None`.
    ///
    /// # Returns
    /// - `Some(Timestamp)` if the inner value exists.
    /// - `None` if the inner value does not exist.
    pub fn as_timestamp(&self) -> Option<Timestamp> {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::{Add, Sub};

    #[test]
    fn test_parse_custom() {
        let rules = DateTimeRules::default();
        let now: Timestamp = SystemTime::now().add(Duration::from_secs(10)).into();
        let subject = Some(now);
        let result = DateTimeValue::parse_custom(subject, rules);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_default_err() {
        let subject = None;
        let result = DateTimeValue::parse(subject);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_default_ok() {
        let subject = Some(SystemTime::now().add(Duration::from_secs(10)).into());
        let result = DateTimeValue::parse(subject);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_default_max_err() {
        let subject = Some(
            SystemTime::now()
                .add(Duration::from_secs(31 * 24 * 60 * 60))
                .into(),
        );
        let result = DateTimeValue::parse(subject);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_default_min_err() {
        let subject = Some(SystemTime::now().sub(Duration::from_secs(10)).into());
        let result = DateTimeValue::parse(subject);
        assert!(result.is_err());
    }
}
