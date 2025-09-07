use crate::base::date_time::data::AsDateTimeData;
use crate::base::date_time::rules::{DateTimeMandatoryRules, DateTimeRangeRules};
use crate::common::locale::{ValidateErrorCollector, ValidateErrorStore};
use crate::common::validation_check::ValidationCheck;
use chrono::{DateTime, NaiveDateTime, TimeDelta, TimeZone, Utc};
use std::ops::Add;
use thiserror::Error;

pub struct DateTimeRules {
    pub is_mandatory: bool,
    pub min: Option<DateTime<Utc>>,
    pub max: Option<DateTime<Utc>>,
}

impl Default for DateTimeRules {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            is_mandatory: true,
            min: Some(now.clone()),
            max: Some(now.clone().add(TimeDelta::days(30))),
        }
    }
}

impl Into<(DateTimeMandatoryRules, DateTimeRangeRules)> for &DateTimeRules {
    fn into(self) -> (DateTimeMandatoryRules, DateTimeRangeRules) {
        (
            DateTimeMandatoryRules {
                is_mandatory: self.is_mandatory,
            },
            DateTimeRangeRules {
                min: self.min.as_ref().map(|min| min.as_date_time_data()),
                max: self.max.as_ref().map(|max| max.as_date_time_data()),
            },
        )
    }
}

impl DateTimeRules {
    fn rules(&self) -> (DateTimeMandatoryRules, DateTimeRangeRules) {
        self.into()
    }

    fn check<Tz: TimeZone>(
        &self,
        messages: &mut ValidateErrorCollector,
        subject: Option<&DateTime<Tz>>,
    ) {
        if !self.is_mandatory && subject.is_none() {
            return;
        }
        let subject = subject.map(|s| s.as_date_time_data());
        let (mandatory_rule, range_rule) = self.rules();
        mandatory_rule.check(messages, subject.as_ref());
        if !messages.is_empty() {
            return;
        }
        range_rule.check(messages, subject.as_ref());
    }
}

#[derive(Debug, Error, PartialEq, Clone, Default)]
#[error("DateTime Validation Error")]
pub struct DateTimeError(pub ValidateErrorStore);

impl ValidationCheck for DateTimeError {
    fn validate_new(messages: ValidateErrorStore) -> Self {
        Self(messages)
    }
}

pub struct DateTimeValue<Tz: TimeZone>(Option<DateTime<Tz>>);

impl<Tz: TimeZone> DateTimeValue<Tz> {
    pub fn parse_custom(
        subject: Option<DateTime<Tz>>,
        rules: DateTimeRules,
    ) -> Result<Self, DateTimeError> {
        let mut messages = ValidateErrorCollector::new();
        rules.check(&mut messages, subject.as_ref());
        DateTimeError::validate_check(messages)?;
        Ok(Self(subject))
    }

    pub fn parse_custom_naive_with_tz(
        subject: Option<NaiveDateTime>,
        rules: DateTimeRules,
        tz: Tz,
    ) -> Result<Self, DateTimeError> {
        let subject = subject.map(|s| s.and_local_timezone(tz).unwrap());
        Self::parse_custom(subject, rules)
    }

    pub fn parse(subject: Option<DateTime<Tz>>) -> Result<Self, DateTimeError> {
        Self::parse_custom(subject, DateTimeRules::default())
    }

    pub fn parse_naive_with_tz(
        subject: Option<NaiveDateTime>,
        tz: Tz,
    ) -> Result<Self, DateTimeError> {
        Self::parse_custom_naive_with_tz(subject, DateTimeRules::default(), tz)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_custom() {
        let subject = Some(Utc::now().add(TimeDelta::days(1)));
        let rules = DateTimeRules::default();
        let result = DateTimeValue::parse_custom(subject, rules);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_default_err() {
        let result = DateTimeValue::<Utc>::parse(None);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_default_max_min_ok() {
        let subject = Some(Utc::now().add(TimeDelta::days(1)));
        let result = DateTimeValue::parse(subject);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_default_max_err() {
        let subject = Some(Utc::now().add(TimeDelta::days(31)));
        let result = DateTimeValue::parse(subject);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_default_min_err() {
        let subject = Some(Utc::now().add(TimeDelta::days(-1)));
        let result = DateTimeValue::parse(subject);
        assert!(result.is_err());
    }
}
