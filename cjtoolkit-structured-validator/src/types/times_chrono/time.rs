use crate::base::date_time::data::AsDateTimeData;
use crate::base::date_time::rules::{DateTimeMandatoryRules, DateTimeRangeRules};
use crate::common::locale::{ValidateErrorCollector, ValidateErrorStore};
use crate::common::validation_check::ValidationCheck;
use chrono::NaiveTime;
use thiserror::Error;

/// A struct that defines time-based rules and constraints for a certain operation or entity.
///
/// # Fields
///
/// * `is_mandatory`
///   - A boolean flag indicating whether adhering to the time rules is mandatory.
///   - `true` if the rules are strict and must be followed, `false` if otherwise.
///
/// * `min`
///   - An `Option<NaiveTime>` specifying the minimum allowable time.
///   - If `Some(NaiveTime)`, it represents the earliest valid time.
///   - If `None`, there is no minimum time constraint.
///
/// * `max`
///   - An `Option<NaiveTime>` specifying the maximum allowable time.
///   - If `Some(NaiveTime)`, it represents the latest valid time.
///   - If `None`, there is no maximum time constraint.
///
/// # Example
///
/// ```
/// use chrono::NaiveTime;
/// use cjtoolkit_structured_validator::types::times_chrono::time::TimeRules;
///
/// let time_rules = TimeRules {
///     is_mandatory: true,
///     min: Some(NaiveTime::from_hms_opt(9, 0, 0).unwrap_or_default()), // The earliest allowed time is 9:00 AM
///     max: Some(NaiveTime::from_hms_opt(17, 0, 0).unwrap_or_default()), // The latest allowed time is 5:00 PM
/// };
/// ```
///
/// This struct can be used to enforce time range policies for various contexts, such as
/// scheduling tasks or validating user input within a specific time interval.
pub struct TimeRules {
    pub is_mandatory: bool,
    pub min: Option<NaiveTime>,
    pub max: Option<NaiveTime>,
}

impl Default for TimeRules {
    fn default() -> Self {
        Self {
            is_mandatory: true,
            min: Some(NaiveTime::from_hms_opt(9, 0, 0).unwrap_or_default()),
            max: Some(NaiveTime::from_hms_opt(17, 0, 0).unwrap_or_default()),
        }
    }
}

impl TimeRules {
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
        subject: Option<&NaiveTime>,
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
        range_rule.check_time(messages, subject.as_ref());
    }
}

/// A custom error type for handling time validation errors.
///
/// The `TimeError` struct represents an error that occurs
/// during the validation of time-related data. This struct wraps
/// the `ValidateErrorStore` type, which is used to store and manage
/// detailed validation errors.
///
/// # Attributes
/// * `0` - An instance of `ValidateErrorStore` that contains detailed
/// validation error information.
///
/// # Derives
/// * `Debug` - Allows the `TimeError` struct to be formatted using the `fmt::Debug` trait.
/// * `Error` - Makes `TimeError` compliant with the `thiserror` crate's `Error` trait,
/// enabling it to be used as a standard error type.
/// * `PartialEq` - Enables comparison between two `TimeError` instances for equality.
/// * `Clone` - Allows `TimeError` instances to be cloned.
/// * `Default` - Provides a default implementation for the `TimeError` struct.
///
/// # Display
/// When formatted or displayed as a string, `TimeError` will output:
/// `Time Validation Error`.
///
/// # Usage
/// This error can be used in validation routines to represent and propagate
/// time-related validation issues.
#[derive(Debug, Error, PartialEq, Clone, Default)]
#[error("Time Validation Error")]
pub struct TimeError(pub ValidateErrorStore);

impl ValidationCheck for TimeError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

/// A wrapper struct for representing an optional `NaiveTime` value.
///
/// The `TimeValue` struct provides a convenient way to handle optional time values
/// using a `chrono::NaiveTime` internally. It supports debug formatting, partial
/// equality comparison, cloning, and provides a default value.
///
/// ## Fields:
/// - `0`: An `Option<NaiveTime>` representing the wrapped optional time value.
///
/// ## Derives:
/// - `Debug`: Allows the struct to be formatted using the `{:?}` formatter.
/// - `PartialEq`: Enables comparison between `TimeValue` instances for equality.
/// - `Clone`: Provides the ability to create a copy of the `TimeValue` instance.
/// - `Default`: Supplies a default value of `None` for the underlying time.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(any(feature = "allow-default-value", test), derive(Default))]
pub struct TimeValue(Option<NaiveTime>);

