//! This module contains structures and traits for working with unsigned numerical values.

use crate::base::number_rules::{NumberMandatoryRules, NumberRangeRules};
use crate::common::locale::{ValidateErrorCollector, ValidateErrorStore};
use crate::common::validation_check::ValidationCheck;

/// A structure representing rules and constraints for unsigned numerical values.
///
/// The `UnsignedRules` struct is used to define validation rules for an unsigned number.
/// This can include whether the value is mandatory, and optional minimum and maximum bounds.
///
/// # Fields
///
/// * `is_mandatory` - A boolean flag that specifies whether the value is required to be present.
/// * `min` - An optional minimum value (inclusive) of type `usize`.
///            If `None`, no minimum constraint is applied.
/// * `max` - An optional maximum value (inclusive) of a type `usize`.
///            If `None`, no maximum constraint is applied.
pub struct UnsignedRules {
    pub is_mandatory: bool,
    pub min: Option<usize>,
    pub max: Option<usize>,
}

impl Default for UnsignedRules {
    fn default() -> Self {
        Self {
            is_mandatory: true,
            min: Some(0),
            max: Some(255),
        }
    }
}

impl Into<(NumberMandatoryRules, NumberRangeRules<usize>)> for &UnsignedRules {
    fn into(self) -> (NumberMandatoryRules, NumberRangeRules<usize>) {
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

impl UnsignedRules {
    fn rules(&self) -> (NumberMandatoryRules, NumberRangeRules<usize>) {
        self.into()
    }

    fn check(&self, messages: &mut ValidateErrorCollector, subject: Option<usize>) {
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

/// Represents an error structure specifically for handling unsigned validation errors.
///
/// The `UnsignedError` struct wraps around a `ValidateErrorStore`, allowing
/// it to encapsulate and store validation errors that occur in unsigned contexts.
///
/// # Attributes
/// - `0` (`ValidateErrorStore`): The internal storage for the validation errors.
///
/// # Traits
/// - `Debug`: Enables the struct to be formatted using the `{:?}` formatter for debugging purposes.
/// - `PartialEq`: Allows for equality comparisons between two instances of `UnsignedError`.
/// - `Clone`: Allows duplication of `UnsignedError` instances to create new copies.
/// - `Default`: Provides a default value for `UnsignedError`, which initializes the wrapped
///              `ValidateErrorStore` to its default state.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct UnsignedError(pub ValidateErrorStore);

impl ValidationCheck for UnsignedError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

impl Into<ValidateErrorStore> for &UnsignedError {
    fn into(self) -> ValidateErrorStore {
        self.0.clone()
    }
}

/// A struct that represents an unsigned integer with additional metadata.
///
/// The `Unsigned` struct encapsulates a `usize` value along with a boolean flag,
/// potentially allowing it to carry additional contextual information.
///
/// # Fields
/// - `0`: A `usize` representing the primary unsigned value.
/// - `1`: A `bool` providing additional associated metadata.
///
/// # Derives
/// - `Debug`: Enables formatting using the `{:?}` formatter.
/// - `PartialEq`: Enables comparison for equality between two `Unsigned` instances.
/// - `Clone`: Allows cloning of `Unsigned` values for producing duplicates.
#[derive(Debug, PartialEq, Clone)]
pub struct Unsigned(usize, bool);

#[cfg(any(feature = "allow-default-value", test))]
impl Default for Unsigned {
    fn default() -> Self {
        Self(0, true)
    }
}

impl Unsigned {
    /// Parses an optional unsigned value (`usize`) according to given validation rules and returns a result.
    ///
    /// # Arguments
    /// * `s` - An `Option<usize>` representing the input value to be parsed. If `None` is provided, a default value is used.
    /// * `rules` - A `UnsignedRules` instance containing validation rules for the input.
    ///
    /// # Returns
    /// An `Ok(Self)` containing the constructed object, or an `Err(UnsignedError)` if the input fails validation.
    ///
    /// # Errors
    /// Return an `UnsignedError` if the input value does not satisfy the provided validation rules.
    ///
    /// # Behavior
    /// 1. Checks whether the provided value `s` is `None`.
    /// 2. Creates a new `ValidateErrorCollector` for collecting potential validation errors.
    /// 3. Applies the provided `rules` to validate `s`. Any validation issues are collected via the error collector.
    /// 4. If validation errors are present, returns an `UnsignedError` encapsulating the details.
    /// 5. If validation is successful:
    ///     - Constructs and returns the object using the unwrapped value of `s` (or default value if `s` is `None`) and a
    ///       boolean indicator of whether the original input was `None`.
    ///
    pub fn parse_custom(s: Option<usize>, rules: UnsignedRules) -> Result<Self, UnsignedError> {
        let is_none = s.is_none();
        let mut messages = ValidateErrorCollector::new();
        rules.check(&mut messages, s);
        UnsignedError::validate_check(messages)?;
        Ok(Self(s.unwrap_or_default(), is_none))
    }

    /// Parses an optional `usize` value into the current type, applying the default rules.
    ///
    /// # Parameters
    /// - `s`: An `Option<usize>` value to be parsed. If `Some(value)` is provided, the method processes the value;
    ///   if `None`, it skips the parsing and behavior depends on later logic.
    ///
    /// # Returns
    /// - `Result<Self, UnsignedError>`:
    ///   - `Ok(Self)`: If parsing is successful.
    ///   - `Err(UnsignedError)`: If there is a failure during parsing.
    ///
    /// # Errors
    /// This function returns an `UnsignedError` if the provided value does not conform to the expected
    /// format based on `UnsignedRules::default()`.
    ///
    /// # Behavior
    /// Internally calls `Self::parse_custom` with the provided optional value and applies the
    /// default parsing rules defined by `UnsignedRules::default()`.
    pub fn parse(s: Option<usize>) -> Result<Self, UnsignedError> {
        Self::parse_custom(s, UnsignedRules::default())
    }

    /// Returns the inner value of the implementing type as a `usize`.
    ///
    /// This method accesses the inner representation of the type
    /// (assumed to be a tuple struct where the first field is a `usize`)
    /// and returns it directly.
    ///
    /// # Returns
    ///
    /// A `usize` representing the internal value of the instance.
    pub fn as_usize(&self) -> usize {
        self.0
    }

    /// Converts the instance into an `Option<Unsigned>`.
    ///
    /// # Description
    /// This method checks the second element (`self.1`) of the tuple struct instance.
    /// - If `self.1` is `true`, the method returns `None`.
    /// - If `self.1` is `false`, the method wraps the instance (`self`) into a `Some` variant, returning an `Option<Unsigned>`.
    ///
    /// # Returns
    /// - `None`: If the second value (`self.1`) is `true`.
    /// - `Some(Unsigned)`: If the second value (`self.1`) is `false`.
    pub fn into_option(self) -> Option<Unsigned> {
        if self.1 { None } else { Some(self) }
    }
}

pub trait AsUnsignedOnResult {
    fn as_usize(&self) -> usize;
}

impl<E> AsUnsignedOnResult for Result<Unsigned, E> {
    fn as_usize(&self) -> usize {
        self.as_ref().ok().map_or(0, |u| u.as_usize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsigned() {
        let unsigned = Unsigned::parse(Some(10));
        assert!(unsigned.is_ok());
        let unsigned = Unsigned::parse(Some(1000));
        assert!(unsigned.is_err());
    }

    #[test]
    fn test_none_unsigned() {
        let unsigned = Unsigned::parse(None);
        assert!(unsigned.is_err());
    }
}
