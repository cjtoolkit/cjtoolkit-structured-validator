//! This module contains structures and traits for working with floating-point numbers.

use crate::base::number_rules::{NumberMandatoryRules, NumberRangeRules};
use crate::common::locale::{ValidateErrorCollector, ValidateErrorStore};
use crate::common::validation_check::ValidationCheck;

/// A structure representing rules and constraints for floating-point values.
///
/// # Fields
/// - `is_mandatory`:
///   Determines whether the floating-point value is mandatory (`true`) or optional (`false`).
///
/// - `min`:
///   An optional lower bound (`Option<f64>`) for the value. If `Some`, the value must be greater than or equal to this.
///   If `None`, there is no minimum constraint.
///
/// - `max`:
///   An optional upper bound (`Option<f64>`) for the value. If `Some`, the value must be less than or equal to this.
///   If `None`, there is no maximum constraint.
///
/// This structure can be used to validate or enforce business logic with respect to floating-point numbers.
pub struct FloatRules {
    pub is_mandatory: bool,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

impl Default for FloatRules {
    fn default() -> Self {
        Self {
            is_mandatory: true,
            min: Some(0.0),
            max: Some(255.0),
        }
    }
}

impl Into<(NumberMandatoryRules, NumberRangeRules<f64>)> for &FloatRules {
    fn into(self) -> (NumberMandatoryRules, NumberRangeRules<f64>) {
        (
            NumberMandatoryRules {
                is_mandatory: self.is_mandatory,
            },
            NumberRangeRules {
                min: self.min,
                max: self.max,
            },
        )
    }
}

impl FloatRules {
    fn rules(&self) -> (NumberMandatoryRules, NumberRangeRules<f64>) {
        self.into()
    }

