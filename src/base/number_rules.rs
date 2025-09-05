use crate::common::locale::{LocaleData, LocaleMessage, LocaleValue, ValidateErrorCollector};
use std::fmt::Display;

/// `NumberMandatoryLocale` is a struct representing a type that may be used
/// to enforce the concept.
///
///
/// # Possible key values:
/// * `validate-cannot-be-empty`
pub struct NumberMandatoryLocale;

impl LocaleMessage for NumberMandatoryLocale {
    fn get_locale_data(&self) -> LocaleData {
        LocaleData::new("validate-cannot-be-empty")
    }
}

/// Represents a set of rules determining whether a number field or value is mandatory.
///
/// # Fields
/// - `is_mandatory` (`bool`):
///   Specifies whether the associated number is mandatory or not.
///   - `true`: The number is required (mandatory).
///   - `false`: The number is optional.
pub struct NumberMandatoryRules {
    pub is_mandatory: bool,
}

impl NumberMandatoryRules {
    /// Checks whether a given subject is valid based on the rules of the current instance
    /// and collects any validation errors in the provided `ValidateErrorCollector`.
    ///
    /// # Type Parameters
    /// - `T`: A type that implements the `Into<LocaleValue>` trait, representing the value to be checked.
    ///
    /// # Parameters
    /// - `&self`: Immutable reference to the instance containing validation settings (`is_mandatory` in this case).
    /// - `messages`: A mutable reference to a `ValidateErrorCollector`, used to collect error messages
    ///   if the validation fails.
    /// - `subject`: An optional input value of type `T` to be validated. If `None` and the rule `is_mandatory`
    ///   is `true`, it triggers a validation error.
    ///
    /// # Behavior
    /// - If `is_mandatory` is `true` and the `subject` is `None` (i.e., no value is provided):
    ///   - The method appends an error message to `messages`, with the description `"Cannot be empty"`
    ///     and a boxed `NumberMandatoryLocale` as additional context.
    ///
    /// # Example
    /// ```rust
    /// use cjtoolkit_structured_validator::base::number_rules::NumberMandatoryRules;
    /// use cjtoolkit_structured_validator::common::locale::ValidateErrorCollector;
    /// let validator = NumberMandatoryRules { is_mandatory: true };
    /// let mut errors = ValidateErrorCollector::new();
    ///
    /// // Example with a subject as None (will cause validation error)
    /// validator.check::<f64>(&mut errors, None);
    /// assert_eq!(errors.len(), 1);
    ///
    /// // Example with a valid subject (no validation error)
    /// validator.check::<f64>(&mut errors, Some(1.0));
    /// assert_eq!(errors.len(), 1); // No additional errors added.
    /// ```
    ///
    /// # Note
    /// - Ensure that `ValidateErrorCollector` is properly initialized and passed by mutable reference
    ///   to capture errors.
    /// - The subject, when provided, must implement the `Into<LocaleValue>` trait for compatibility.
    pub fn check<T: Into<LocaleValue>>(
        &self,
        messages: &mut ValidateErrorCollector,
        subject: Option<T>,
    ) {
        if self.is_mandatory && subject.is_none() {
            messages.push((
                "Cannot be empty".to_string(),
                Box::new(NumberMandatoryLocale),
            ));
        }
    }
}

/// An enumeration representing a range of values with localization support.
///
/// `NumberRangeLocale` is a generic enum used to define a localized numerical
/// range. It encapsulates a minimum or maximum value that can be converted into
/// a locale-specific representation using the `LocaleValue` type.
///
/// # Type Parameters
/// - `T`: A type that implements the traits `Into<LocaleValue>`, `Send`, `Sync`,
///   and `Clone`. This allows for flexible and thread-safe representation of values
///   that can be converted into localized formats.
///
/// # Variants
/// - `MinValue(T)`: Represents the minimum localized value for the range.
/// - `MaxValue(T)`: Represents the maximum localized value for the range.
///
pub enum NumberRangeLocale<T: Into<LocaleValue> + Send + Sync + Clone> {
    /// Represents the minimum localized value for the range.
    /// # Key
    /// * `validate-number-min-value`
    MinValue(T),
    /// Represents the maximum localized value for the range.
    /// # Key
    /// * `validate-number-max-value`
    MaxValue(T),
}

impl<T: Into<LocaleValue> + Send + Sync + Clone> LocaleMessage for NumberRangeLocale<T>
where
    LocaleValue: From<T>,
{
    fn get_locale_data(&self) -> LocaleData {
        use LocaleData as ld;
        use LocaleValue as lv;
        match self {
            Self::MinValue(min) => ld::new_with_vec(
                "validate-number-min-value",
                vec![("min".to_string(), lv::from(min.clone()))],
            ),
            Self::MaxValue(max) => ld::new_with_vec(
                "validate-number-max-value",
                vec![("max".to_string(), lv::from(max.clone()))],
            ),
        }
    }
}

/// A struct that represents rules for defining a range of numeric values with optional minimum and maximum bounds.
///
/// This struct is generic and can work with any type `T` that meets the following trait bounds:
/// - `Clone`: The type can be cloned.
/// - `Into<LocaleValue>`: The type can be converted into a `LocaleValue`. This allows for localization support.
/// - `Default`: The type has a default value.
/// - `PartialOrd`: The type supports partial ordering, enabling comparisons like less than or greater than.
/// - `Display`: The type can be formatted as a string for display purposes.
///
/// # Fields
/// - `min` (Option<T>): The optional lower bound of the range. If `None`, there is no restriction on the minimum value.
/// - `max` (Option<T>): The optional upper bound of the range. If `None`, there is no restriction on the maximum value.
///
pub struct NumberRangeRules<T>
where
    T: Clone + Into<LocaleValue> + Default + PartialOrd + Display,
{
    pub min: Option<T>,
    pub max: Option<T>,
}

