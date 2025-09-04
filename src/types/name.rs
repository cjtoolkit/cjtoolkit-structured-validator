use crate::base::string_rules::{StringLengthRules, StringMandatoryRules};
use crate::common::locale::{ValidateErrorCollector, ValidateErrorStore};
use crate::common::string_validator::{StrValidationExtension, StringValidator};
use crate::common::validation_check::ValidationCheck;
use thiserror::Error;

pub struct NameRules {
    pub is_mandatory: bool,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

impl Default for NameRules {
    fn default() -> Self {
        Self {
            is_mandatory: true,
            min_length: Some(5),
            max_length: Some(20),
        }
    }
}

impl Into<(StringMandatoryRules, StringLengthRules)> for &NameRules {
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

impl NameRules {
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
#[error("Name Validation Error")]
pub struct NameError(pub ValidateErrorStore);

impl ValidationCheck for NameError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Name(String, bool);

impl Default for Name {
    fn default() -> Self {
        Self(String::new(), true)
    }
}

impl Name {
    pub fn parse_custom(s: Option<&str>, rules: NameRules) -> Result<Self, NameError> {
        let is_none = s.is_none();
        let s = s.unwrap_or_default();
        let subject = s.as_string_validator();
        let mut messages = ValidateErrorCollector::new();
        rules.check(&mut messages, &subject, is_none);
        NameError::validate_check(messages)?;
        Ok(Self(s.to_string(), is_none))
    }

    pub fn parse(s: Option<&str>) -> Result<Self, NameError> {
        Self::parse_custom(s, NameRules::default())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_option(self) -> Option<Name> {
        if self.1 { None } else { Some(self) }
    }
}
