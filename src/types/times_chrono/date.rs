use crate::base::date_time::data::AsDateTimeData;
use crate::base::date_time::rules::{DateTimeMandatoryRules, DateTimeRangeRules};
use crate::common::locale::{ValidateErrorCollector, ValidateErrorStore};
use crate::common::validation_check::ValidationCheck;
use chrono::{NaiveDate, TimeDelta, Utc};
use std::ops::Add;
use thiserror::Error;

/// A struct representing validation rules for a date field, specifying its mandatory
/// status and optional boundaries on valid date ranges.
///
/// # Fields
///
/// * `is_mandatory` - A boolean flag that indicates whether the date field is mandatory.
///   If set to `true`, the date field must be provided.
///
/// * `min` - An `Option<NaiveDate>` representing the minimum allowable date. If set to `None`,
///   there is no lower-bound constraint on the date.
///
/// * `max` - An `Option<NaiveDate>` representing the maximum allowable date. If set to `None`,
///   there is no upper-bound constraint on the date.
///
/// # Note
/// This struct uses `NaiveDate` from the `chrono` crate, which represents dates without time zones.
/// Ensure that the `chrono` crate is added as a dependency in your project to use this struct.
pub struct DateRules {
    pub is_mandatory: bool,
    pub min: Option<NaiveDate>,
    pub max: Option<NaiveDate>,
}

impl Default for DateRules {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            is_mandatory: true,
            min: Some(now.clone().date_naive()),
            max: Some(now.clone().add(TimeDelta::days(30)).date_naive()),
        }
    }
}

