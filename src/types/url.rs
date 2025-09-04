use crate::base::string_rules::StringMandatoryRules;
use crate::common::locale::{LocaleMessage, ValidateErrorCollector, ValidateErrorStore};
use crate::common::string_validator::{StrValidationExtension, StringValidator};
use crate::common::validation_check::ValidationCheck;
use thiserror::Error;
use url::Url as UrlValue;

pub struct UrlRules {
    pub is_mandatory: bool,
}

impl Default for UrlRules {
    fn default() -> Self {
        Self { is_mandatory: true }
    }
}

impl Into<StringMandatoryRules> for &UrlRules {
    fn into(self) -> StringMandatoryRules {
        StringMandatoryRules {
            is_mandatory: self.is_mandatory,
        }
    }
}

impl UrlRules {
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
#[error("Url Validation Error")]
pub struct UrlError(pub ValidateErrorStore);

impl ValidationCheck for UrlError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Url(Option<UrlValue>, bool);

impl Default for Url {
    fn default() -> Self {
        Self(None, true)
    }
}

pub struct UrlValueLocale;

impl LocaleMessage for UrlValueLocale {
    fn get_locale_data(&self) -> crate::common::locale::LocaleData {
        crate::common::locale::LocaleData {
            name: "validate-url-value".to_string(),
            args: Default::default(),
        }
    }
}

impl Url {
    pub fn parse_custom(s: Option<&str>, rules: UrlRules) -> Result<Self, UrlError> {
        let is_none = s.is_none();
        let s = s.unwrap_or_default();
        let subject = s.as_string_validator();
        let mut messages = ValidateErrorCollector::new();
        rules.check(&mut messages, &subject, is_none);
        UrlError::validate_check(messages)?;
        let url = match UrlValue::parse(s) {
            Ok(url) => url,
            Err(_) => {
                let mut messages = ValidateErrorCollector::new();
                messages.push(("Invalid Url".to_string(), Box::new(UrlValueLocale)));
                return Err(UrlError(messages.into()));
            }
        };

        Ok(Self(Some(url), is_none))
    }

    pub fn parse(s: Option<&str>) -> Result<Self, UrlError> {
        Self::parse_custom(s, UrlRules::default())
    }

    pub fn as_url(&self) -> Option<&UrlValue> {
        self.0.as_ref()
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref().map(|url| url.as_str()).unwrap_or_default()
    }

    pub fn into_option(self) -> Option<Url> {
        if self.1 { None } else { Some(self) }
    }
}
