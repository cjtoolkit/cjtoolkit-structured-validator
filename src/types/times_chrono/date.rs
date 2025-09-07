use crate::base::date_time::data::AsDateTimeData;
use crate::base::date_time::rules::{DateTimeMandatoryRules, DateTimeRangeRules};
use crate::common::locale::{ValidateErrorCollector, ValidateErrorStore};
use crate::common::validation_check::ValidationCheck;
use chrono::{NaiveDate, TimeDelta, Utc};
use std::ops::Add;
use thiserror::Error;

pub struct DateRules {
    pub is_mandatory: bool,
    pub min: Option<NaiveDate>,
    pub max: Option<NaiveDate>,
}

impl Default for DateRules {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            is_mandatory: true,
            min: Some(now.clone().date_naive()),
            max: Some(now.clone().add(TimeDelta::days(30)).date_naive()),
        }
    }
}

impl DateRules {
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
                    .min
                    .as_ref()
                    .map(|max| (date_format.clone(), max).as_date_time_data()),
            },
        )
    }

    fn check(
        self,
        subject: Option<&NaiveDate>,
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
#[error("Date Validation Error")]
pub struct DateError(pub ValidateErrorStore);

impl ValidationCheck for DateError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct DateValue(Option<NaiveDate>);

impl DateValue {
    pub fn parse_custom_with_format(
        subject: Option<NaiveDate>,
        rules: DateRules,
        format: Option<&str>,
    ) -> Result<Self, DateError> {
        let mut messages = ValidateErrorCollector::new();
        rules.check(subject.as_ref(), &mut messages, format);
        DateError::validate_check(messages)?;
        Ok(Self(subject))
    }

    pub fn parse_custom(subject: Option<NaiveDate>, rules: DateRules) -> Result<Self, DateError> {
        Self::parse_custom_with_format(subject, rules, None)
    }

    pub fn parse(subject: Option<NaiveDate>) -> Result<Self, DateError> {
        Self::parse_custom(subject, DateRules::default())
    }

    pub fn parse_with_format(
        subject: Option<NaiveDate>,
        format: Option<&str>,
    ) -> Result<Self, DateError> {
        Self::parse_custom_with_format(subject, DateRules::default(), format)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_custom() {
        let subject = Some(Utc::now().date_naive());
        let rules = DateRules::default();
        let result = DateValue::parse_custom(subject, rules);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_default_err() {
        let result = DateValue::parse(None);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_default_max_min_ok() {
        let subject = Some(Utc::now().date_naive());
        let result = DateValue::parse(subject);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_default_max_err() {
        let subject = Some(Utc::now().date_naive().add(TimeDelta::days(31)));
        let result = DateValue::parse(subject);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_default_min_err() {
        let subject = Some(Utc::now().date_naive().add(TimeDelta::days(-1)));
        let result = DateValue::parse(subject);
        assert!(result.is_err());
    }
}
