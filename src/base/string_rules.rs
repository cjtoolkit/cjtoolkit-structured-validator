use crate::common::locale::{LocaleData, LocaleMessage, LocaleValue, ValidateErrorCollector};
use crate::common::string_validator::StringValidator;

pub struct StringMandatoryLocale;

impl LocaleMessage for StringMandatoryLocale {
    fn get_locale_data(&self) -> LocaleData {
        LocaleData::new("validate-cannot-be-empty")
    }
}

#[derive(Default)]
pub struct StringMandatoryRules {
    pub is_mandatory: bool,
}

impl StringMandatoryRules {
    pub fn check(&self, messages: &mut ValidateErrorCollector, subject: &StringValidator) {
        if self.is_mandatory && subject.is_empty() {
            messages.push((
                "Cannot be empty".to_string(),
                Box::new(StringMandatoryLocale),
            ));
        }
    }
}

pub enum StringLengthLocale {
    MinLength(usize),
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

#[derive(Default)]
pub struct StringLengthRules {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

impl StringLengthRules {
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

pub enum StringSpecialCharLocale {
    MustHaveSpecialChars,
    MustHaveUppercaseAndLowercase,
    MustHaveUppercase,
    MustHaveLowercase,
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

#[derive(Default)]
pub struct StringSpecialCharRules {
    pub must_have_uppercase: bool,
    pub must_have_lowercase: bool,
    pub must_have_special_chars: bool,
    pub must_have_digit: bool,
}

impl StringSpecialCharRules {
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
