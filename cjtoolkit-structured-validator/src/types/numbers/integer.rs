//! This module contains structures and traits for working with integer values.

use crate::base::number_rules::{NumberMandatoryRules, NumberRangeRules};
use crate::common::locale::{ValidateErrorCollector, ValidateErrorStore};
use crate::common::validation_check::ValidationCheck;

/// A structure representing validation rules for an integer value.
///
/// The `IntegerRules` struct provides constraints that can be used to
/// define the validity of an integer value. It includes properties for
/// specifying whether the value is required, as well as optional minimum
/// and maximum bounds.
///
/// # Fields
///
/// * `is_mandatory` - A boolean flag indicating whether the integer value
///   is mandatory. If `true`, the value must be provided; if `false`,
///   the value is optional.
///
/// * `min` - An optional minimum bound for the integer. If `Some(value)`,
///   the integer must be greater than or equal to `value`. If `None`,
///   no minimum constraint is applied.
///
/// * `max` - An optional maximum bound for the integer. If `Some(value)`,
///   the integer must be less than or equal to `value`. If `None`,
///   no maximum constraint is applied.
pub struct IntegerRules {
    pub is_mandatory: bool,
    pub min: Option<isize>,
    pub max: Option<isize>,
}

impl Default for IntegerRules {
    fn default() -> Self {
        Self {
            is_mandatory: true,
            min: Some(0),
            max: Some(255),
        }
    }
}

