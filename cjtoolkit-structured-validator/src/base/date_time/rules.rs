use crate::base::date_time::data::{DateTimeData, DateTimeKind};
use crate::common::locale::{LocaleData, LocaleMessage, LocaleValue, ValidateErrorCollector};
use std::sync::Arc;

/// `DateTimeMandatoryLocale` is a struct that enforces the usage of a specific locale
/// when working with date and time-related operations.
///
/// This struct can be used to ensure that any functionalities handling date and time
/// conform to a predefined locale, preventing ambiguity or unintended formatting issues.
///
/// # Example
///
/// While this struct itself may not expose direct methods, its purpose could be to act
/// as a marker or to be used as part of a larger system for locale management related to
/// parsing or formatting dates and times.
pub struct DateTimeMandatoryLocale;

impl LocaleMessage for DateTimeMandatoryLocale {
    fn get_locale_data(&self) -> Arc<LocaleData> {
        LocaleData::new("validate-cannot-be-empty")
    }
}

/// The `DateTimeMandatoryRules` struct is used to define rules regarding the mandatory status of a DateTime field.
///
/// This structure is typically used in validation or configuration to indicate whether a particular DateTime value
/// is required (mandatory) or optional.
///
/// # Fields
///
/// * `is_mandatory` (`bool`): A boolean flag indicating whether the DateTime field is mandatory.
///   * `true` - The DateTime is required.
///   * `false` - The DateTime is optional.
pub struct DateTimeMandatoryRules {
    pub is_mandatory: bool,
}

impl DateTimeMandatoryRules {
    /// Checks whether the provided `subject` meets the required validation constraints.
    ///
    /// This method verifies if a `subject` is supplied when the `is_mandatory` field is set to `true`.
    /// If `subject` is `None` and `is_mandatory` is `true`, an error message is pushed into the
    /// `messages` error collector.
    ///
    /// # Parameters
    /// - `messages`: A mutable reference to a `ValidateErrorCollector` that accumulates validation errors.
    /// - `subject`: An `Option` containing the `DateTimeData` to validate. If `None`, the method checks
    ///   if this absence is acceptable based on the `is_mandatory` flag.
    ///
    /// # Behavior
    /// - If `is_mandatory` is `true` and `subject` is `None`, a validation error with a localized
    ///   "Cannot be empty" message is added to the `messages` collector.
    pub fn check(&self, messages: &mut ValidateErrorCollector, subject: Option<&DateTimeData>) {
        if self.is_mandatory && subject.is_none() {
            messages.push((
                "Cannot be empty".to_string(),
                Box::new(DateTimeMandatoryLocale),
            ));
        }
    }
}

/// An enumeration representing the localization of a range constraint for a date-time value.
///
/// The `DateTimeRangeLocale` enum defines two possible constraints for date-time values:
/// - `MinValue`: Represents the minimum value constraint, encapsulated in a `DateTimeData`.
/// - `MaxValue`: Represents the maximum value constraint, encapsulated in a `DateTimeData`.
///
/// # Variants
///
/// - `MinValue(DateTimeData)`
///   Defines the lower boundary for a date-time range. Typically used to specify the earliest allowed date and time.
///
/// - `MaxValue(DateTimeData)`
///   Defines the upper boundary for a date-time range. Typically used to specify the latest allowed date and time.
///
/// # Notes
/// - This enum is designed to provide a flexible way to specify date-time range constraints,
///   which could be used in validation, filtering, or UI-based date pickers.
/// - Ensure the associated `DateTimeData` objects are valid and comply with the required format.
pub enum DateTimeRangeLocale {
    /// A variant representing the minimum value constraint, encapsulated in a `DateTimeData`.
    /// # Key
    /// * `validate-date-min`
    /// * `validate-date-time-min`
    /// * `validate-date-time-naive-min`
    /// * `validate-time-min`
    MinValue(DateTimeData),
    /// A variant representing the maximum value constraint, encapsulated in a `DateTimeData`.
    /// # Key
    /// * `validate-date-max`
    /// * `validate-date-time-max`
    /// * `validate-date-time-naive-max`
    /// * `validate-time-max`
    MaxValue(DateTimeData),
}

