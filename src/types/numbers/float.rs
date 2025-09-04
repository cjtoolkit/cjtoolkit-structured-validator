use crate::base::number_rules::{NumberMandatoryRules, NumberRangeRules};
use crate::common::locale::{ValidateErrorCollector, ValidateErrorStore};
use crate::common::validation_check::ValidationCheck;

pub struct FloatRules {
    pub is_mandatory: bool,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

impl Default for FloatRules {
    fn default() -> Self {
        Self {
            is_mandatory: true,
            min: Some(0.0),
            max: Some(255.0),
        }
    }
}

impl Into<(NumberMandatoryRules, NumberRangeRules<f64>)> for &FloatRules {
    fn into(self) -> (NumberMandatoryRules, NumberRangeRules<f64>) {
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

impl FloatRules {
    fn rules(&self) -> (NumberMandatoryRules, NumberRangeRules<f64>) {
        self.into()
    }

    fn check(&self, messages: &mut ValidateErrorCollector, subject: Option<f64>) {
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

#[derive(Debug, PartialEq, Clone, Default)]
pub struct FloatError(pub ValidateErrorStore);

impl ValidationCheck for FloatError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Float(f64, bool);

impl Float {
    pub fn parse_custom(s: Option<f64>, rules: FloatRules) -> Result<Self, FloatError> {
        let is_none = s.is_none();
        let mut messages = ValidateErrorCollector::new();
        rules.check(&mut messages, s);
        FloatError::validate_check(messages)?;
        Ok(Self(s.unwrap_or_default(), is_none))
    }

    pub fn parse(s: Option<f64>) -> Result<Self, FloatError> {
        Self::parse_custom(s, FloatRules::default())
    }

    pub fn as_f64(&self) -> f64 {
        self.0
    }

    pub fn into_option(self) -> Option<Float> {
        if self.1 { None } else { Some(self) }
    }
}
