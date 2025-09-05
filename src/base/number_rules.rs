use crate::common::locale::{LocaleData, LocaleMessage, LocaleValue, ValidateErrorCollector};
use std::fmt::Display;

pub struct NumberMandatoryLocale;

impl LocaleMessage for NumberMandatoryLocale {
    fn get_locale_data(&self) -> LocaleData {
        LocaleData::new("validate-cannot-be-empty")
    }
}

pub struct NumberMandatoryRules {
    pub is_mandatory: bool,
}

impl NumberMandatoryRules {
    pub fn check<T: Into<LocaleValue>>(
        &self,
        messages: &mut ValidateErrorCollector,
        subject: Option<T>,
    ) {
        if self.is_mandatory && subject.is_none() {
            messages.push((
                "Cannot be empty".to_string(),
                Box::new(NumberMandatoryLocale),
            ));
        }
    }
}

pub enum NumberRangeLocale<T: Into<LocaleValue> + Send + Sync + Clone> {
    MinValue(T),
    MaxValue(T),
}

impl<T: Into<LocaleValue> + Send + Sync + Clone> LocaleMessage for NumberRangeLocale<T>
where
    LocaleValue: From<T>,
{
    fn get_locale_data(&self) -> LocaleData {
        use LocaleData as ld;
        use LocaleValue as lv;
        match self {
            Self::MinValue(min) => ld::new_with_vec(
                "validate-number-min-value",
                vec![("min".to_string(), lv::from(min.clone()))],
            ),
            Self::MaxValue(max) => ld::new_with_vec(
                "validate-number-max-value",
                vec![("max".to_string(), lv::from(max.clone()))],
            ),
        }
    }
}

pub struct NumberRangeRules<T>
where
    T: Clone + Into<LocaleValue> + Default + PartialOrd + Display,
{
    pub min: Option<T>,
    pub max: Option<T>,
}

impl<T> NumberRangeRules<T>
where
    T: Clone + Into<LocaleValue> + Default + PartialOrd + Display,
{
    pub fn check(&self, messages: &mut ValidateErrorCollector, subject: Option<T>) {
        let is_some = subject.is_some();
        let subject = subject.unwrap_or_default();
        if let Some(min) = &self.min {
            if is_some && subject < *min {
                messages.push((
                    format!("Must be at least {}", min),
                    Box::new(NumberRangeLocale::MinValue(min.clone().into())),
                ));
            }
        }
        if let Some(max) = &self.max {
            if is_some && subject > *max {
                messages.push((
                    format!("Must be at most {}", max),
                    Box::new(NumberRangeLocale::MaxValue(max.clone().into())),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod number_mandatory_rule {
        use super::*;

        #[test]
        fn test_empty_value() {
            let mut messages = ValidateErrorCollector::new();
            let subject: Option<f64> = None;
            let rules = NumberMandatoryRules { is_mandatory: true };
            rules.check(&mut messages, subject);
            assert_eq!(messages.len(), 1);
            assert_eq!(messages.0[0].0, "Cannot be empty");
        }

        #[test]
        fn test_not_empty_value() {
            let mut messages = ValidateErrorCollector::new();
            let subject: Option<f64> = Some(1.0);
            let rules = NumberMandatoryRules { is_mandatory: true };
            rules.check(&mut messages, subject);
            assert_eq!(messages.len(), 0);
        }
    }

    mod number_range_rule {
        use super::*;

        #[test]
        fn test_invalid_min_value_rule() {
            let mut messages = ValidateErrorCollector::new();
            let subject: Option<f64> = Some(1.0);
            let rules = NumberRangeRules {
                min: Some(2.0),
                max: None,
            };
            rules.check(&mut messages, subject);
            assert_eq!(messages.len(), 1);
            assert_eq!(messages.0[0].0, "Must be at least 2");
        }

        #[test]
        fn test_valid_min_value_rule() {
            let mut messages = ValidateErrorCollector::new();
            let subject: Option<f64> = Some(2.0);
            let rules = NumberRangeRules {
                min: Some(2.0),
                max: None,
            };
            rules.check(&mut messages, subject);
            assert_eq!(messages.len(), 0);
        }

        #[test]
        fn test_max_value_rule() {
            let mut messages = ValidateErrorCollector::new();
            let subject: Option<f64> = Some(1.0);
            let rules = NumberRangeRules {
                min: None,
                max: Some(2.0),
            };
            rules.check(&mut messages, subject);
            assert_eq!(messages.len(), 0);
        }

        #[test]
        fn test_valid_max_value_rule() {
            let mut messages = ValidateErrorCollector::new();
            let subject: Option<f64> = Some(2.1);
            let rules = NumberRangeRules {
                min: None,
                max: Some(2.0),
            };
            rules.check(&mut messages, subject);
            assert_eq!(messages.len(), 1);
            assert_eq!(messages.0[0].0, "Must be at most 2");
        }
    }
}
