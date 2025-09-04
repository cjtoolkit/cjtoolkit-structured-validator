use crate::base::string_rules::StringMandatoryRules;
use crate::common::locale::{LocaleMessage, ValidateErrorCollector, ValidateErrorStore};
use crate::common::string_validator::{StrValidationExtension, StringValidator};
use crate::common::validation_check::ValidationCheck;
use email_address_parser::EmailAddress;
use thiserror::Error;

pub struct EmailRules {
    pub is_mandatory: bool,
}

impl Default for EmailRules {
    fn default() -> Self {
        Self { is_mandatory: true }
    }
}

impl Into<StringMandatoryRules> for &EmailRules {
    fn into(self) -> StringMandatoryRules {
        StringMandatoryRules {
            is_mandatory: self.is_mandatory,
        }
    }
}

impl EmailRules {
    fn rule(&self) -> StringMandatoryRules {
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
        let rule = self.rule();
        rule.check(messages, subject);
    }
}

#[derive(Debug, Error, PartialEq, Clone, Default)]
#[error("Email Validation Error")]
pub struct EmailError(pub ValidateErrorStore);

impl ValidationCheck for EmailError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Email(Option<EmailAddress>, bool);

impl Default for Email {
    fn default() -> Self {
        Self(None, true)
    }
}

pub enum EmailAddressLocale {
    InvalidEmail,
    DoesNotMatch,
}

impl LocaleMessage for EmailAddressLocale {
    fn get_locale_data(&self) -> crate::common::locale::LocaleData {
        match self {
            Self::InvalidEmail => crate::common::locale::LocaleData {
                name: "validate-email-invalid".to_string(),
                args: Default::default(),
            },
            Self::DoesNotMatch => crate::common::locale::LocaleData {
                name: "validate-email-does-not-match".to_string(),
                args: Default::default(),
            },
        }
    }
}

impl Email {
    pub fn parse_custom(s: Option<&str>, rules: EmailRules) -> Result<Self, EmailError> {
        let is_none = s.is_none();
        let s = s.unwrap_or_default();
        let subject = s.as_string_validator();
        let mut messages = ValidateErrorCollector::new();
        rules.check(&mut messages, &subject, is_none);
        EmailError::validate_check(messages)?;

        let email = match EmailAddress::parse(s, None) {
            Some(email) => email,
            None => {
                let mut messages = ValidateErrorCollector::new();
                messages.push((
                    "Invalid Email".to_string(),
                    Box::new(EmailAddressLocale::InvalidEmail),
                ));
                return Err(EmailError(messages.into()));
            }
        };

        Ok(Self(Some(email), is_none))
    }

    pub fn parse(s: Option<&str>) -> Result<Self, EmailError> {
        Self::parse_custom(s, EmailRules::default())
    }

    pub fn parse_confirm(&self, confirm_email: &str) -> Result<Self, EmailError> {
        let mut messages = ValidateErrorCollector::new();
        if self.0.as_ref().map(|e| e.to_string()) != Some(confirm_email.to_string()) {
            messages.push((
                "Email does not match".to_string(),
                Box::new(EmailAddressLocale::DoesNotMatch),
            ));
        }
        EmailError::validate_check(messages)?;
        Ok(self.clone())
    }

    pub fn as_email(&self) -> Option<&EmailAddress> {
        self.0.as_ref()
    }

    pub fn to_string(&self) -> String {
        self.0.as_ref().map(|e| e.to_string()).unwrap_or_default()
    }

    pub fn into_option(self) -> Option<Email> {
        if self.1 { None } else { Some(self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let email = Email::parse(Some("test@example.com"));
        assert!(email.is_ok());
    }

    #[test]
    fn test_invalid_email() {
        let email = Email::parse(Some("test"));
        assert!(email.is_err());
    }

    #[test]
    fn test_email_confirm_valid() {
        let email = Email::parse(Some("test@example.com")).unwrap_or_default();
        let email_confirm = email.parse_confirm("test@example.com");
        assert!(email_confirm.is_ok());
    }

    #[test]
    fn test_email_confirm_invalid() {
        let email = Email::parse(Some("test@example.com")).unwrap_or_default();
        let email_confirm = email.parse_confirm("test");
        assert!(email_confirm.is_err());
    }
}