    fn check(&self, messages: &mut ValidateErrorCollector, subject: Option<f64>) {
        if !self.is_mandatory && subject.is_none() {
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

/// A structure representing an error that occurs during validation of a floating-point value.
///
/// The `FloatError` wraps around a `ValidateErrorStore`, which is used to store details
/// related to a failed validation. This struct derives several standard traits to enhance
/// its usability:
///
/// - `Debug`: Enables the error to be formatted using the `{:?}` formatter, mainly for debugging purposes.
/// - `PartialEq`: Allows comparison between two `FloatError` instances for equality.
/// - `Clone`: Permits creating a duplicate of the `FloatError` instance.
/// - `Default`: Provides a default empty implementation of `FloatError`.
///
/// # Fields
///
/// * `0: ValidateErrorStore` - The underlying store containing validation error details.
///
#[derive(Debug, PartialEq, Clone, Default)]
pub struct FloatError(pub ValidateErrorStore);

impl ValidationCheck for FloatError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

impl Into<ValidateErrorStore> for &FloatError {
    fn into(self) -> ValidateErrorStore {
        self.0.clone()
    }
}

/// A struct that represents a floating-point number with additional metadata.
///
/// The `Float` struct is composed of two fields:
/// - A `f64`, which stores the numeric value.
/// - A `bool`, which stores an additional metadata flag associated with the float.
///
/// # Derives
/// - `Debug`: Enables the struct to be formatted using the `{:?}` formatter, useful for debugging.
/// - `PartialEq`: Allows comparing two `Float` instances for equality.
/// - `Clone`: Provides the ability to create a copy of a `Float` instance.
///
/// # Fields
/// - `f64`: Represents the numeric value of the floating-point number.
/// - `bool`: Represents the metadata flag associated with the float, which can be used for custom purposes.
#[derive(Debug, PartialEq, Clone)]
pub struct Float(f64, bool);

#[cfg(any(feature = "allow-default-value", test))]
impl Default for Float {
    fn default() -> Self {
        Self(0.0, true)
    }
}

impl Float {
    /// Parses an optional floating-point value (`Option<f64>`) and validates it against a set of rules.
    ///
    /// # Parameters
    /// - `s`: An `Option<f64>` value to be parsed. If `None`, the function uses a default value of `0.0`.
    /// - `rules`: A `FloatRules` object that defines validation rules for the floating-point value.
    ///
    /// # Returns
    /// - `Ok(Self)`: If the input value passes the validation rules, the function returns an instance of `Self`,
    ///               which encapsulates the parsed floating-point value and a boolean indicating whether the input was `None`.
    /// - `Err(FloatError)`: If the input value violates any of the provided validation rules, an error is returned.
    ///
    /// # Behavior
    /// 1. Checks if the input value `s` is `None` and stores the result in `is_none`.
    /// 2. Creates a validation error collector using `ValidateErrorCollector`.
    /// 3. Invokes the `check` method of `rules` to validate the input value `s`, accumulating any validation errors.
    /// 4. Checks if there are any validation errors using `FloatError::validate_check`.
    ///    If validation errors are found, an error is returned.
    /// 5. On successful validation, constructs and returns an instance of `Self` containing:
    ///    - The unwrapped floating-point value (or `0.0` if `s` is `None`).
    ///    - A boolean indicating whether the input was `None`.
    ///
    /// # Example
    /// ```rust
    /// use cjtoolkit_structured_validator::types::numbers::float::{Float, FloatRules};
    /// let rules = FloatRules{
    ///     is_mandatory: false,
    ///     min: Some(2.0),
    ///     max: Some(7.5)
    /// };
    /// let result = Float::parse_custom(Some(5.0), rules);
    ///
    /// match result {
    ///     Ok(parsed) => println!("Parsed value: {:?}", parsed),
    ///     Err(err) => println!("Error: {:?}", err),
    /// }
    /// ```
    ///
    /// # Errors
    /// - Returns a `FloatError` if the input value does not satisfy the validation rules provided in `rules`.
    pub fn parse_custom(s: Option<f64>, rules: FloatRules) -> Result<Self, FloatError> {
        let is_none = s.is_none();
        let mut messages = ValidateErrorCollector::new();
        rules.check(&mut messages, s);
        FloatError::validate_check(messages)?;
        Ok(Self(s.unwrap_or_default(), is_none))
    }

    ///
    /// Parses an optional `f64` value into a custom type implementing `Self`.
    ///
    /// # Arguments
    ///
    /// * `s` - An `Option<f64>` value to be parsed.
    ///          - If `Some(f64)` is provided, it attempts to parse the float value.
    ///          - If `None` is provided, it implies no value was provided.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - If parsing succeeds and the value adheres to `FloatRules`.
    /// * `Err(FloatError)` - If parsing fails or the value violates `FloatRules`.
    ///
    /// # Behavior
    ///
    /// This function uses the `Self::parse_custom` method to perform the actual parsing
    /// logic with a default set of rules provided by the `FloatRules::default()` method.
    ///
    /// # Errors
    ///
    /// This function returns a `FloatError` if:
    /// - The input float is not valid based on the custom rules.
    /// - The parsing logic encounters any issues, such as unexpected `None` input.
    ///
    /// # Example
    ///
    /// ```
    /// use cjtoolkit_structured_validator::types::numbers::float::{Float, FloatRules};
    ///
    /// let input = Some(42.0);
    /// let result = Float::parse(input);
    ///
    /// match result {
    ///     Ok(value) => println!("Parsed value: {:?}", value),
    ///     Err(e) => println!("Failed to parse: {:?}", e),
    /// }
    /// ```
    ///
    /// This function delegates the responsibility for custom parsing logic to `Self::parse_custom`
    /// while adhering to a default set of floating-point validation rules.
    ///
    pub fn parse(s: Option<f64>) -> Result<Self, FloatError> {
        Self::parse_custom(s, FloatRules::default())
    }

    /// Returns the inner value as a `f64`.
    ///
    /// This method provides access to the stored value of the type `f64` encapsulated within the struct.
    ///
    /// # Returns
    ///
    /// * `f64` - The inner `f64` value stored in the struct.
    pub fn as_f64(&self) -> f64 {
        self.0
    }

    /// Converts the current instance into an `Option<Float>` depending on its internal state.
    ///
    /// # Returns
    /// - `None` if the second element of the tuple (`self.1`) evaluates to `true`.
    /// - `Some(Float)` if the second element of the tuple (`self.1`) evaluates to `false`,
    ///   containing the current instance.
    ///
    /// # Notes
    /// - Assumes `self` is of a tuple type where the first element represents a `Float`
    ///   and the second element is a boolean.
    pub fn into_option(self) -> Option<Float> {
        if self.1 { None } else { Some(self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float() {
        let float = Float::parse(Some(10.0));
        assert!(float.is_ok());
        let float = Float::parse(Some(1000.0));
        assert!(float.is_err());
        let float = Float::parse(Some(-0.1));
        assert!(float.is_err());
    }

    #[test]
    fn test_none_float() {
        let float = Float::parse(None);
        assert!(float.is_err());
    }
}
