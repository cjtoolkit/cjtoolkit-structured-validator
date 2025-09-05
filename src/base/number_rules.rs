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