impl Into<(NumberMandatoryRules, NumberRangeRules<isize>)> for &IntegerRules {
    fn into(self) -> (NumberMandatoryRules, NumberRangeRules<isize>) {
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

impl IntegerRules {
    fn rules(&self) -> (NumberMandatoryRules, NumberRangeRules<isize>) {
        self.into()
    }

    fn check(&self, messages: &mut ValidateErrorCollector, subject: Option<isize>) {
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

/// Represents an error type for integer validation.
///
/// This struct wraps a `ValidateErrorStore` to provide detailed
/// information about validation errors specifically for integers.
///
/// # Derives
/// - `Debug`: Enables formatting the struct using the `{:?}` formatter.
/// - `PartialEq`: Allows comparison between two `IntegerError` objects for equality.
/// - `Clone`: Enables cloning of `IntegerError` instances.
/// - `Default`: Provides a default value of `IntegerError` using the default value of `ValidateErrorStore`.
///
/// # Fields
/// - `0: ValidateErrorStore`: The underlying error storage containing detailed validation error information.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct IntegerError(pub ValidateErrorStore);

impl ValidationCheck for IntegerError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

/// A struct representing an integer value paired with a boolean flag.
///
/// The `Integer` struct contains:
/// - A signed integer value of a type `isize`.
/// - A boolean flag associated with the integer.
///
/// The struct derives:
/// - `Debug` for formatting and debugging purposes.
/// - `PartialEq` to enable equality comparisons between `Integer` instances.
/// - `Clone` to allow creating a duplicate of an `Integer` instance.
///
/// # Fields
/// - `isize`: The signed integer value.
/// - `bool`: The boolean flag associated with the integer.
#[derive(Debug, PartialEq, Clone)]
pub struct Integer(isize, bool);

#[cfg(any(feature = "allow-default-value", test))]
impl Default for Integer {
    fn default() -> Self {
        Self(0, true)
    }
}

impl Integer {
    /// Parses an `Option<isize>` value according to the provided `IntegerRules`.
    ///
    /// # Arguments
    ///
    /// * `s` - An `Option<isize>` value to be parsed. If `None`, a default value will be used.
    /// * `rules` - A set of validation rules represented by `IntegerRules` that must be applied to the input value.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Self, IntegerError>`:
    /// * `Ok(Self)` - If the input value satisfies the provided `rules`. The result includes:
    ///   - The parsed `isize` value (or a default value if the input is `None`).
    ///   - A flag indicating whether the original input was `None`.
    /// * `Err(IntegerError)` - If the validation fails, containing details of the validation errors.
    ///
    /// # Behavior
    ///
    /// 1. Checks if the input `s` is `None` to determine if a default value should be used.
    /// 2. Uses a `ValidateErrorCollector` to aggregate and validate errors encountered when applying the `rules`.
    /// 3. If any validation errors occur, the function returns an `Err(IntegerError)`.
    /// 4. On successful validation, returns the parsed value (or default) and the `is_none` flag.
    ///
    /// # Errors
    ///
    /// This function returns an `IntegerError` if the input value does not meet the conditions
    /// defined in the `rules`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cjtoolkit_structured_validator::types::numbers::integer::{Integer, IntegerRules};
    ///
    /// let rules = IntegerRules{
    ///     is_mandatory: true,
    ///     min: Some(0),
    ///     max: Some(5),
    /// };
    /// let result = Integer::parse_custom(Some(42), rules);
    ///
    /// match result {
    ///     Ok(parsed) => println!("Parsed value: {:?}", parsed),
    ///     Err(err) => eprintln!("Validation error: {:?}", err),
    /// }
    /// ```
    pub fn parse_custom(s: Option<isize>, rules: IntegerRules) -> Result<Self, IntegerError> {
        let is_none = s.is_none();
        let mut messages = ValidateErrorCollector::new();
        rules.check(&mut messages, s);
        IntegerError::validate_check(messages)?;
        Ok(Self(s.unwrap_or_default(), is_none))
    }

    /// Parses an optional integer (`Option<isize>`) into a `Self` type,
    /// returning a `Result` that contains either the parsed value or
    /// an `IntegerError` if the input is invalid or cannot be parsed.
    ///
    /// # Arguments
    ///
    /// * `s` - An `Option<isize>` representing the integer input to parse.
    ///         If `None`, the method might handle it based on the default
    ///         rules provided by `IntegerRules::default()`.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - If the input is successfully parsed into the desired type.
    /// * `Err(IntegerError)` - If the input fails to parse due to invalid
    ///   format or other constraints.
    ///
    /// # Details
    ///
    /// This method leverages the custom parsing logic defined in
    /// `Self::parse_custom` and uses the default rules provided
    /// by `IntegerRules::default()` as the parsing guideline.
    ///
    /// # Example
    ///
    /// ```
    /// use cjtoolkit_structured_validator::types::numbers::integer::Integer;
    /// match Integer::parse(Some(42)) {
    ///     Ok(_) => println!("Parsed value"),
    ///     Err(_) => eprintln!("Failed to parse"),
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// This function may return an `IntegerError` in scenarios such as
    /// - The input value is `None` and cannot be handled appropriately.
    /// - The value does not comply with the validation or conversion rules.
    pub fn parse(s: Option<isize>) -> Result<Self, IntegerError> {
        Self::parse_custom(s, IntegerRules::default())
    }

    /// Converts the value contained in the type to an `isize`.
    ///
    /// # Returns
    /// This method returns the inner value of the type as an `isize`.
    pub fn as_isize(&self) -> isize {
        self.0
    }

    /// Converts the `Integer` to an `Option<Integer>`.
    ///
    /// # Description
    ///
    /// This function transforms the current instance of a type implementing this method
    /// into an `Option<Integer>`. If the `self.1` component is `true`, the function
    /// returns `None`. Otherwise, it wraps the current instance in `Some`.
    ///
    /// # Returns
    ///
    /// - `Some(self)` if `self.1` is `false`.
    /// - `None` if `self.1` is `true`.
    pub fn into_option(self) -> Option<Integer> {
        if self.1 { None } else { Some(self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer() {
        let integer = Integer::parse(Some(10));
        assert!(integer.is_ok());
        let integer = Integer::parse(Some(1000));
        assert!(integer.is_err());
        let integer = Integer::parse(Some(-50));
        assert!(integer.is_err());
    }

    #[test]
    fn test_none_integer() {
        let integer = Integer::parse(None);
        assert!(integer.is_err());
    }
}
