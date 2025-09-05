use crate::common::locale::{ValidateErrorCollector, ValidateErrorStore};

pub trait ValidationCheck: Sized {
    fn validate_new(messages: ValidateErrorStore) -> Self;

    fn validate_check(messages: ValidateErrorCollector) -> Result<(), Self> {
        if messages.is_empty() {
            Ok(())
        } else {
            Err(Self::validate_new(messages.into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::string_rules::StringMandatoryLocale;

    struct TestValidationCheck;

    impl ValidationCheck for TestValidationCheck {
        fn validate_new(_: ValidateErrorStore) -> Self {
            Self
        }
    }

    #[test]
    fn test_validate_check_is_err() {
        let mut messages = ValidateErrorCollector::new();
        messages.push(("error".to_string(), Box::new(StringMandatoryLocale)));
        assert!(TestValidationCheck::validate_check(messages).is_err());
    }

    #[test]
    fn test_validate_check_is_ok() {
        let messages = ValidateErrorCollector::new();
        assert!(TestValidationCheck::validate_check(messages).is_ok());
    }
}