impl TimeValue {
    /// Parses a given `NaiveTime` value (`subject`) according to the provided `TimeRules` (`rules`)
    /// and an optional custom format string (`format`).
    ///
    /// # Parameters
    /// - `subject`: An `Option<NaiveTime>` representing the time to be parsed. If `None`, no time is provided.
    /// - `rules`: A `TimeRules` instance specifying validation rules for the time.
    /// - `format`: An optional string slice representing a custom format to apply during validation.
    ///
    /// # Returns
    /// - `Ok(Self)`: Returns an instance of `Self` encapsulating the validated `NaiveTime`, if parsing
    ///   and validation succeed.
    /// - `Err(TimeError)`: Returns a `TimeError` if the time fails validation or if any issues are encountered
    ///   during parsing.
    ///
    /// # Errors
    /// This function will return `TimeError` under the following conditions:
    /// - If the `subject` does not adhere to the rules specified in `TimeRules`.
    /// - If any other errors arise during the validation and parsing process.
    ///
    /// # Implementation Details
    /// - The function uses an error collector (`ValidateErrorCollector`) to accumulate validation messages.
    /// - The `rules.check` method is called to validate the `subject` against the rules and optional format.
    /// - If any validation errors are detected, they are converted into a `TimeError` and returned.
    /// - On successful validation, the function constructs and returns an instance of `Self` containing the
    ///   provided `subject`.
    ///
    /// # Examples
    /// ```rust
    /// use chrono::NaiveTime;
    /// use cjtoolkit_structured_validator::types::times_chrono::time::{TimeRules, TimeValue};
    ///
    /// let time = Some(NaiveTime::from_hms_opt(12, 30, 45).unwrap_or_default());
    /// let rules = TimeRules::default();
    /// let format = Some("%H:%M:%S");
    ///
    /// let result = TimeValue::parse_custom_with_format(time, rules, format);
    /// assert!(result.is_ok());
    /// ```
    pub fn parse_custom_with_format(
        subject: Option<NaiveTime>,
        rules: TimeRules,
        format: Option<&str>,
    ) -> Result<Self, TimeError> {
        let mut messages = ValidateErrorCollector::new();
        rules.check(subject.as_ref(), &mut messages, format);
        TimeError::validate_check(messages)?;
        Ok(Self(subject))
    }

    /// Parses a custom time based on the provided `subject` and `rules`.
    ///
    /// This function takes an optional `NaiveTime` value (`subject`) and a set of `TimeRules`
    /// to generate a `Self` instance. It internally delegates the parsing logic to the
    /// `parse_custom_with_format` method, passing the `rules` and an optional format.
    ///
    /// # Parameters
    ///
    /// - `subject`: An `Option<NaiveTime>` that represents the time to be parsed. If `None`, parsing
    ///   may rely solely on the `rules` provided.
    /// - `rules`: The `TimeRules` that define how the time should be parsed or adjusted.
    ///
    /// # Returns
    ///
    /// - `Ok(Self)`: On successful parsing and generation of the `Self` instance.
    /// - `Err(TimeError)`: If there is an error during the parsing, or if the input does not meet the expected criteria.
    ///
    /// # See Also
    ///
    /// - [`parse_custom_with_format`]: The method that this function relies on for processing
    ///   the time along with an optional format.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveTime;
    /// use cjtoolkit_structured_validator::types::times_chrono::time::{TimeRules, TimeValue};
    ///
    /// let time = Some(NaiveTime::from_hms_opt(14, 30, 0).unwrap_or_default());
    /// let rules = TimeRules::default(); // Assuming a valid TimeRules implementation exists
    ///
    /// let result = TimeValue::parse_custom(time, rules);
    /// match result {
    ///     Ok(parsed_time) => println!("Parsed time: {:?}", parsed_time),
    ///     Err(err) => eprintln!("Error parsing time: {:?}", err),
    /// }
    /// ```
    pub fn parse_custom(subject: Option<NaiveTime>, rules: TimeRules) -> Result<Self, TimeError> {
        Self::parse_custom_with_format(subject, rules, None)
    }

