use crate::base::number_rules::{NumberMandatoryRules, NumberRangeRules};
use crate::common::locale::{ValidateErrorCollector, ValidateErrorStore};
use crate::common::validation_check::ValidationCheck;

pub struct IntegerRules {
    pub is_mandatory: bool,
    pub min: Option<isize>,
    pub max: Option<isize>,
}

impl Default for IntegerRules {
    fn default() -> Self {
        Self {
            is_mandatory: true,
            min: Some(0),
            max: Some(255),
        }
    }
}

impl Into<(NumberMandatoryRules, NumberRangeRules<isize>)> for &IntegerRules {
    fn into(self) -> (NumberMandatoryRules, NumberRangeRules<isize>) {
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

impl IntegerRules {
    fn rules(&self) -> (NumberMandatoryRules, NumberRangeRules<isize>) {
        self.into()
    }

    fn check(&self, messages: &mut ValidateErrorCollector, subject: Option<isize>) {
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
pub struct IntegerError(pub ValidateErrorStore);

impl ValidationCheck for IntegerError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Integer(isize, bool);

impl Integer {
    pub fn parse_custom(s: Option<isize>, rules: IntegerRules) -> Result<Self, IntegerError> {
        let is_none = s.is_none();
        let mut messages = ValidateErrorCollector::new();
        rules.check(&mut messages, s);
        IntegerError::validate_check(messages)?;
        Ok(Self(s.unwrap_or_default(), is_none))
    }

    pub fn parse(s: Option<isize>) -> Result<Self, IntegerError> {
        Self::parse_custom(s, IntegerRules::default())
    }

    pub fn as_isize(&self) -> isize {
        self.0
    }

    pub fn into_option(self) -> Option<Integer> {
        if self.1 { None } else { Some(self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer() {
        let integer = Integer::parse(Some(10));
        assert!(integer.is_ok());
        let integer = Integer::parse(Some(1000));
        assert!(integer.is_err());
        let integer = Integer::parse(Some(-50));
        assert!(integer.is_err());
    }

    #[test]
    fn test_none_integer() {
        let integer = Integer::parse(None);
        assert!(integer.is_err());
    }
}
