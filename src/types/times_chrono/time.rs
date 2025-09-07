use crate::base::date_time::data::AsDateTimeData;
use crate::base::date_time::rules::{DateTimeMandatoryRules, DateTimeRangeRules};
use crate::common::locale::{ValidateErrorCollector, ValidateErrorStore};
use crate::common::validation_check::ValidationCheck;
use chrono::NaiveTime;
use thiserror::Error;

pub struct TimeRules {
    pub is_mandatory: bool,
    pub min: Option<NaiveTime>,
    pub max: Option<NaiveTime>,
}

impl Default for TimeRules {
    fn default() -> Self {
        Self {
            is_mandatory: true,
            min: Some(NaiveTime::from_hms_opt(9, 0, 0).unwrap_or_default()),
            max: Some(NaiveTime::from_hms_opt(17, 0, 0).unwrap_or_default()),
        }
    }
}

impl TimeRules {
    fn rules(&self, date_format: Option<&str>) -> (DateTimeMandatoryRules, DateTimeRangeRules) {
        (
            DateTimeMandatoryRules {
                is_mandatory: self.is_mandatory,
            },
            DateTimeRangeRules {
                min: self
                    .min
                    .as_ref()
                    .map(|min| (date_format.clone(), min).as_date_time_data()),
                max: self
                    .max
                    .as_ref()
                    .map(|max| (date_format.clone(), max).as_date_time_data()),
            },
        )
    }

    fn check(
        self,
        subject: Option<&NaiveTime>,
        messages: &mut ValidateErrorCollector,
        date_format: Option<&str>,
    ) {
        if !self.is_mandatory && subject.is_none() {
            return;
        }
        let subject = subject.map(|s| (date_format.clone(), s).as_date_time_data());
        let (mandatory_rule, range_rule) = self.rules(date_format);
        mandatory_rule.check(messages, subject.as_ref());
        if !messages.is_empty() {
            return;
        }
        range_rule.check(messages, subject.as_ref());
    }
}

#[derive(Debug, Error, PartialEq, Clone, Default)]
#[error("Time Validation Error")]
pub struct TimeError(pub ValidateErrorStore);

impl ValidationCheck for TimeError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TimeValue(Option<NaiveTime>);

impl TimeValue {
    pub fn parse_custom_with_format(
        subject: Option<NaiveTime>,
        rules: TimeRules,
        format: Option<&str>,
    ) -> Result<Self, TimeError> {
        let mut messages = ValidateErrorCollector::new();
        rules.check(subject.as_ref(), &mut messages, format);
        TimeError::validate_check(messages)?;
        Ok(Self(subject))
    }

    pub fn parse_custom(subject: Option<NaiveTime>, rules: TimeRules) -> Result<Self, TimeError> {
        Self::parse_custom_with_format(subject, rules, None)
    }

    pub fn parse(subject: Option<NaiveTime>) -> Result<Self, TimeError> {
        Self::parse_custom(subject, TimeRules::default())
    }

    pub fn parse_with_format(
        subject: Option<NaiveTime>,
        format: Option<&str>,
    ) -> Result<Self, TimeError> {
        Self::parse_custom_with_format(subject, TimeRules::default(), format)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_custom() {
        let subject = NaiveTime::from_hms_opt(10, 0, 0);
        let rules = TimeRules::default();
        let result = TimeValue::parse_custom(subject, rules);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_default_err() {
        let result = TimeValue::parse(None);
        assert!(result.is_err());
    }
    #[test]
    fn test_parse_default_max_min_ok() {
        let subject = NaiveTime::from_hms_opt(10, 0, 0);
        let result = TimeValue::parse(subject);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_default_max_err() {
        let subject = NaiveTime::from_hms_opt(18, 0, 0);
        let result = TimeValue::parse(subject);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_default_min_err() {
        let subject = NaiveTime::from_hms_opt(8, 0, 0);
        let result = TimeValue::parse(subject);
        assert!(result.is_err());
    }
}
