use crate::base::date_time::data::AsDateTimeData;
use crate::base::date_time::rules::{DateTimeMandatoryRules, DateTimeRangeRules};
use crate::common::locale::{ValidateErrorCollector, ValidateErrorStore};
use crate::common::validation_check::ValidationCheck;
use chrono::{NaiveDateTime, TimeDelta, Utc};
use std::ops::Add;
use thiserror::Error;

pub struct NaiveDateTimeRules {
    pub is_mandatory: bool,
    pub min: Option<NaiveDateTime>,
    pub max: Option<NaiveDateTime>,
}

impl Default for NaiveDateTimeRules {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            is_mandatory: true,
            min: Some(now.clone().naive_utc()),
            max: Some(now.clone().naive_utc().add(TimeDelta::days(30))),
        }
    }
}

impl NaiveDateTimeRules {
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
        subject: Option<&NaiveDateTime>,
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
#[error("NaiveDateTime Validation Error")]
pub struct NaiveDateTimeError(pub ValidateErrorStore);

impl ValidationCheck for NaiveDateTimeError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct NaiveDateTimeValue(Option<NaiveDateTime>);

impl NaiveDateTimeValue {
    pub fn parse_custom_with_format(
        subject: Option<NaiveDateTime>,
        rules: NaiveDateTimeRules,
        format: Option<&str>,
    ) -> Result<Self, NaiveDateTimeError> {
        let mut messages = ValidateErrorCollector::new();
        rules.check(subject.as_ref(), &mut messages, format);
        NaiveDateTimeError::validate_check(messages)?;
        Ok(Self(subject))
    }

    pub fn parse_custom(
        subject: Option<NaiveDateTime>,
        rules: NaiveDateTimeRules,
    ) -> Result<Self, NaiveDateTimeError> {
        Self::parse_custom_with_format(subject, rules, None)
    }

    pub fn parse(subject: Option<NaiveDateTime>) -> Result<Self, NaiveDateTimeError> {
        Self::parse_custom(subject, NaiveDateTimeRules::default())
    }

    pub fn parse_with_format(
        subject: Option<NaiveDateTime>,
        format: Option<&str>,
    ) -> Result<Self, NaiveDateTimeError> {
        Self::parse_custom_with_format(subject, NaiveDateTimeRules::default(), format)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_custom() {
        let subject = Some(Utc::now().naive_utc().add(TimeDelta::days(1)));
        let rules = NaiveDateTimeRules::default();
        let result = NaiveDateTimeValue::parse_custom(subject, rules);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_default_err() {
        let result = NaiveDateTimeValue::parse(None);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_default_max_min_ok() {
        let subject = Some(Utc::now().naive_utc().add(TimeDelta::days(1)));
        let result = NaiveDateTimeValue::parse(subject);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_default_max_err() {
        let subject = Some(Utc::now().naive_utc().add(TimeDelta::days(31)));
        let result = NaiveDateTimeValue::parse(subject);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_default_min_err() {
        let subject = Some(Utc::now().naive_utc().add(TimeDelta::days(-1)));
        let result = NaiveDateTimeValue::parse(subject);
        assert!(result.is_err());
    }
}
