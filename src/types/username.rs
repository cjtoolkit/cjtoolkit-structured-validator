use crate::base::string_rules::{StringLengthRules, StringMandatoryRules};
use crate::common::locale::{
    LocaleData, LocaleMessage, ValidateErrorCollector, ValidateErrorStore,
};
use crate::common::string_validator::{StrValidationExtension, StringValidator};
use crate::common::validation_check::ValidationCheck;
use thiserror::Error;

pub struct UsernameRules {
    pub is_mandatory: bool,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

impl Default for UsernameRules {
    fn default() -> Self {
        Self {
            is_mandatory: true,
            min_length: None,
            max_length: Some(40),
        }
    }
}

impl Into<(StringMandatoryRules, StringLengthRules)> for &UsernameRules {
    fn into(self) -> (StringMandatoryRules, StringLengthRules) {
        (
            StringMandatoryRules {
                is_mandatory: self.is_mandatory,
            },
            StringLengthRules {
                min_length: self.min_length,
                max_length: self.max_length,
            },
        )
    }
}

impl UsernameRules {
    fn rules(&self) -> (StringMandatoryRules, StringLengthRules) {
        self.into()
    }

    fn check(
        &self,
        messages: &mut ValidateErrorCollector,
        subject: &StringValidator,
        is_none: bool,
    ) {
        if !self.is_mandatory && is_none {
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

#[derive(Debug, Error, PartialEq, Clone, Default)]
#[error("Username Validation Error")]
pub struct UsernameError(pub ValidateErrorStore);

impl ValidationCheck for UsernameError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Username(String, bool);

pub trait IsUsernameTaken {
    fn is_username_taken(&self, username: &str) -> bool;
}

pub trait IsUsernameTakenAsync {
    fn is_username_taken_async(&self, username: &str) -> impl Future<Output = bool>;
}

pub struct UsernameTakenLocale;

impl LocaleMessage for UsernameTakenLocale {
    fn get_locale_data(&self) -> LocaleData {
        LocaleData {
            name: "validate-username-taken".to_string(),
            args: Default::default(),
        }
    }
}

impl Username {
    pub fn parse_custom(s: Option<&str>, rules: UsernameRules) -> Result<Self, UsernameError> {
        let is_none = s.is_none();
        let s = s.unwrap_or_default();
        let subject = s.as_string_validator();
        let mut messages = ValidateErrorCollector::new();
        rules.check(&mut messages, &subject, is_none);
        UsernameError::validate_check(messages)?;
        Ok(Self(s.to_string(), is_none))
    }

    pub fn parse(s: Option<&str>) -> Result<Self, UsernameError> {
        Self::parse_custom(s, UsernameRules::default())
    }

    pub fn check_username_taken<T: IsUsernameTaken>(
        &self,
        service: &T,
    ) -> Result<Self, UsernameError> {
        let mut messages = ValidateErrorCollector::new();

        service.is_username_taken(self.as_str()).then(|| {
            messages.push(("Already taken".to_string(), Box::new(UsernameTakenLocale)));
        });

        UsernameError::validate_check(messages)?;
        Ok(self.clone())
    }

    pub async fn check_username_taken_async<T: IsUsernameTakenAsync>(
        &self,
        service: &T,
    ) -> Result<Self, UsernameError> {
        let mut messages = ValidateErrorCollector::new();

        service
            .is_username_taken_async(self.as_str())
            .await
            .then(|| {
                messages.push(("Already taken".to_string(), Box::new(UsernameTakenLocale)));
            });

        UsernameError::validate_check(messages)?;
        Ok(self.clone())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_option(self) -> Option<Username> {
        if self.1 { None } else { Some(self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeUsernameCheckService(String);

    impl IsUsernameTaken for FakeUsernameCheckService {
        fn is_username_taken(&self, username: &str) -> bool {
            username == self.0.as_str()
        }
    }

    #[test]
    fn username_is_taken() {
        let username_result = Username("taken".to_string(), false);

        assert!(
            username_result
                .check_username_taken(&FakeUsernameCheckService("taken".to_string()))
                .is_err()
        )
    }

    #[test]
    fn username_is_not_taken() {
        let username_result = Username("not_taken".to_string(), false);

        assert!(
            username_result
                .check_username_taken(&FakeUsernameCheckService("taken".to_string()))
                .is_ok()
        )
    }
}