impl LocaleMessage for DateTimeRangeLocale {
    fn get_locale_data(&self) -> Arc<LocaleData> {
        use LocaleData as ld;
        use LocaleValue as lv;
        match self {
            DateTimeRangeLocale::MinValue(min) => match min.kind {
                DateTimeKind::Date => ld::new_with_vec(
                    "validate-date-min",
                    vec![("min".to_string(), lv::from(min.date_formatted.clone()))],
                ),
                DateTimeKind::DateTime => ld::new_with_vec(
                    "validate-date-time-min",
                    vec![("min".to_string(), lv::from(min.date_formatted.clone()))],
                ),
                DateTimeKind::DateTimeNaive => ld::new_with_vec(
                    "validate-date-time-naive-min",
                    vec![("min".to_string(), lv::from(min.date_formatted.clone()))],
                ),
                DateTimeKind::Time => ld::new_with_vec(
                    "validate-time-min",
                    vec![("min".to_string(), lv::from(min.date_formatted.clone()))],
                ),
            },
            DateTimeRangeLocale::MaxValue(max) => match max.kind {
                DateTimeKind::Date => ld::new_with_vec(
                    "validate-date-max",
                    vec![("max".to_string(), lv::from(max.date_formatted.clone()))],
                ),
                DateTimeKind::DateTime => ld::new_with_vec(
                    "validate-date-time-max",
                    vec![("max".to_string(), lv::from(max.date_formatted.clone()))],
                ),
                DateTimeKind::DateTimeNaive => ld::new_with_vec(
                    "validate-date-time-naive-max",
                    vec![("max".to_string(), lv::from(max.date_formatted.clone()))],
                ),
                DateTimeKind::Time => ld::new_with_vec(
                    "validate-time-max",
                    vec![("max".to_string(), lv::from(max.date_formatted.clone()))],
                ),
            },
        }
    }
}

/// Represents a set of rules that define a valid range for datetime values.
///
/// The `DateTimeRangeRules` struct is used to impose constraints on a datetime range by specifying
/// optional minimum (`min`) and maximum (`max`) boundaries. Both boundaries are inclusive if provided.
///
/// # Fields
///
/// * `min` - An `Option<DateTimeData>` that specifies the minimum allowable datetime value.
///           If `None`, there is no lower boundary.
/// * `max` - An `Option<DateTimeData>` that specifies the maximum allowable datetime value.
///           If `None`, there is no upper boundary.
pub struct DateTimeRangeRules {
    pub min: Option<DateTimeData>,
    pub max: Option<DateTimeData>,
}

impl DateTimeRangeRules {
    /// Validates the given `subject` `DateTimeData` against the minimum and maximum constraints
    /// specified in the current instance and collects validation errors.
    ///
    /// # Parameters
    /// - `messages`: A mutable reference to a `ValidateErrorCollector` that will store any validation errors encountered.
    /// - `subject`: An optional reference to a `DateTimeData` object to be validated. If `subject` is `None`,
    ///   a default `DateTimeData` object is used.
    ///
    /// # Behavior
    /// - If a `min` constraint is specified in the instance (`self.min`) and `subject` exists and is fewer than `min`,
    ///   an error message is added to `messages`.
    /// - If a `max` constraint is specified in the instance (`self.max`) and `subject` exists and is greater than `max`,
    ///   an error message is added to `messages`.
    ///
    /// # Errors
    /// - If `subject` does not meet the `min` or `max` constraints, a corresponding error message is pushed to the
    ///   `messages` collection.
    pub fn check(&self, messages: &mut ValidateErrorCollector, subject: Option<&DateTimeData>) {
        let default = DateTimeData::default();
        let is_some = subject.is_some();
        let subject = subject.unwrap_or(&default);
        if let Some(min) = &self.min {
            if is_some && subject < min {
                messages.push((
                    format!("Must be after '{}'", &subject.date_formatted),
                    Box::new(DateTimeRangeLocale::MinValue(min.clone())),
                ))
            }
        }
        if let Some(max) = &self.max {
            if is_some && subject > max {
                messages.push((
                    format!("Must be before '{}'", &subject.date_formatted),
                    Box::new(DateTimeRangeLocale::MaxValue(max.clone())),
                ))
            }
        }
    }