impl DateRules {
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
                    .min
                    .as_ref()
                    .map(|max| (date_format.clone(), max).as_date_time_data()),
            },
        )
    }

    fn check(
        self,
        subject: Option<&NaiveDate>,
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

/// Represents an error encountered during date validation.
///
/// This struct encapsulates `ValidateErrorStore`, allowing for detailed error
/// reporting and debugging when a date fails validation.
///
/// # Derive Attributes
/// - `Debug`: Enables debugging output for the struct.
/// - `Error`: Implements the `std::error::Error` trait, making this struct a standard-compliant error type.
/// - `PartialEq`: Allows comparison of two `DateError` instances for equality.
/// - `Clone`: Enables deep cloning of the struct.
/// - `Default`: Provides a default constructor for the struct.
///
/// # Error Message
/// The custom error message associated with this struct is `"Date Validation Error"`.
///
/// # Fields
/// - `0: ValidateErrorStore` - A field that stores validation errors for further analysis.
#[derive(Debug, Error, PartialEq, Clone, Default)]
#[error("Date Validation Error")]
pub struct DateError(pub ValidateErrorStore);

impl ValidationCheck for DateError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

/// A wrapper struct for `Option<NaiveDate>` that represents an optional date value.
///
/// `DateValue` is a simple wrapper around `Option<NaiveDate>` to handle date values
/// more effectively, providing utility through traits like `Debug`, `PartialEq`, `Clone`,
/// and implementing a default constructor.
///
/// # Fields
/// - `0`: The inner `Option<NaiveDate>` that holds the optional date.
///
/// # Derives
/// - `Debug`: Enables the ability to format the struct using the `{:?}` formatter.
/// - `PartialEq`: Allows comparison between instances of `DateValue`.
/// - `Clone`: Enables the cloning of `DateValue` instances.
/// - `Default`: Provides a default constructor, which initializes the struct with `None`.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct DateValue(Option<NaiveDate>);

impl DateValue {
    /// Parses a date based on the provided subject, rules, and optional format, returning a new instance of the struct or an error.
    ///
    /// # Arguments
    ///
    /// * `subject` - An `Option<NaiveDate>` representing the date to be validated and parsed.
    ///   If `None` is provided, no date value is used for validation.
    /// * `rules` - A `DateRules` object containing the rules for validating the provided date.
    /// * `format` - An `Option<&str>` specifying the custom date format to be used during validation.
    ///   If `None`, the default rules or behavior will apply.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - If the provided date `subject` passes validation according to the `rules`
    ///   and optional `format`, a new instance of the struct is returned.
    /// * `Err(DateError)` - If validation fails, a `DateError` is returned containing
    ///   detailed information about the validation error.
    ///
    /// # Errors
    ///
    /// This function returns a `DateError` if:
    /// * The provided `subject` does not satisfy the conditions defined by the `rules`.
    /// * An invalid `format` is supplied, or the date fails to parse based on the custom format.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use cjtoolkit_structured_validator::types::times_chrono::date::{DateRules, DateValue};
    ///
    /// let date = Some(NaiveDate::from_ymd_opt(2023, 10, 7).unwrap());
    /// let rules = DateRules::default();
    /// let format = Some("%Y-%m-%d");
    ///
    /// match DateValue::parse_custom_with_format(date, rules, format) {
    ///     Ok(instance) => println!("Date parsed successfully: {:?}", instance),
    ///     Err(err) => println!("Failed to parse date: {:?}", err),
    /// }
    /// ```
    pub fn parse_custom_with_format(
        subject: Option<NaiveDate>,
        rules: DateRules,
        format: Option<&str>,
    ) -> Result<Self, DateError> {
        let mut messages = ValidateErrorCollector::new();
        rules.check(subject.as_ref(), &mut messages, format);
        DateError::validate_check(messages)?;
        Ok(Self(subject))
    }

    /// Parses a date based on the provided `subject` and `rules`.
    ///
    /// This function takes an optional `NaiveDate` as the `subject` and a `DateRules`
    /// object that defines specific parsing rules. It attempts to parse the date accordingly.
    ///
    /// # Arguments
    ///
    /// * `subject` - An `Option<NaiveDate>` representing the input date to be parsed.
    ///               If `None`, a fallback logic (if defined in `parse_custom_with_format`)
    ///               may handle the case or produce an error.
    /// * `rules` - A `DateRules` structure defining the constraints or logic for parsing
    ///             the provided date.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - If the date is successfully parsed based on the defined
    ///   rules and format.
    /// * `Err(DateError)` - If the parsing fails due to invalid input, missing
    ///   rules, or formatting issues.
    ///
    /// # Errors
    ///
    /// This function returns a `DateError` if:
    /// - The `subject` does not conform to the expected format or rules.
    /// - The `rules` object defines constraints that the provided input cannot satisfy.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use cjtoolkit_structured_validator::types::times_chrono::date::{DateRules, DateValue};
    ///
    /// let subject = Some(NaiveDate::from_ymd(2023, 10, 25));
    /// let rules = DateRules::default(); // Assume `DateRules::new` constructs default rules.
    ///
    /// let parsed_date = DateValue::parse_custom(subject, rules);
    /// match parsed_date {
    ///     Ok(date) => println!("Parsed date: {:?}", date),
    ///     Err(e) => println!("Failed to parse date: {:?}", e),
    /// }
    /// ```
    ///
    /// Internally, this function delegates the parsing operation to
    /// `parse_custom_with_format`, with the format parameter being `None`.
    pub fn parse_custom(subject: Option<NaiveDate>, rules: DateRules) -> Result<Self, DateError> {
        Self::parse_custom_with_format(subject, rules, None)
    }

    /// Parses the given `Option<NaiveDate>` using the default date rules.
    ///
    /// # Arguments
    /// * `subject` - An `Option<NaiveDate>` to be parsed, where:
    ///     - `Some(NaiveDate)` represents a valid date input.
    ///     - `None` represents the absence of a date to parse.
    ///
    /// # Returns
    /// * `Result<Self, DateError>` -
    ///     - `Ok(Self)` if the parsing is successful.
    ///     - `Err(DateError)` if an error occurs during parsing.
    ///
    /// # Notes
    /// This function leverages `parse_custom` internally, using the default
    /// rules defined by `DateRules`.
    ///
    /// # Examples
    /// ```rust
    /// use chrono::NaiveDate;
    /// use cjtoolkit_structured_validator::types::times_chrono::date::DateValue;
    ///
    /// let date = Some(NaiveDate::from_ymd(2023, 10, 12));
    /// let parsed = DateValue::parse(date);
    /// assert!(parsed.is_ok() || parsed.is_err());
    ///
    /// let none_date: Option<NaiveDate> = None;
    /// let parsed_none = DateValue::parse(none_date);
    /// assert!(parsed.is_ok() || parsed_none.is_err());
    /// ```
    pub fn parse(subject: Option<NaiveDate>) -> Result<Self, DateError> {
        Self::parse_custom(subject, DateRules::default())
    }

    /// Parses a `NaiveDate` with a custom format and returns a `Self` type or a `DateError`.
    ///
    /// This function enables parsing of a `NaiveDate` (wrapped inside an `Option`) using a specified
    /// custom format (also wrapped in an `Option`). It leverages the default date rules (`DateRules::default()`)
    /// in its implementation.
    ///
    /// # Arguments
    ///
    /// * `subject` - An `Option<NaiveDate>` representing the date to be parsed. If `None`, the parsing process is skipped.
    /// * `format` - An `Option<&str>` containing the custom format to be used for parsing. If `None`,
    ///              a default or alternative logic within `Self::parse_custom_with_format` will likely apply.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - If the parsing is successful, the function returns an instance of the `Self` type.
    /// * `Err(DateError)` - If the parsing fails, an appropriate `DateError` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use cjtoolkit_structured_validator::types::times_chrono::date::DateValue;
    /// let date = NaiveDate::from_ymd(2023, 10, 5); // Example date (2023-10-05)
    /// let format = Some("%Y-%m-%d"); // Format example: Year-Month-Day
    ///
    /// let parsed_result = DateValue::parse_with_format(Some(date), format);
    /// match parsed_result {
    ///     Ok(result) => println!("Parsed successfully: {:?}", result),
    ///     Err(err) => eprintln!("Parsing failed: {:?}", err),
    /// }
    /// ```
    ///
    /// # Implementation Note
    ///
    /// This function internally relies on `Self::parse_custom_with_format` and applies
    /// the default `DateRules`. It abstracts away the rule configuration for convenience.
    pub fn parse_with_format(
        subject: Option<NaiveDate>,
        format: Option<&str>,
    ) -> Result<Self, DateError> {
        Self::parse_custom_with_format(subject, DateRules::default(), format)
    }

    /// Converts the `CustomDate` object into an `Option<NaiveDate>`.
    ///
    /// # Returns
    /// - `Some(NaiveDate)` if the inner date value (`self.0`) exists.
    /// - `None` if the inner value is `None`.
    ///
    /// # Notes
    /// `NaiveDate` is a struct provided by the `chrono` crate, representing a calendar date without timezone information.
    pub fn as_naive_date(&self) -> Option<NaiveDate> {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_custom() {
        let subject = Some(Utc::now().date_naive());
        let rules = DateRules::default();
        let result = DateValue::parse_custom(subject, rules);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_default_err() {
        let result = DateValue::parse(None);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_default_max_min_ok() {
        let subject = Some(Utc::now().date_naive());
        let result = DateValue::parse(subject);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_default_max_err() {
        let subject = Some(Utc::now().date_naive().add(TimeDelta::days(31)));
        let result = DateValue::parse(subject);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_default_min_err() {
        let subject = Some(Utc::now().date_naive().add(TimeDelta::days(-1)));
        let result = DateValue::parse(subject);
        assert!(result.is_err());
    }
}
