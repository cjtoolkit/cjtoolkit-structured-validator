use crate::common::locale::{LocaleData, LocaleMessage, LocaleValue, ValidateErrorCollector};
use num::{Num, NumCast};
use std::fmt::Display;

pub struct NumberMandatoryLocale;

impl LocaleMessage for NumberMandatoryLocale {
    fn get_locale_data(&self) -> LocaleData {
        LocaleData {
            name: "validate-cannot-be-empty".to_string(),
            args: Default::default(),
        }
    }
}

pub struct NumberMandatoryRules {
    pub is_mandatory: bool,
}

impl NumberMandatoryRules {
    pub fn check<T: Num>(&self, messages: &mut ValidateErrorCollector, subject: Option<T>) {
        if self.is_mandatory && subject.is_none() {
            messages.push((
                "Cannot be empty".to_string(),
                Box::new(NumberMandatoryLocale),
            ));
        }
    }
}

pub enum NumberRangeLocale {
    MinValue(f64),
    MaxValue(f64),
}

impl LocaleMessage for NumberRangeLocale {
    fn get_locale_data(&self) -> LocaleData {
        match self {
            Self::MinValue(min) => LocaleData {
                name: "validate-number-min-value".to_string(),
                args: vec![("min".to_string(), LocaleValue::Float(*min))]
                    .into_iter()
                    .collect(),
            },
            Self::MaxValue(max) => LocaleData {
                name: "validate-number-max-value".to_string(),
                args: vec![("max".to_string(), LocaleValue::Float(*max))]
                    .into_iter()
                    .collect(),
            },
        }
    }
}

pub struct NumberRangeRules<T>
where
    T: NumCast + Default + PartialOrd + Display,
{
    pub min: Option<T>,
    pub max: Option<T>,
}

impl<T: NumCast + Default + PartialOrd + Display> NumberRangeRules<T> {
    pub fn check(&self, messages: &mut ValidateErrorCollector, subject: Option<T>) {
        let is_some = subject.is_some();
        let subject = subject.unwrap_or_default();
        if let Some(min) = &self.min {
            if is_some && subject < *min {
                messages.push((
                    format!("Must be at least {}", min),
                    Box::new(NumberRangeLocale::MinValue(
                        min.to_f64().unwrap_or_default(),
                    )),
                ));
            }
        }
        if let Some(max) = &self.max {
            if is_some && subject > *max {
                messages.push((
                    format!("Must be at most {}", max),
                    Box::new(NumberRangeLocale::MaxValue(
                        max.to_f64().unwrap_or_default(),
                    )),
                ));
            }
        }
    }
}