    /// See check, this is more for checking time alone
    pub fn check_time(
        &self,
        messages: &mut ValidateErrorCollector,
        subject: Option<&DateTimeData>,
    ) {
        let default = DateTimeData::default();
        let is_some = subject.is_some();
        let subject = subject.unwrap_or(&default);
        if let Some(min) = &self.min {
            if is_some && subject < min {
                messages.push((
                    format!("Must be after '{}'", &subject.date_formatted),
                    Box::new(DateTimeRangeLocale::MinValue(min.clone())),
                ))
            }
        }
        if let Some(max) = &self.max {
            let mut max = max.clone();
            if let Some(min) = &self.min
                && &max < min
            {
                // add day
                max.timestamp_seconds_days += 24 * 60 * 60;
            }
            if is_some && subject > &max {
                messages.push((
                    format!("Must be before '{}'", &subject.date_formatted),
                    Box::new(DateTimeRangeLocale::MaxValue(max.clone())),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod date_time_mandatory_rule {
        use super::*;
        use crate::base::date_time::data::DateTimeKind;

        #[test]
        fn test_empty_value() {
            let mut messages = ValidateErrorCollector::new();
            let subject: Option<DateTimeData> = None;
            let rules = DateTimeMandatoryRules { is_mandatory: true };
            rules.check(&mut messages, subject.as_ref());
            assert_eq!(messages.len(), 1);
            assert_eq!(messages.0[0].0, "Cannot be empty");
        }

        #[test]
        fn test_not_empty_value() {
            let mut messages = ValidateErrorCollector::new();
            let subject: Option<DateTimeData> = Some(DateTimeData {
                kind: DateTimeKind::DateTime,
                date_formatted: "".to_string(),
                timestamp_seconds_days: 1,
                subsec_nano: 1,
            });
            let rules = DateTimeMandatoryRules { is_mandatory: true };
            rules.check(&mut messages, subject.as_ref());
            assert_eq!(messages.len(), 0);
        }
    }

    mod date_time_range_rule {
        use super::*;
        use crate::base::date_time::data::DateTimeKind;

        #[test]
        fn test_min() {
            let mut messages = ValidateErrorCollector::new();
            let subject: Option<DateTimeData> = Some(DateTimeData {
                kind: DateTimeKind::DateTime,
                date_formatted: "".to_string(),
                timestamp_seconds_days: 1,
                subsec_nano: 1,
            });
            let rules = DateTimeRangeRules {
                min: Some(DateTimeData {
                    kind: DateTimeKind::DateTime,
                    date_formatted: "".to_string(),
                    timestamp_seconds_days: 1,
                    subsec_nano: 2,
                }),
                max: None,
            };
            rules.check(&mut messages, subject.as_ref());
            assert_eq!(messages.len(), 1);
            assert_eq!(messages.0[0].0, "Must be after ''");

            let mut messages = ValidateErrorCollector::new();
            let subject: Option<DateTimeData> = Some(DateTimeData {
                kind: DateTimeKind::DateTime,
                date_formatted: "".to_string(),
                timestamp_seconds_days: 1,
                subsec_nano: 3,
            });
            let rules = DateTimeRangeRules {
                min: Some(DateTimeData {
                    kind: DateTimeKind::DateTime,
                    date_formatted: "".to_string(),
                    timestamp_seconds_days: 1,
                    subsec_nano: 2,
                }),
                max: None,
            };
            rules.check(&mut messages, subject.as_ref());
            assert_eq!(messages.len(), 0);

            let mut messages = ValidateErrorCollector::new();
            let subject: Option<DateTimeData> = Some(DateTimeData {
                kind: DateTimeKind::DateTime,
                date_formatted: "".to_string(),
                timestamp_seconds_days: 1,
                subsec_nano: 1,
            });
            let rules = DateTimeRangeRules {
                min: Some(DateTimeData {
                    kind: DateTimeKind::DateTime,
                    date_formatted: "".to_string(),
                    timestamp_seconds_days: 2,
                    subsec_nano: 1,
                }),
                max: None,
            };
            rules.check(&mut messages, subject.as_ref());
            assert_eq!(messages.len(), 1);
            assert_eq!(messages.0[0].0, "Must be after ''");

            let mut messages = ValidateErrorCollector::new();
            let subject: Option<DateTimeData> = Some(DateTimeData {
                kind: DateTimeKind::DateTime,
                date_formatted: "".to_string(),
                timestamp_seconds_days: 3,
                subsec_nano: 1,
            });
            let rules = DateTimeRangeRules {
                min: Some(DateTimeData {
                    kind: DateTimeKind::DateTime,
                    date_formatted: "".to_string(),
                    timestamp_seconds_days: 2,
                    subsec_nano: 1,
                }),
                max: None,
            };
            rules.check(&mut messages, subject.as_ref());
            assert_eq!(messages.len(), 0);
        }
    }
}
