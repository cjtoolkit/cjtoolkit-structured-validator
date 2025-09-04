use crate::base::number_rules::{NumberMandatoryRules, NumberRangeRules};
use crate::common::locale::{ValidateErrorCollector, ValidateErrorStore};
use crate::common::validation_check::ValidationCheck;

pub struct UnsignedRules {
    pub is_mandatory: bool,
    pub min: Option<usize>,
    pub max: Option<usize>,
}

impl Default for UnsignedRules {
    fn default() -> Self {
        Self {
            is_mandatory: true,
            min: Some(0),
            max: Some(255),
        }
    }
}

impl Into<(NumberMandatoryRules, NumberRangeRules<usize>)> for &UnsignedRules {
    fn into(self) -> (NumberMandatoryRules, NumberRangeRules<usize>) {
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

impl UnsignedRules {
    fn rules(&self) -> (NumberMandatoryRules, NumberRangeRules<usize>) {
        self.into()
    }

    fn check(&self, messages: &mut ValidateErrorCollector, subject: Option<usize>) {
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
pub struct UnsignedError(pub ValidateErrorStore);

impl ValidationCheck for UnsignedError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Unsigned(usize, bool);

impl Default for Unsigned {
    fn default() -> Self {
        Self(0, true)
    }
}

impl Unsigned {
    pub fn parse_custom(s: Option<usize>, rules: UnsignedRules) -> Result<Self, UnsignedError> {
        let is_none = s.is_none();
        let mut messages = ValidateErrorCollector::new();
        rules.check(&mut messages, s);
        UnsignedError::validate_check(messages)?;
        Ok(Self(s.unwrap_or_default(), is_none))
    }

    pub fn parse(s: Option<usize>) -> Result<Self, UnsignedError> {
        Self::parse_custom(s, UnsignedRules::default())
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn into_option(self) -> Option<Unsigned> {
        if self.1 { None } else { Some(self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsigned() {
        let unsigned = Unsigned::parse(Some(10));
        assert!(unsigned.is_ok());
        let unsigned = Unsigned::parse(Some(1000));
        assert!(unsigned.is_err());
    }

    #[test]
    fn test_none_unsigned() {
        let unsigned = Unsigned::parse(None);
        assert!(unsigned.is_err());
    }
}