    /// Parses an optional `NaiveTime` instance into the custom `Self` type, applying the default time rules.
    ///
    /// # Arguments
    /// - `subject`: An `Option<NaiveTime>` representing the time to be parsed. If `None`, default time rules are applied.
    ///
    /// # Returns
    /// - `Ok(Self)`: If the given time satisfies the specified rules and is successfully parsed.
    /// - `Err(TimeError)`: If the parsing fails due to an invalid time or rule violation.
    ///
    /// # Behavior
    /// This method delegates parsing to the `parse_custom` function, using default `TimeRules`.
    ///
    /// # Example
    /// ```
    /// use chrono::NaiveTime;
    /// use cjtoolkit_structured_validator::types::times_chrono::time::TimeValue;
    /// let naive_time = Some(NaiveTime::from_hms_opt(12, 0, 0).unwrap_or_default());
    /// let parsed_time = TimeValue::parse(naive_time);
    /// assert!(parsed_time.is_ok() || parsed_time.is_err());
    /// ```
    pub fn parse(subject: Option<NaiveTime>) -> Result<Self, TimeError> {
        Self::parse_custom(subject, TimeRules::default())
    }

    /// Parses a given optional `NaiveTime` value using a specified optional format string and returns
    /// the corresponding struct representation or a `TimeError` if the parsing fails.
    ///
    /// This function leverages a custom parsing method, combined with the default set of time rules,
    /// to interpret the time data. It is designed to handle cases where both the `NaiveTime` and
    /// formatting are optional, providing flexibility in the parsing logic.
    ///
    /// # Arguments
    ///
    /// * `subject` - An `Option<NaiveTime>` representing the time to be parsed. If `None`, the function
    /// falls back to using the rules and implementation provided in the parsing logic.
    /// * `format` - An `Option<&str>` containing the format string to interpret the given time.
    /// If `None`, the function may fall back to default rules or formats defined by the implementation.
    ///
    /// # Returns
    ///
    /// * `Result<Self, TimeError>` - If parsing is successful, returns an instance of the struct implementing
    /// the method. Otherwise, returns a `TimeError` indicating the failure during parsing.
    ///
    /// # Errors
    ///
    /// * Returns `TimeError` if the parsing fails due to an invalid time input, incompatible formatting,
    /// or other internal errors.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::NaiveTime;
    /// use cjtoolkit_structured_validator::types::times_chrono::time::TimeValue;
    ///
    /// let time = NaiveTime::parse_from_str("12:30:45", "%H:%M:%S").ok();
    /// let format = Some("%H:%M:%S");
    ///
    /// let parsed = TimeValue::parse_with_format(time, format);
    /// assert!(parsed.is_ok() || parsed.is_err());
    /// ```
    ///
    /// If `subject` or `format` is `None`:
    ///
    /// ```rust
    /// use cjtoolkit_structured_validator::types::times_chrono::time::TimeValue;
    /// let parsed = TimeValue::parse_with_format(None, None);
    /// assert!(parsed.is_ok() || parsed.is_err());
    /// ```
    pub fn parse_with_format(
        subject: Option<NaiveTime>,
        format: Option<&str>,
    ) -> Result<Self, TimeError> {
        Self::parse_custom_with_format(subject, TimeRules::default(), format)
    }

    /// Converts the current object into a `NaiveTime` instance.
    ///
    /// # Returns
    ///
    /// * `Some(NaiveTime)` - If the `NaiveTime` value exists and can be cloned.
    /// * `None` - If the `NaiveTime` value does not exist.
    ///
    /// Note: Replace `YourStruct` with the appropriate struct name that contains a tuple of type `Option<NaiveTime>`.
    pub fn as_time(&self) -> Option<NaiveTime> {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_custom() {
        let subject = NaiveTime::from_hms_opt(10, 0, 0);
        let rules = TimeRules::default();
        let result = TimeValue::parse_custom(subject, rules);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_default_err() {
        let result = TimeValue::parse(None);
        assert!(result.is_err());
    }
    #[test]
    fn test_parse_default_max_min_ok() {
        let subject = NaiveTime::from_hms_opt(10, 0, 0);
        let result = TimeValue::parse(subject);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_default_max_err() {
        let subject = NaiveTime::from_hms_opt(18, 0, 0);
        let result = TimeValue::parse(subject);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_default_min_err() {
        let subject = NaiveTime::from_hms_opt(8, 0, 0);
        let result = TimeValue::parse(subject);
        assert!(result.is_err());
    }
}