impl<T> NumberRangeRules<T>
where
    T: Clone + Into<LocaleValue> + Default + PartialOrd + Display,
{
    /// Validates a given `subject` against optional minimum and maximum value constraints.
    ///
    /// # Parameters
    ///
    /// - `&self`: A reference to the current instance of the object containing validation constraints.
    /// - `messages`: A mutable reference to a `ValidateErrorCollector`, where validation error messages
    ///   will be stored if the `subject` does not meet the constraints.
    /// - `subject`: An optional value of type `T` to be validated against the constraints.
    ///
    /// # Behavior
    ///
    /// - If the `subject` is `Some`:
    ///     - It checks whether the value is less than the optional minimum value (`self.min`).
    ///         - If the value is less, an error message is added to `messages` stating that the value
    ///           must be at least the specified minimum.
    ///     - It checks whether the value is greater than the optional maximum value (`self.max`).
    ///         - If the value is greater, an error message is added to `messages` stating that the value
    ///           must be at most the specified maximum.
    /// - If the `subject` is `None`, the default value is used during validation (`T::default()`).
    /// - Does nothing if both `self.min` and `self.max` are `None`.
    ///
    /// # Usage
    ///
    /// This function is intended to validate numerical ranges or similar constraints. The errors detected
    /// during validation are collected into the `ValidateErrorCollector` provided in the `messages` parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// use cjtoolkit_structured_validator::common::locale::ValidateErrorCollector;
    /// use cjtoolkit_structured_validator::base::number_rules::NumberRangeRules;
    /// let mut error_collector = ValidateErrorCollector::new();
    /// let validator = NumberRangeRules::<usize> {
    ///     min: Some(10),
    ///     max: Some(100),
    /// };
    ///
    /// validator.check(&mut error_collector, Some(5));   // Value too small, error is added.
    /// validator.check(&mut error_collector, Some(105)); // Value too large, error is added.
    /// validator.check(&mut error_collector, Some(50));  // Valid value, no error.
    /// ```
    ///
    /// # Note
    ///
    /// - It is assumed that `T` implements the `Default`, `PartialOrd`, and `Clone` traits.
    /// - The `ValidateErrorCollector` and `NumberRangeLocale` types are expected to support the operations shown above.
    ///
    pub fn check(&self, messages: &mut ValidateErrorCollector, subject: Option<T>) {
        let is_some = subject.is_some();
        let subject = subject.unwrap_or_default();
        if let Some(min) = &self.min {
            if is_some && subject < *min {
                messages.push((
                    format!("Must be at least {}", min),
                    Box::new(NumberRangeLocale::MinValue(min.clone().into())),
                ));
            }
        }
        if let Some(max) = &self.max {
            if is_some && subject > *max {
                messages.push((
                    format!("Must be at most {}", max),
                    Box::new(NumberRangeLocale::MaxValue(max.clone().into())),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod number_mandatory_rule {
        use super::*;

        #[test]
        fn test_empty_value() {
            let mut messages = ValidateErrorCollector::new();
            let subject: Option<f64> = None;
            let rules = NumberMandatoryRules { is_mandatory: true };
            rules.check(&mut messages, subject);
            assert_eq!(messages.len(), 1);
            assert_eq!(messages.0[0].0, "Cannot be empty");
        }

        #[test]
        fn test_not_empty_value() {
            let mut messages = ValidateErrorCollector::new();
            let subject: Option<f64> = Some(1.0);
            let rules = NumberMandatoryRules { is_mandatory: true };
            rules.check(&mut messages, subject);
            assert_eq!(messages.len(), 0);
        }
    }

    mod number_range_rule {
        use super::*;

        #[test]
        fn test_invalid_min_value_rule() {
            let mut messages = ValidateErrorCollector::new();
            let subject: Option<f64> = Some(1.0);
            let rules = NumberRangeRules {
                min: Some(2.0),
                max: None,
            };
            rules.check(&mut messages, subject);
            assert_eq!(messages.len(), 1);
            assert_eq!(messages.0[0].0, "Must be at least 2");
        }

        #[test]
        fn test_valid_min_value_rule() {
            let mut messages = ValidateErrorCollector::new();
            let subject: Option<f64> = Some(2.0);
            let rules = NumberRangeRules {
                min: Some(2.0),
                max: None,
            };
            rules.check(&mut messages, subject);
            assert_eq!(messages.len(), 0);
        }

        #[test]
        fn test_max_value_rule() {
            let mut messages = ValidateErrorCollector::new();
            let subject: Option<f64> = Some(1.0);
            let rules = NumberRangeRules {
                min: None,
                max: Some(2.0),
            };
            rules.check(&mut messages, subject);
            assert_eq!(messages.len(), 0);
        }

        #[test]
        fn test_valid_max_value_rule() {
            let mut messages = ValidateErrorCollector::new();
            let subject: Option<f64> = Some(2.1);
            let rules = NumberRangeRules {
                min: None,
                max: Some(2.0),
            };
            rules.check(&mut messages, subject);
            assert_eq!(messages.len(), 1);
            assert_eq!(messages.0[0].0, "Must be at most 2");
        }
    }
}
