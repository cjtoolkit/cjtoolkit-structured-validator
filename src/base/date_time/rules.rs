use crate::base::date_time::data::{DateTimeData, DateTimeKind};
use crate::common::locale::{LocaleData, LocaleMessage, LocaleValue, ValidateErrorCollector};

pub struct DateTimeMandatoryLocale;

impl LocaleMessage for DateTimeMandatoryLocale {
    fn get_locale_data(&self) -> LocaleData {
        LocaleData::new("validate-cannot-be-empty")
    }
}

pub struct DateTimeMandatoryRules {
    pub is_mandatory: bool,
}

impl DateTimeMandatoryRules {
    pub fn check(&self, messages: &mut ValidateErrorCollector, subject: Option<&DateTimeData>) {
        if self.is_mandatory && subject.is_none() {
            messages.push((
                "Cannot be empty".to_string(),
                Box::new(DateTimeMandatoryLocale),
            ));
        }
    }
}

pub enum DateTimeRangeLocale {
    MinValue(DateTimeData),
    MaxValue(DateTimeData),
}

impl LocaleMessage for DateTimeRangeLocale {
    fn get_locale_data(&self) -> LocaleData {
        use LocaleData as ld;
        use LocaleValue as lv;
        match self {
            DateTimeRangeLocale::MinValue(min) => match min.kind {
                DateTimeKind::Date => ld::new_with_vec(
                    "validate-date-min",
                    vec![("min".to_string(), lv::from(min.date_formatted.clone()))],
                ),
                DateTimeKind::DateTime => ld::new_with_vec(
                    "validate-date-time-min",
                    vec![("min".to_string(), lv::from(min.date_formatted.clone()))],
                ),
                DateTimeKind::DateTimeNaive => ld::new_with_vec(
                    "validate-date-time-naive-min",
                    vec![("min".to_string(), lv::from(min.date_formatted.clone()))],
                ),
                DateTimeKind::Time => ld::new_with_vec(
                    "validate-time-min",
                    vec![("min".to_string(), lv::from(min.date_formatted.clone()))],
                ),
            },
            DateTimeRangeLocale::MaxValue(max) => match max.kind {
                DateTimeKind::Date => ld::new_with_vec(
                    "validate-date-max",
                    vec![("max".to_string(), lv::from(max.date_formatted.clone()))],
                ),
                DateTimeKind::DateTime => ld::new_with_vec(
                    "validate-date-time-max",
                    vec![("max".to_string(), lv::from(max.date_formatted.clone()))],
                ),
                DateTimeKind::DateTimeNaive => ld::new_with_vec(
                    "validate-date-time-naive-max",
                    vec![("max".to_string(), lv::from(max.date_formatted.clone()))],
                ),
                DateTimeKind::Time => ld::new_with_vec(
                    "validate-time-max",
                    vec![("max".to_string(), lv::from(max.date_formatted.clone()))],
                ),
            },
        }
    }
}

pub struct DateTimeRangeRules {
    pub min: Option<DateTimeData>,
    pub max: Option<DateTimeData>,
}

impl DateTimeRangeRules {
    pub fn check(&self, messages: &mut ValidateErrorCollector, subject: Option<&DateTimeData>) {
        let default = DateTimeData::default();
        let is_some = subject.is_some();
        let subject = subject.unwrap_or(&default);
        if let Some(min) = &self.min {
            if is_some && subject < min {
                messages.push((
                    format!("Must be after '{}'", &subject.date_formatted),
                    Box::new(DateTimeRangeLocale::MinValue(min.clone())),
                ))
            }
        }
        if let Some(max) = &self.max {
            if is_some && subject > max {
                messages.push((
                    format!("Must be before '{}'", &subject.date_formatted),
                    Box::new(DateTimeRangeLocale::MaxValue(max.clone())),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod date_time_mandatory_rule {
        use super::*;
        use crate::base::date_time::data::DateTimeKind;

        #[test]
        fn test_empty_value() {
            let mut messages = ValidateErrorCollector::new();
            let subject: Option<DateTimeData> = None;
            let rules = DateTimeMandatoryRules { is_mandatory: true };
            rules.check(&mut messages, subject.as_ref());
            assert_eq!(messages.len(), 1);
            assert_eq!(messages.0[0].0, "Cannot be empty");
        }

        #[test]
        fn test_not_empty_value() {
            let mut messages = ValidateErrorCollector::new();
            let subject: Option<DateTimeData> = Some(DateTimeData {
                kind: DateTimeKind::DateTime,
                date_formatted: "".to_string(),
                timestamp_seconds_days: 1,
                subsec_nano: 1,
            });
            let rules = DateTimeMandatoryRules { is_mandatory: true };
            rules.check(&mut messages, subject.as_ref());
            assert_eq!(messages.len(), 0);
        }
    }

    mod date_time_range_rule {
        use super::*;
        use crate::base::date_time::data::DateTimeKind;

        #[test]
        fn test_min() {
            let mut messages = ValidateErrorCollector::new();
            let subject: Option<DateTimeData> = Some(DateTimeData {
                kind: DateTimeKind::DateTime,
                date_formatted: "".to_string(),
                timestamp_seconds_days: 1,
                subsec_nano: 1,
            });
            let rules = DateTimeRangeRules {
                min: Some(DateTimeData {
                    kind: DateTimeKind::DateTime,
                    date_formatted: "".to_string(),
                    timestamp_seconds_days: 1,
                    subsec_nano: 2,
                }),
                max: None,
            };
            rules.check(&mut messages, subject.as_ref());
            assert_eq!(messages.len(), 1);
            assert_eq!(messages.0[0].0, "Must be after ''");

            let mut messages = ValidateErrorCollector::new();
            let subject: Option<DateTimeData> = Some(DateTimeData {
                kind: DateTimeKind::DateTime,
                date_formatted: "".to_string(),
                timestamp_seconds_days: 1,
                subsec_nano: 3,
            });
            let rules = DateTimeRangeRules {
                min: Some(DateTimeData {
                    kind: DateTimeKind::DateTime,
                    date_formatted: "".to_string(),
                    timestamp_seconds_days: 1,
                    subsec_nano: 2,
                }),
                max: None,
            };
            rules.check(&mut messages, subject.as_ref());
            assert_eq!(messages.len(), 0);

            let mut messages = ValidateErrorCollector::new();
            let subject: Option<DateTimeData> = Some(DateTimeData {
                kind: DateTimeKind::DateTime,
                date_formatted: "".to_string(),
                timestamp_seconds_days: 1,
                subsec_nano: 1,
            });
            let rules = DateTimeRangeRules {
                min: Some(DateTimeData {
                    kind: DateTimeKind::DateTime,
                    date_formatted: "".to_string(),
                    timestamp_seconds_days: 2,
                    subsec_nano: 1,
                }),
                max: None,
            };
            rules.check(&mut messages, subject.as_ref());
            assert_eq!(messages.len(), 1);
            assert_eq!(messages.0[0].0, "Must be after ''");

            let mut messages = ValidateErrorCollector::new();
            let subject: Option<DateTimeData> = Some(DateTimeData {
                kind: DateTimeKind::DateTime,
                date_formatted: "".to_string(),
                timestamp_seconds_days: 3,
                subsec_nano: 1,
            });
            let rules = DateTimeRangeRules {
                min: Some(DateTimeData {
                    kind: DateTimeKind::DateTime,
                    date_formatted: "".to_string(),
                    timestamp_seconds_days: 2,
                    subsec_nano: 1,
                }),
                max: None,
            };
            rules.check(&mut messages, subject.as_ref());
            assert_eq!(messages.len(), 0);
        }
    }
}
