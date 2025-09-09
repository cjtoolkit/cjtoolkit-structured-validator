//! This module contains structures and traits for defining rules for validating strings.

use crate::common::locale::{LocaleData, LocaleMessage, LocaleValue, ValidateErrorCollector};
use crate::common::string_validator::StringValidator;

/// A struct representing a mandatory locale for string processing.
///
/// The `StringMandatoryLocale` struct is a placeholder or marker to enforce the use
/// of a specific locale in processes or systems that require a string to always be
/// associated with a locale. This struct does not currently carry any data or functionality
/// on its own but can be used for typing or enforcing constraints within a program.
///
/// # Use Case
/// - Enforcing locale-specific business logic.
/// - Providing stricter typing in functions or structs requiring locale-based string operations.
/// # Key
/// * `validate-cannot-be-empty`
pub struct StringMandatoryLocale;

impl LocaleMessage for StringMandatoryLocale {
    fn get_locale_data(&self) -> LocaleData {
        LocaleData::new("validate-cannot-be-empty")
    }
}

/// A struct representing rules for mandatory string fields.
///
/// This struct is designed to hold the configuration for whether a particular
/// string field is mandatory or optional. It provides a simple boolean flag
/// indicating the "mandatory" nature of the string.
///
/// # Fields
///
/// * `is_mandatory`
///   - A boolean field that determines whether the string is mandatory.
///   - When set to `true`, the associated string must be provided.
///   - When set to `false`, the associated string is optional.
///
/// # Traits
///
/// * The `Default` trait is implemented for this struct, allowing you to
///   create a default instance where `is_mandatory` is set to `false`.
///
#[derive(Default)]
pub struct StringMandatoryRules {
    pub is_mandatory: bool,
}

impl StringMandatoryRules {
    /// Validates a string based on the provided `StringValidator` and collects validation errors.
    ///
    /// # Parameters
    /// - `messages`: A mutable reference to a `ValidateErrorCollector` that accumulates validation errors encountered during the check.
    /// - `subject`: A reference to a `StringValidator` representing the string to be validated.
    ///
    /// # Behavior
    /// - If the `self.is_mandatory` field is `true` and the `subject` is empty, an error message with the text `"Cannot be empty"`
    ///   is pushed into the `messages` collector along with a locale identifier (`StringMandatoryLocale`).
    ///
    /// # Example
    /// ```
    /// use cjtoolkit_structured_validator::common::locale::ValidateErrorCollector;
    /// use cjtoolkit_structured_validator::common::string_validator::StrValidationExtension;
    /// use cjtoolkit_structured_validator::base::string_rules::StringMandatoryRules;
    /// let mut messages = ValidateErrorCollector::new();
    /// let validator = StringMandatoryRules { is_mandatory: true };
    /// let subject = "".as_string_validator();
    ///
    /// validator.check(&mut messages, &subject);
    ///
    /// assert_eq!(messages.len(), 1); // If the subject is empty and is_mandatory is true, an error will be collected.
    /// ```
    pub fn check(&self, messages: &mut ValidateErrorCollector, subject: &StringValidator) {
        if self.is_mandatory && subject.is_empty() {
            messages.push((
                "Cannot be empty".to_string(),
                Box::new(StringMandatoryLocale),
            ));
        }
    }
}

/// An enumeration representing the constraints for string length,
/// either specifying a minimum length or a maximum length.
///
/// # Variants
///
/// - `MinLength(usize)`
///   Specifies the minimum length that a string is allowed to have.
///   The `usize` represents the minimum number of characters required.
///
/// - `MaxLength(usize)`
///   Specifies the maximum length that a string is allowed to have.
///   The `usize` represents the maximum number of characters allowed.
///
pub enum StringLengthLocale {
    /// Minimum length constraint.
    /// # Key
    /// `validate-min-length`
    MinLength(usize),
    /// Maximum length constraint.
    /// # Key
    /// `validate-max-length`
    MaxLength(usize),
}

