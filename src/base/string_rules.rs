use crate::common::locale::{LocaleData, LocaleMessage, LocaleValue, ValidateErrorCollector};
use crate::common::string_validator::StringValidator;

pub struct StringMandatoryLocale;

impl LocaleMessage for StringMandatoryLocale {
    fn get_locale_data(&self) -> LocaleData {
        LocaleData {
            name: "validate-cannot-be-empty".to_string(),
            args: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct StringMandatoryRules {
    pub is_mandatory: bool,
}

impl StringMandatoryRules {
    pub fn check(&self, msgs: &mut ValidateErrorCollector, subject: &StringValidator) {
        if self.is_mandatory && subject.is_empty() {
            msgs.push((
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
        match self {
            Self::MinLength(min_length) => LocaleData {
                name: "validate-min-length".to_string(),
                args: vec![("min".to_string(), LocaleValue::Uint(*min_length))]
                    .into_iter()
                    .collect(),
            },
            Self::MaxLength(max_length) => LocaleData {
                name: "validate-max-length".to_string(),
                args: vec![("max".to_string(), LocaleValue::Uint(*max_length))]
                    .into_iter()
                    .collect(),
            },
        }
    }
}

#[derive(Default)]
pub struct StringLengthRules {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

impl StringLengthRules {
    pub fn check(&self, msgs: &mut ValidateErrorCollector, subject: &StringValidator) {
        if let Some(min_length) = self.min_length {
            if subject.count_graphemes() < min_length {
                msgs.push((
                    format!("Must be at least {} characters", min_length),
                    Box::new(StringLengthLocale::MinLength(min_length)),
                ));
            }
        }
        if let Some(max_length) = self.max_length {
            if subject.count_graphemes() > max_length {
                msgs.push((
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
        match self {
            Self::MustHaveSpecialChars => LocaleData {
                name: "validate-must-have-special-chars".to_string(),
                args: Default::default(),
            },
            Self::MustHaveUppercaseAndLowercase => LocaleData {
                name: "validate-must-have-uppercase-and-lowercase".to_string(),
                args: Default::default(),
            },
            Self::MustHaveUppercase => LocaleData {
                name: "validate-must-have-uppercase".to_string(),
                args: Default::default(),
            },
            Self::MustHaveLowercase => LocaleData {
                name: "validate-must-have-lowercase".to_string(),
                args: Default::default(),
            },
            Self::MustHaveDigit => LocaleData {
                name: "validate-must-have-digit".to_string(),
                args: Default::default(),
            },
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
    pub fn check(&self, msgs: &mut ValidateErrorCollector, subject: &StringValidator) {
        if self.must_have_special_chars {
            if !subject.has_special_chars() {
                msgs.push((
                    "Must contain at least one special character".to_string(),
                    Box::new(StringSpecialCharLocale::MustHaveSpecialChars),
                ));
            }
        }
        if self.must_have_uppercase && self.must_have_lowercase {
            if !subject.has_ascii_uppercase_and_lowercase() {
                msgs.push((
                    "Must contain at least one uppercase and lowercase letter".to_string(),
                    Box::new(StringSpecialCharLocale::MustHaveUppercaseAndLowercase),
                ));
            }
        } else {
            if self.must_have_uppercase {
                if !subject.has_ascii_uppercase() {
                    msgs.push((
                        "Must contain at least one uppercase letter".to_string(),
                        Box::new(StringSpecialCharLocale::MustHaveUppercase),
                    ));
                }
            }
            if self.must_have_lowercase {
                if !subject.has_ascii_lowercase() {
                    msgs.push((
                        "Must contain at least one lowercase letter".to_string(),
                        Box::new(StringSpecialCharLocale::MustHaveLowercase),
                    ));
                }
            }
        }
        if self.must_have_digit {
            if !subject.has_ascii_digit() {
                msgs.push((
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
            let mut msgs = ValidateErrorCollector::new();
            let subject = "".as_string_validator();
            let rule = StringMandatoryRules { is_mandatory: true };
            rule.check(&mut msgs, &subject);
            assert_eq!(msgs.len(), 1);
            assert_eq!(msgs.0[0].0, "Cannot be empty");
        }

        #[test]
        fn test_string_mandatory_rule_check_not_empty_string() {
            let mut msgs = ValidateErrorCollector::new();
            let subject = "Hello".as_string_validator();
            let rule = StringMandatoryRules { is_mandatory: true };
            rule.check(&mut msgs, &subject);
            assert_eq!(msgs.len(), 0);
        }
    }

    mod string_length_rule {
        use super::*;

        #[test]
        fn test_string_length_rule_check_empty_string() {
            let mut msgs = ValidateErrorCollector::new();
            let subject = "".as_string_validator();
            let rule = StringLengthRules {
                min_length: Some(5),
                max_length: Some(10),
            };
            rule.check(&mut msgs, &subject);
            assert_eq!(msgs.len(), 1);
            assert_eq!(msgs.0[0].0, "Must be at least 5 characters");
        }

        #[test]
        fn test_string_length_rule_check_too_long_string() {
            let mut msgs = ValidateErrorCollector::new();
            let subject = "Hello".as_string_validator();
            let rule = StringLengthRules {
                min_length: Some(2),
                max_length: Some(4),
            };
            rule.check(&mut msgs, &subject);
            assert_eq!(msgs.len(), 1);
            assert_eq!(msgs.0[0].0, "Must be at most 4 characters");
        }
    }

    mod string_special_char_rule {
        use super::*;

        #[test]
        fn test_string_special_char_rule_check_empty_string() {
            let mut msgs = ValidateErrorCollector::new();
            let subject = "".as_string_validator();
            let rule = StringSpecialCharRules {
                must_have_uppercase: true,
                must_have_lowercase: true,
                must_have_special_chars: true,
                must_have_digit: true,
            };
            rule.check(&mut msgs, &subject);
            assert_eq!(msgs.len(), 3);
            assert_eq!(msgs.0[0].0, "Must contain at least one special character");
            assert_eq!(
                msgs.0[1].0,
                "Must contain at least one uppercase and lowercase letter"
            );
            assert_eq!(msgs.0[2].0, "Must contain at least one digit");
        }

        #[test]
        fn test_string_special_char_rule_check_not_empty_string() {
            let mut msgs = ValidateErrorCollector::new();
            let subject = "Hello".as_string_validator();
            let rule = StringSpecialCharRules {
                must_have_uppercase: true,
                must_have_lowercase: true,
                must_have_special_chars: true,
                must_have_digit: true,
            };
            rule.check(&mut msgs, &subject);
            assert_eq!(msgs.len(), 2);
            assert_eq!(msgs.0[0].0, "Must contain at least one special character");
            assert_eq!(msgs.0[1].0, "Must contain at least one digit");
        }

        #[test]
        fn test_string_special_char_rule_check_not_empty_string_with_uppercase_and_lowercase_and_symbol()
         {
            let mut msgs = ValidateErrorCollector::new();
            let subject = "Hello@".as_string_validator();
            let rule = StringSpecialCharRules {
                must_have_uppercase: true,
                must_have_lowercase: true,
                must_have_special_chars: true,
                must_have_digit: true,
            };
            rule.check(&mut msgs, &subject);
            assert_eq!(msgs.len(), 1);
            assert_eq!(msgs.0[0].0, "Must contain at least one digit");
        }

        #[test]
        fn test_string_special_char_rule_check_not_empty_string_with_uppercase_and_lowercase_and_digit()
         {
            let mut msgs = ValidateErrorCollector::new();
            let subject = "Hello1".as_string_validator();
            let rule = StringSpecialCharRules {
                must_have_uppercase: true,
                must_have_lowercase: true,
                must_have_special_chars: true,
                must_have_digit: true,
            };
            rule.check(&mut msgs, &subject);
            assert_eq!(msgs.len(), 1);
            assert_eq!(msgs.0[0].0, "Must contain at least one special character");
        }

        #[test]
        fn test_string_special_char_rule_check_not_empty_string_with_uppercase_and_lowercase_digit_and_symbol()
         {
            let mut msgs = ValidateErrorCollector::new();
            let subject = "Hello1@".as_string_validator();
            let rule = StringSpecialCharRules {
                must_have_uppercase: true,
                must_have_lowercase: true,
                must_have_special_chars: true,
                must_have_digit: true,
            };
            rule.check(&mut msgs, &subject);
            assert_eq!(msgs.len(), 0);
        }
    }
}
