use crate::base::string_rules::{StringLengthRules, StringMandatoryRules, StringSpecialCharRules};
use crate::common::locale::{
    LocaleData, LocaleMessage, ValidateErrorCollector, ValidateErrorStore,
};
use crate::common::string_validator::{StrValidationExtension, StringValidator};
use crate::common::validation_check::ValidationCheck;
use thiserror::Error;

pub struct PasswordRules {
    pub is_mandatory: bool,
    pub must_have_uppercase: bool,
    pub must_have_lowercase: bool,
    pub must_have_special_chars: bool,
    pub must_have_digit: bool,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

impl Default for PasswordRules {
    fn default() -> Self {
        Self {
            is_mandatory: true,
            must_have_uppercase: true,
            must_have_lowercase: true,
            must_have_special_chars: true,
            must_have_digit: true,
            min_length: Some(8),
            max_length: Some(64),
        }
    }
}

impl
    Into<(
        StringMandatoryRules,
        StringLengthRules,
        StringSpecialCharRules,
    )> for &PasswordRules
{
    fn into(
        self,
    ) -> (
        StringMandatoryRules,
        StringLengthRules,
        StringSpecialCharRules,
    ) {
        (
            StringMandatoryRules {
                is_mandatory: self.is_mandatory,
            },
            StringLengthRules {
                min_length: self.min_length,
                max_length: self.max_length,
            },
            StringSpecialCharRules {
                must_have_uppercase: self.must_have_uppercase,
                must_have_lowercase: self.must_have_lowercase,
                must_have_special_chars: self.must_have_special_chars,
                must_have_digit: self.must_have_digit,
            },
        )
    }
}

impl PasswordRules {
    fn rules(
        &self,
    ) -> (
        StringMandatoryRules,
        StringLengthRules,
        StringSpecialCharRules,
    ) {
        self.into()
    }

    fn check(&self, msgs: &mut ValidateErrorCollector, subject: &StringValidator) {
        let (mandatory_rule, length_rule, special_char_rule) = self.rules();
        mandatory_rule.check(msgs, subject);
        if !msgs.is_empty() {
            return;
        }
        length_rule.check(msgs, subject);
        special_char_rule.check(msgs, subject);
    }
}

#[derive(Debug, Error, PartialEq, Clone, Default)]
#[error("Password Validation Error")]
pub struct PasswordError(pub ValidateErrorStore);

impl ValidationCheck for PasswordError {
    fn validate_new(msgs: ValidateErrorStore) -> Self {
        Self(msgs)
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Password(pub String);

pub struct PasswordDoesNotMatchLocale;

impl LocaleMessage for PasswordDoesNotMatchLocale {
    fn get_locale_data(&self) -> LocaleData {
        LocaleData {
            name: "validate-password-does-not-match".to_string(),
            args: Default::default(),
        }
    }
}

impl Password {
    pub fn parse_custom(s: &str, rules: &PasswordRules) -> Result<Self, PasswordError> {
        let subject = s.as_string_validator();
        let mut msgs = ValidateErrorCollector::new();
        rules.check(&mut msgs, &subject);
        PasswordError::validate_check(msgs)?;
        Ok(Self(s.to_string()))
    }

    pub fn parse(s: &str) -> Result<Self, PasswordError> {
        Self::parse_custom(s, &PasswordRules::default())
    }

    pub fn parse_confirm(&self, password_confirm: &str) -> Result<Self, PasswordError> {
        let mut msgs = ValidateErrorCollector::new();

        (password_confirm != self.as_str()).then(|| {
            msgs.push((
                "Password does not match".to_string(),
                Box::new(PasswordDoesNotMatchLocale),
            ));
        });

        PasswordError::validate_check(msgs)?;
        Ok(self.clone())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_parse_error_password_confirmation_mismatch() {
        let password = Password("match".to_string());
        let password = password.parse_confirm("mismatch");
        assert!(password.is_err());
    }

    #[test]
    fn test_password_parse_error_password_confirmation_match() {
        let password = Password("match".to_string());
        let password = password.parse_confirm("match");
        assert!(password.is_ok());
    }
}