impl LocaleMessage for StringLengthLocale {
    fn get_locale_data(&self) -> LocaleData {
        use LocaleData as ld;
        use LocaleValue as lv;
        match self {
            Self::MinLength(min_length) => ld::new_with_vec(
                "validate-min-length",
                vec![("min".to_string(), lv::from(*min_length))],
            ),
            Self::MaxLength(max_length) => ld::new_with_vec(
                "validate-max-length",
                vec![("max".to_string(), lv::from(*max_length))],
            ),
        }
    }
}

/// A structure representing rules for validating the length of a string.
///
/// This struct allows specifying optional minimum and maximum length constraints
/// for a string. Both constraints are optional, meaning one or both of them
/// can be unset depending on the requirements.
///
/// # Fields
/// * `min_length` - An optional minimum length constraint for the string.
///   If set, the string must have at least this many characters to pass validation.
/// * `max_length` - An optional maximum length constraint for the string.
///   If set, the string must not exceed this many characters to pass validation.
///
/// # Defaults
/// When derived using `Default`, both `min_length` and `max_length` will be set to `None`.
///
#[derive(Default)]
pub struct StringLengthRules {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

impl StringLengthRules {
    /// Validates the length of a given string using the specified criteria for minimum and maximum
    /// lengths. If the string does not meet the specified length constraints, an error message is added
    /// to the validation error collector.
    ///
    /// # Parameters
    ///
    /// * `messages` - A mutable reference to a `ValidateErrorCollector` for storing validation error
    ///   messages if any constraints are violated.
    /// * `subject` - A reference to a `StringValidator` that provides the string to validate against
    ///   the defined length rules.
    ///
    /// # Behavior
    ///
    /// 1. If a minimum length (`min_length`) is specified via `self` and the `subject` string's
    ///    grapheme count is less than the minimum; an error message is added to the `messages` collector
    ///    indicating that the string must be at least the specified number of characters.
    /// 2. If a maximum length (`max_length`) is specified via `self` and the `subject` string's
    ///    grapheme count exceeds the maximum, an error message is added to the `messages` collector
    ///    indicating that the string must be at most the specified number of characters.
    ///
    /// # Notes
    ///
    /// This function assumes the `count_graphemes` method is available on the `subject` to properly count
    /// grapheme clusters, ensuring correctness when dealing with multibyte characters or special Unicode
    /// characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cjtoolkit_structured_validator::common::locale::ValidateErrorCollector;
    /// use cjtoolkit_structured_validator::common::string_validator::StrValidationExtension;
    /// use cjtoolkit_structured_validator::base::string_rules::StringLengthRules;
    /// let mut messages = ValidateErrorCollector::new();
    /// let validator = "example".as_string_validator();
    /// let criteria = StringLengthRules { min_length: Some(5), max_length: Some(10) };
    ///
    /// criteria.check(&mut messages, &validator);
    ///
    /// assert!(messages.is_empty()); // The string "example" satisfies the length constraints.
    /// ```
    pub fn check(&self, messages: &mut ValidateErrorCollector, subject: &StringValidator) {
        if let Some(min_length) = self.min_length {
            if subject.count_graphemes() < min_length {
                messages.push((
                    format!("Must be at least {} characters", min_length),
                    Box::new(StringLengthLocale::MinLength(min_length)),
                ));
            }
        }
        if let Some(max_length) = self.max_length {
            if subject.count_graphemes() > max_length {
                messages.push((
                    format!("Must be at most {} characters", max_length),
                    Box::new(StringLengthLocale::MaxLength(max_length)),
                ));
            }
        }
    }
}

/// An enumeration defining various string constraints or requirements based on the presence of
/// special characters, case sensitivity, or digits.
///
/// This enum can be used to specify which kind of validation or rules should be applied
/// to a string across different locales, ensuring compliance with specific character requirements.
///
/// # Variants
///
/// - `MustHaveSpecialChars`
///   Enforces that the string must contain at least one special character
///   (e.g., symbols like `@`, `#`, `$`, etc.).
///
/// - `MustHaveUppercaseAndLowercase`
///   Enforces that the string must contain at least one uppercase and one lowercase character.
///
///
/// - `MustHaveUppercase`
///   Enforces that the string must contain at least one uppercase character.
///
/// - `MustHaveLowercase`
///   Enforces that the string must contain at least one lowercase character.
///
/// - `MustHaveDigit`
///   Enforces that the string must contain at least one numeric digit (0-9).
///
pub enum StringSpecialCharLocale {
    /// Must have special characters.
    /// # Key
    /// `validate-must-have-special-chars`
    MustHaveSpecialChars,
    /// Must have uppercase and lowercase characters.
    /// # Key
    /// `validate-must-have-uppercase-and-lowercase`
    MustHaveUppercaseAndLowercase,
    /// Must have uppercase characters.
    /// # Key
    /// `validate-must-have-uppercase`
    MustHaveUppercase,
    /// Must have lowercase characters.
    /// # Key
    /// `validate-must-have-lowercase`
    MustHaveLowercase,
    /// Must have digits.
    /// # Key
    /// `validate-must-have-digit`
    MustHaveDigit,
}

impl LocaleMessage for StringSpecialCharLocale {
    fn get_locale_data(&self) -> LocaleData {
        use LocaleData as ld;
        match self {
            Self::MustHaveSpecialChars => ld::new("validate-must-have-special-chars"),
            Self::MustHaveUppercaseAndLowercase => {
                ld::new("validate-must-have-uppercase-and-lowercase")
            }
            Self::MustHaveUppercase => ld::new("validate-must-have-uppercase"),
            Self::MustHaveLowercase => ld::new("validate-must-have-lowercase"),
            Self::MustHaveDigit => ld::new("validate-must-have-digit"),
        }
    }
}

/// A structure that defines rules for validating the presence
/// of characters in a string. This can be used to enforce certain validation criteria
/// for strings containing uppercase letters, lowercase letters, special characters,
/// and numeric digits.
///
/// # Fields
///
/// * `must_have_uppercase` - A boolean flag indicating whether the string must contain
///   at least one uppercase letter (`true` if required, `false` otherwise).
///
/// * `must_have_lowercase` - A boolean flag indicating whether the string must contain
///   at least one lowercase letter (`true` if required, `false` otherwise).
///
/// * `must_have_special_chars` - A boolean flag indicating whether the string must contain
///   at least one special character (e.g., `!`, `@`, `#`, etc.) (`true` if required, `false` otherwise).
///
/// * `must_have_digit` - A boolean flag indicating whether the string must contain
///   at least one numeric digit (`true` if required, `false` otherwise).
///
/// # Default Implementation
///
/// By default, all fields are set to `false`, meaning no specific character requirements
/// will be enforced unless explicitly configured.
///
/// This structure can be used in validation logic where customizable character rules
/// are required, such as password or input string checks.
///
#[derive(Default)]
pub struct StringSpecialCharRules {
    pub must_have_uppercase: bool,
    pub must_have_lowercase: bool,
    pub must_have_special_chars: bool,
    pub must_have_digit: bool,
}

impl StringSpecialCharRules {
    /// Validates a string based on multiple constraints such as the presence of special characters,
    /// uppercase and lowercase letters, and digits. If any constraint is not met, an error
    /// message along with the corresponding error locale is added to the provided `ValidateErrorCollector`.
    ///
    /// # Parameters
    ///
    /// * `messages`: A mutable reference to a `ValidateErrorCollector`, which collects validation errors
    ///   encountered during the checks.
    /// * `subject`: A reference to a `StringValidator` object, which provides methods to check
    ///   various string properties based on constraints.
    ///
    /// # Behavior
    ///
    /// - If `must_have_special_chars` is true, the method checks whether the subject contains
    ///   at least one special character. If not, an error is added to `messages`.
    /// - If both `must_have_uppercase` and `must_have_lowercase` are true, the method verifies
    ///   that the subject has at least one uppercase and one lowercase letter. An error is added
    ///   if this condition is not met.
    /// - If only `must_have_uppercase` is true, the method ensures the presence of at least one
    ///   uppercase letter in the subject, adding an error if the condition fails.
    /// - If only `must_have_lowercase` is true, the method ensures the presence of at least one
    ///   lowercase letter in the subject, adding an error if the condition fails.
    /// - If `must_have_digit` is true, the method checks that the subject contains at least one
    ///   numeric digit. If not, an error is added to `messages`.
    ///
    /// # Error Handling
    ///
    /// Each validation failure results in an entry being added to the `ValidateErrorCollector`,
    /// consisting of an error message string and a corresponding locale represented by `StringSpecialCharLocale`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cjtoolkit_structured_validator::common::locale::ValidateErrorCollector;
    /// use cjtoolkit_structured_validator::common::string_validator::StrValidationExtension;
    /// use cjtoolkit_structured_validator::base::string_rules::StringSpecialCharRules;
    /// let mut errors = ValidateErrorCollector::new();
    /// let validator = "Password123!".as_string_validator();
    /// let rules = StringSpecialCharRules {
    ///     must_have_special_chars: true,
    ///     must_have_uppercase: true,
    ///     must_have_lowercase: true,
    ///     must_have_digit: true,
    /// };
    ///
    /// rules.check(&mut errors, &validator);
    ///
    /// if errors.is_empty() {
    ///     println!("Validation passed!");
    /// } else {
    ///     println!("Validation failed with errors");
    /// }
    /// ```
    pub fn check(&self, messages: &mut ValidateErrorCollector, subject: &StringValidator) {
        if self.must_have_special_chars {
            if !subject.has_special_chars() {
                messages.push((
                    "Must contain at least one special character".to_string(),
                    Box::new(StringSpecialCharLocale::MustHaveSpecialChars),
                ));
            }
        }
        if self.must_have_uppercase && self.must_have_lowercase {
            if !subject.has_ascii_uppercase_and_lowercase() {
                messages.push((
                    "Must contain at least one uppercase and lowercase letter".to_string(),
                    Box::new(StringSpecialCharLocale::MustHaveUppercaseAndLowercase),
                ));
            }
        } else {
            if self.must_have_uppercase {
                if !subject.has_ascii_uppercase() {
                    messages.push((
                        "Must contain at least one uppercase letter".to_string(),
                        Box::new(StringSpecialCharLocale::MustHaveUppercase),
                    ));
                }
            }
            if self.must_have_lowercase {
                if !subject.has_ascii_lowercase() {
                    messages.push((
                        "Must contain at least one lowercase letter".to_string(),
                        Box::new(StringSpecialCharLocale::MustHaveLowercase),
                    ));
                }
            }
        }
        if self.must_have_digit {
            if !subject.has_ascii_digit() {
                messages.push((
                    "Must contain at least one digit".to_string(),
                    Box::new(StringSpecialCharLocale::MustHaveDigit),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::string_validator::StrValidationExtension;

    mod string_mandatory_rule {
        use super::*;

        #[test]
        fn test_string_mandatory_rule_check_empty_string() {
            let mut messages = ValidateErrorCollector::new();
            let subject = "".as_string_validator();
            let rule = StringMandatoryRules { is_mandatory: true };
            rule.check(&mut messages, &subject);
            assert_eq!(messages.len(), 1);
            assert_eq!(messages.0[0].0, "Cannot be empty");
        }

        #[test]
        fn test_string_mandatory_rule_check_not_empty_string() {
            let mut messages = ValidateErrorCollector::new();
            let subject = "Hello".as_string_validator();
            let rule = StringMandatoryRules { is_mandatory: true };
            rule.check(&mut messages, &subject);
            assert_eq!(messages.len(), 0);
        }
    }

    mod string_length_rule {
        use super::*;

        #[test]
        fn test_string_length_rule_check_empty_string() {
            let mut messages = ValidateErrorCollector::new();
            let subject = "".as_string_validator();
            let rule = StringLengthRules {
                min_length: Some(5),
                max_length: Some(10),
            };
            rule.check(&mut messages, &subject);
            assert_eq!(messages.len(), 1);
            assert_eq!(messages.0[0].0, "Must be at least 5 characters");
        }

        #[test]
        fn test_string_length_rule_check_too_long_string() {
            let mut messages = ValidateErrorCollector::new();
            let subject = "Hello".as_string_validator();
            let rule = StringLengthRules {
                min_length: Some(2),
                max_length: Some(4),
            };
            rule.check(&mut messages, &subject);
            assert_eq!(messages.len(), 1);
            assert_eq!(messages.0[0].0, "Must be at most 4 characters");
        }
    }

    mod string_special_char_rule {
        use super::*;

        #[test]
        fn test_string_special_char_rule_check_empty_string() {
            let mut messages = ValidateErrorCollector::new();
            let subject = "".as_string_validator();
            let rule = StringSpecialCharRules {
                must_have_uppercase: true,
                must_have_lowercase: true,
                must_have_special_chars: true,
                must_have_digit: true,
            };
            rule.check(&mut messages, &subject);
            assert_eq!(messages.len(), 3);
            assert_eq!(
                messages.0[0].0,
                "Must contain at least one special character"
            );
            assert_eq!(
                messages.0[1].0,
                "Must contain at least one uppercase and lowercase letter"
            );
            assert_eq!(messages.0[2].0, "Must contain at least one digit");
        }

        #[test]
        fn test_string_special_char_rule_check_not_empty_string() {
            let mut messages = ValidateErrorCollector::new();
            let subject = "Hello".as_string_validator();
            let rule = StringSpecialCharRules {
                must_have_uppercase: true,
                must_have_lowercase: true,
                must_have_special_chars: true,
                must_have_digit: true,
            };
            rule.check(&mut messages, &subject);
            assert_eq!(messages.len(), 2);
            assert_eq!(
                messages.0[0].0,
                "Must contain at least one special character"
            );
            assert_eq!(messages.0[1].0, "Must contain at least one digit");
        }

        #[test]
        fn test_string_special_char_rule_check_not_empty_string_with_uppercase_and_lowercase_and_symbol()
         {
            let mut messages = ValidateErrorCollector::new();
            let subject = "Hello@".as_string_validator();
            let rule = StringSpecialCharRules {
                must_have_uppercase: true,
                must_have_lowercase: true,
                must_have_special_chars: true,
                must_have_digit: true,
            };
            rule.check(&mut messages, &subject);
            assert_eq!(messages.len(), 1);
            assert_eq!(messages.0[0].0, "Must contain at least one digit");
        }

        #[test]
        fn test_string_special_char_rule_check_not_empty_string_with_uppercase_and_lowercase_and_digit()
         {
            let mut messages = ValidateErrorCollector::new();
            let subject = "Hello1".as_string_validator();
            let rule = StringSpecialCharRules {
                must_have_uppercase: true,
                must_have_lowercase: true,
                must_have_special_chars: true,
                must_have_digit: true,
            };
            rule.check(&mut messages, &subject);
            assert_eq!(messages.len(), 1);
            assert_eq!(
                messages.0[0].0,
                "Must contain at least one special character"
            );
        }

        #[test]
        fn test_string_special_char_rule_check_not_empty_string_with_uppercase_and_lowercase_digit_and_symbol()
         {
            let mut messages = ValidateErrorCollector::new();
            let subject = "Hello1@".as_string_validator();
            let rule = StringSpecialCharRules {
                must_have_uppercase: true,
                must_have_lowercase: true,
                must_have_special_chars: true,
                must_have_digit: true,
            };
            rule.check(&mut messages, &subject);
            assert_eq!(messages.len(), 0);
        }
    }
}
