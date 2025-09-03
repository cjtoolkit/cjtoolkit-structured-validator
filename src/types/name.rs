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

    fn check(&self, msgs: &mut ValidateErrorCollector, subject: &StringValidator) {
        let (mandatory_rule, length_rule) = self.rules();
        mandatory_rule.check(msgs, subject);
        if !msgs.is_empty() {
            return;
        }
        length_rule.check(msgs, subject);
    }
}

#[derive(Debug, Error, PartialEq, Clone, Default)]
#[error("Name Validation Error")]
pub struct NameError(pub ValidateErrorStore);

impl ValidationCheck for NameError {
    fn validate_new(msgs: ValidateErrorStore) -> Self {
        Self(msgs)
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Name(pub String);

impl Name {
    pub fn parse_custom(s: &str, rules: &NameRules) -> Result<Self, NameError> {
        let subject = s.as_string_validator();
        let mut msgs = ValidateErrorCollector::new();
        rules.check(&mut msgs, &subject);
        NameError::validate_check(msgs)?;
        Ok(Self(s.to_string()))
    }

    pub fn parse(s: &str) -> Result<Self, NameError> {
        Self::parse_custom(s, &NameRules::default())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
